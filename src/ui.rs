/*
Display the UI.

The UI is built in EGUI.
*/
use crate::{
    egui_ui::{layout_ui, UIState},
    events::Events,
};

pub struct UI {
    events: Vec<Events>,
}

// TODO: Move the EGUI Macroquad stuff up to this level. No need for a wrapper.

impl UI {
    pub fn new() -> Self {
        Self { events: vec![] }
    }

    pub fn render(&mut self, ui_state: &UIState) {
        egui_macroquad::ui(|egui_ctx| layout_ui(egui_ctx, ui_state, &mut self.events));
        egui_macroquad::draw();
    }

    pub fn flush_events(&mut self) -> Vec<Events> {
        let out = self.events.clone();
        self.events = vec![];
        out
    }
}
