use crate::support;
use conrod_core::{widget, Colorable, Labelable, Positionable, Sizeable, Widget};
use std::iter::once;

mod board;
mod tile;
// mod info;

pub fn gui(ui: &mut conrod_core::UiCell, ids: &support::Ids, app: &mut support::App) {
    const MARGIN: conrod_core::Scalar = 30.0;
    const TITLE_SIZE: conrod_core::FontSize = 42;
    const TITLE: &'static str = "Othello-RS";
    
    let current_player = match app.player {
        support::Player::White => "White to Play",
        support::Player::Black => "Black to Play",
        _ => "Unknown player"
    };
    
    widget::Canvas::new()
        .pad(MARGIN)
        .set(ids.canvas, ui);

    widget::Text::new(TITLE)
        .font_size(TITLE_SIZE)
        .top_left_of(ids.canvas)
        // .mid_top_of(ids.canvas)
        .set(ids.title, ui);
    
    widget::Text::new(current_player)
        .font_size(TITLE_SIZE)
        .mid_top_of(ids.canvas)
        .set(ids.current_player, ui);

    widget::Text::new(&app.get_white_score().to_string())
        .font_size(TITLE_SIZE)
        .color(conrod_core::color::WHITE)
        .mid_top_of(ids.canvas)
        .set(ids.white_score, ui);
    
    widget::Text::new(&app.get_black_score().to_string())
        .font_size(TITLE_SIZE)
        .color(conrod_core::color::BLACK)
        .mid_top_of(ids.canvas)
        .set(ids.black_score, ui);

    for event in board::Board::new(&app.board_state)
        .top_left_of(ids.canvas)
        .set(ids.board, ui) {
        app.place_tile(event.cell);
    }
    
    for _press in widget::Button::new().label("Undo").mid_right_of(ids.canvas)
    .set(ids.undo_button, ui) {
        app.undo();
    }

    // for _press in widget::Button::new().label("Redo").set(ids.redo_button, ui) {
    //     app.redo();
    // }
    
}