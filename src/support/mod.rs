#![allow(dead_code)]

use glium::{
    glutin::{event, event_loop},
    Display,
};

use fnv::FnvHashMap;

use crate::engine;

pub enum Request<'a, 'b: 'a> {
    Event {
        event: &'a event::Event<'b, ()>,
        should_update_ui: &'a mut bool,
        should_exit: &'a mut bool,
    },
    SetUi {
        needs_redraw: &'a mut bool,
    },
    Redraw,
}

/// In most of the examples the `glutin` crate is used for providing the window context and
/// events while the `glium` crate is used for displaying `conrod_core::render::Primitives` to the
/// screen.
///
/// This function simplifies some of the boilerplate involved in limiting the redraw rate in the
/// glutin+glium event loop.
pub fn run_loop<F>(display: Display, event_loop: event_loop::EventLoop<()>, mut callback: F) -> !
where
    F: 'static + FnMut(Request, &Display),
{
    let sixteen_ms = std::time::Duration::from_millis(16);
    let mut next_update = None;
    let mut ui_update_needed = false;
    event_loop.run(move |event, _, control_flow| {
        {
            let mut should_update_ui = false;
            let mut should_exit = false;
            callback(
                Request::Event {
                    event: &event,
                    should_update_ui: &mut should_update_ui,
                    should_exit: &mut should_exit,
                },
                &display,
            );
            ui_update_needed |= should_update_ui;
            if should_exit {
                *control_flow = event_loop::ControlFlow::Exit;
                return;
            }
        }

        // We don't want to draw any faster than 60 FPS, so set the UI only on every 16ms, unless:
        // - this is the very first event, or
        // - we didn't request update on the last event and new events have arrived since then.
        let should_set_ui_on_main_events_cleared = next_update.is_none() && ui_update_needed;
        match (&event, should_set_ui_on_main_events_cleared) {
            (event::Event::NewEvents(event::StartCause::Init { .. }), _)
            | (event::Event::NewEvents(event::StartCause::ResumeTimeReached { .. }), _)
            | (event::Event::MainEventsCleared, true) => {
                next_update = Some(std::time::Instant::now() + sixteen_ms);
                ui_update_needed = false;

                let mut needs_redraw = false;
                callback(
                    Request::SetUi {
                        needs_redraw: &mut needs_redraw,
                    },
                    &display,
                );
                if needs_redraw {
                    display.gl_window().window().request_redraw();
                } else {
                    // We don't need to redraw anymore until more events arrives.
                    next_update = None;
                }
            }
            _ => {}
        }
        if let Some(next_update) = next_update {
            *control_flow = event_loop::ControlFlow::WaitUntil(next_update);
        } else {
            *control_flow = event_loop::ControlFlow::Wait;
        }

        // Request redraw if needed.
        match &event {
            event::Event::RedrawRequested(_) => {
                callback(Request::Redraw, &display);
            }
            _ => {}
        }
    })
}

// Conversion functions for converting between types from glium's version of `winit` and
// `conrod_core`.
conrod_winit::v023_conversion_fns!();

widget_ids! {
    pub struct Ids {
        canvas,
        title,
        board,
        current_player,
        undo_button,
        redo_button,
        white_score,
        black_score
    }
}

pub struct App {
    pub board_state: [[Player; 8]; 8],
    pub player: Player,
    pub engine: engine::board::Board,
    pub previous_states: Vec<engine::board::Board>,
    pub turn: usize,
    pub winner: Player,
    pub transposition_table: fnv::FnvHashMap<(u64, u64), (i16, u8)>
}

impl App {
    /// Simple constructor for the `DemoApp`.
    pub fn new() -> Self {
        App {
            board_state: [
            [Player::None, Player::None, Player::None, Player::None, Player::None, Player::None, Player::None, Player::None],
            [Player::None, Player::None, Player::None, Player::None, Player::None, Player::None, Player::None, Player::None],
            [Player::None, Player::None, Player::None, Player::Valid, Player::None, Player::None, Player::None, Player::None],
            [Player::None, Player::None, Player::Valid, Player::White, Player::Black, Player::None, Player::None, Player::None],
            [Player::None, Player::None, Player::None, Player::Black, Player::White, Player::Valid, Player::None, Player::None],
            [Player::None, Player::None, Player::None, Player::None, Player::Valid, Player::None, Player::None, Player::None],
            [Player::None, Player::None, Player::None, Player::None, Player::None, Player::None, Player::None, Player::None],
            [Player::None, Player::None, Player::None, Player::None, Player::None, Player::None, Player::None, Player::None]
            ],
            player: Player::Black, // default player
            engine: engine::board::Board::new(),
            previous_states: Vec::new(),
            turn: 0,
            winner: Player::None,
            transposition_table: fnv::FnvHashMap::default()
        }
    }

    pub fn place_tile(&mut self, cell: [u8; 2]) {
        self.previous_states.truncate(self.turn + 1);
        self.turn += 1;
        self.previous_states.push(self.engine.clone());
        self.engine = self.engine.place_tile(engine::board::get_bitmask_for_index(engine::board::get_index_from_move(engine::board::Move{col: cell[1] as i8, row: cell[0] as i8})));
        self.reinitialize_board();
        self.engine = self.engine.place_tile(crate::engine::AI::AI::get_minimax_move(self.engine, &mut self.transposition_table));
        self.reinitialize_board();
    }

    pub fn undo(&mut self) {
        if self.turn != 0 {
            self.turn -= 1;
            self.engine = self.previous_states[self.turn];
            self.reinitialize_board();
        }
    }

    // pub fn redo(&mut self) {
    //     if self.turn != self.previous_states.len() {
    //         self.turn += 1;
    //         self.engine = self.previous_states[self.turn];
    //         self.reinitialize_board();
    //     }
    // }

    pub fn get_white_score(&self) -> u32 {
        self.engine.white_bitboard.count_ones()
    }

    pub fn get_black_score(&self) -> u32 {
        self.engine.black_bitboard.count_ones()
    }

    fn reinitialize_board(&mut self) {
        let new_board = self.engine.get_board();
        self.player = match self.engine.turn {
            engine::enums::Player::Black => Player::Black,
            engine::enums::Player::White => Player::White
        };
        for row in 0..8 {
            for col in 0..8 {
                match new_board[row][col] {
                    engine::enums::Position::White => {self.board_state[row][col] = Player::White},
                    engine::enums::Position::Black => {self.board_state[row][col] = Player::Black},
                    engine::enums::Position::Valid => {self.board_state[row][col] = Player::Valid},
                    engine::enums::Position::Empty => {self.board_state[row][col] = Player::None},
                }
            }
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum Player {
    Black,
    White,
    Valid,
    None
}