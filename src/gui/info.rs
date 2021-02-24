// use conrod_core::{
//     self, widget, widget_ids, Colorable, Labelable, Point, Positionable, Widget, Borderable, {position::Relative}
// };
// use std::iter::once;

// use crate::support;
// use crate::gui::tile;
// use crate::gui::board;

// #[derive(WidgetCommon)]
// pub struct Info<'a> {
//     #[conrod(common_builder)]
//     common: widget::CommonBuilder,
//     style: Style,
//     white_score: u32,
//     black_score: u32,
//     undo: dyn Fn()
// }

// #[derive(Copy, Clone, Debug, Default, PartialEq, WidgetStyle)]
// pub struct Style {
//     /// Color of the button.
//     #[conrod(default = "conrod_core::color::WHITE")]
//     pub color: Option<conrod_core::Color>,
// }

// widget_ids! {
//     struct Ids {
//        white_score,
//        black_score,
//        undo_button,
//     }
// }

// pub struct State {
//     ids: Ids,
// }

// impl<'a> Info<'a> {
//     pub fn new(undo: dyn Fn(), white_score: u32, black_score: u32) -> Self {
//         Info {
//             undo,
//             white_score,
//             black_score,
//             common: widget::CommonBuilder::default(),
//             style: Style::default(),

//         }
//     }
// }

// #[derive(Debug)]
// enum EventType {
//     Click
// }

// #[derive(Debug)]
// pub struct Event {
//     eventType: EventType, 
//     pub cell: [u8; 2]
// }
// impl<'a> Widget for Board<'a> {
//     type State = State;
//     type Style = Style;
//     type Event = Option<Event>;

//     fn init_state(&self, id_gen: widget::id::Generator) -> Self::State {
//         State {
//             ids: Ids::new(id_gen),
//         }
//     }

//     fn style(&self) -> Self::Style {
//         self.style.clone()
//     }

//     /// Update the state of the button by handling any input that has occurred since the last
//     /// update.
//     fn update(self, args: widget::UpdateArgs<Self>) -> Self::Event {
//         let widget::UpdateArgs {
//             id,
//             state,
//             rect,
//             ui,
//             style,
//             ..
//         } = args;

//         // let event = ui.widget_input(id).clicks().left().next().map(|_| ());

//         // Background
//         let radius = 10.0;
//         widget::RoundedRectangle::fill([800.0, 800.0], radius)
//             .color(conrod_core::color::CHARCOAL)
//             .set(state.ids.background, ui);

//         // Columns
//         let col_ids = [state.ids.col0, state.ids.col1, state.ids.col2, state.ids.col3, state.ids.col4, state.ids.col5, state.ids.col6, state.ids.col7, state.ids.col8];
//         for i in 0..9 {
//             widget::Line::centred([0.0, 0.0], [0.0, -800.0])
//                 .color(conrod_core::color::WHITE)
//                 .x_y_position_relative_to(state.ids.background, Relative::Scalar(-400.0 + (100.0 * i as f64)), Relative::Scalar(0.0))
//                 .set(col_ids[i], ui);    
//         }
        
//         // Rows
//         let row_ids = [state.ids.row0, state.ids.row1, state.ids.row2, state.ids.row3, state.ids.row4, state.ids.row5, state.ids.row6, state.ids.row7, state.ids.row8];
//         for i in 0..9 {
//             widget::Line::centred([0.0, 0.0], [-800.0, 0.0])
//             .color(conrod_core::color::WHITE)
//             .x_y_position_relative_to(state.ids.background, Relative::Scalar(0.0), Relative::Scalar(400.0 - (100.0 * i as f64)))
//             .set(row_ids[i], ui);    
//         }
        
//         // Tiles
//         let mut event: Option<(conrod_core::event::Click, [u8; 2])> = None;
//         let tile_ids = [
//             [state.ids.tile00, state.ids.tile01, state.ids.tile02, state.ids.tile03, state.ids.tile04, state.ids.tile05, state.ids.tile06, state.ids.tile07],
//             [state.ids.tile10, state.ids.tile11, state.ids.tile12, state.ids.tile13, state.ids.tile14, state.ids.tile15, state.ids.tile16, state.ids.tile17],
//             [state.ids.tile20, state.ids.tile21, state.ids.tile22, state.ids.tile23, state.ids.tile24, state.ids.tile25, state.ids.tile26, state.ids.tile27],
//             [state.ids.tile30, state.ids.tile31, state.ids.tile32, state.ids.tile33, state.ids.tile34, state.ids.tile35, state.ids.tile36, state.ids.tile37],
//             [state.ids.tile40, state.ids.tile41, state.ids.tile42, state.ids.tile43, state.ids.tile44, state.ids.tile45, state.ids.tile46, state.ids.tile47],
//             [state.ids.tile50, state.ids.tile51, state.ids.tile52, state.ids.tile53, state.ids.tile54, state.ids.tile55, state.ids.tile56, state.ids.tile57],
//             [state.ids.tile60, state.ids.tile61, state.ids.tile62, state.ids.tile63, state.ids.tile64, state.ids.tile65, state.ids.tile66, state.ids.tile67],
//             [state.ids.tile70, state.ids.tile71, state.ids.tile72, state.ids.tile73, state.ids.tile74, state.ids.tile75, state.ids.tile76, state.ids.tile77]
//             ];
//         for i in 0..8 {
//             for j in 0..8 {
//                 for e in tile::Tile::new([i as u8, j as u8]).owner(self.board_state[i][j]).x_y_position_relative_to(state.ids.background, Relative::Scalar(-350.0 + (100.0 * j as f64)), Relative::Scalar(350.0 - (100.0 * i as f64))).set(tile_ids[i][j], ui) {
//                     event = Some(e);
//                 }
//             }
//         }

//         match event {
//             Some(x) => Some(Event{eventType: EventType::Click, cell: x.1}),
//             None => None
//         }
//     }
// }