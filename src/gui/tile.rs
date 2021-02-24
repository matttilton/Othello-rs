use conrod_core::{
    self, widget, widget_ids, Colorable, Labelable, Point, Positionable, Widget,
};

use crate::support;

#[derive(WidgetCommon)]
pub struct Tile {
    #[conrod(common_builder)]
    common: widget::CommonBuilder,
    style: Style,
    enabled: bool,
    owner: support::Player,
    cell: [u8; 2]
}

#[derive(Copy, Clone, Debug, Default, PartialEq, WidgetStyle)]
pub struct Style {
    /// Color of the button.
    #[conrod(default = "conrod_core::color::WHITE")]
    pub color: Option<conrod_core::Color>,
}

widget_ids! {
    struct Ids {
        circle,
    }
}

pub struct State {
    ids: Ids,
}

impl<'a> Tile {
    pub fn new(cell: [u8; 2]) -> Self {
        Tile {
            common: widget::CommonBuilder::default(),
            style: Style::default(),
            enabled: false,
            owner: support::Player::None,
            cell
        }
    }

    pub fn owner(mut self, new_owner: support::Player) -> Self {
        self.owner = new_owner;
        self
    }

    pub fn enabled(mut self, flag: bool) -> Self {
        self.enabled = flag;
        self
    }
}

impl<'a> Widget for Tile {
    type State = State;
    type Style = Style;
    type Event = Option<(conrod_core::event::Click, [u8; 2])>;

    fn init_state(&self, id_gen: widget::id::Generator) -> Self::State {
        State {
            ids: Ids::new(id_gen),
        }
    }

    fn style(&self) -> Self::Style {
        self.style.clone()
    }

    /// Update the state of the button by handling any input that has occurred since the last
    /// update.
    fn update(self, args: widget::UpdateArgs<Self>) -> Self::Event {
        let widget::UpdateArgs {
            id,
            state,
            rect,
            ui,
            style,
            ..
        } = args;

        // If the button was clicked, produce `Some` event.
        let mut event = None;

        if self.owner == support::Player::Valid {
            let input = ui.widget_input(id);
            event = input.clicks().left().next().map(|x| {
                x
            });
        }

        

        let color = match self.owner {
            support::Player::Black => conrod_core::color::BLACK,
            support::Player::White => conrod_core::color::WHITE,
            support::Player::None => conrod_core::color::TRANSPARENT,
            support::Player::Valid => conrod_core::color::YELLOW
        };

        // Finally, we'll describe how we want our widget drawn by simply instantiating the
        // necessary primitive graphics widgets.
        //
        // Conrod will automatically determine whether or not any changes have occurred and
        // whether or not any widgets need to be re-drawn.
        //
        // The primitive graphics widgets are special in that their unique state is used within
        // conrod's backend to do the actual drawing. This allows us to build up more complex
        // widgets by using these simple primitives with our familiar layout, coloring, etc
        // methods.
        //
        // If you notice that conrod is missing some sort of primitive graphics that you
        // require, please file an issue or open a PR so we can add it! :)

        // First, we'll draw the **Circle** with a radius that is half our given width.
        let radius = 40.0;
        widget::Circle::fill(radius)
            .middle_of(id)
            .graphics_for(id)
            .color(color)
            .set(state.ids.circle, ui);

        match event {
            Some(x) => Some((x, self.cell)),
            None => None
        }
    }
}
/// Provide the chainable color() configuration method.
impl<'a> Colorable for Tile {
    fn color(mut self, color: conrod_core::Color) -> Self {
        self.style.color = Some(color);
        self
    }
}