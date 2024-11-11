/*
Display the UI.

The UI is built in EGUI.
*/
use crate::{
    consts::{UserHit, BEATS_PER_LOOP},
    egui_ui::{draw_ui, UIState},
    events::Events,
    score::MISS_MARGIN,
};

pub struct UI {
    events: Vec<Events>,
}

impl UI {
    pub fn new() -> Self {
        Self { events: vec![] }
    }

    pub fn render(&mut self, ui_state: &UIState) {
        egui_macroquad::ui(|egui_ctx| draw_ui(egui_ctx, ui_state, &mut self.events));
        egui_macroquad::draw();
    }

    pub fn flush_events(&mut self) -> Vec<Events> {
        let out = self.events.clone();
        self.events = vec![];
        out
    }
}

pub fn get_hits_from_nth_loop(user_hits: &Vec<UserHit>, desired_loop_idx: usize) -> Vec<UserHit> {
    let last_loop_hits: Vec<UserHit> = user_hits
        .iter()
        .filter(|hit| {
            // include hits from just before start of loop (back to 0 - MISS), since those could be early or on-time hits
            let loop_num_for_hit = ((hit.clock_tick + MISS_MARGIN) / BEATS_PER_LOOP) as usize;
            loop_num_for_hit == desired_loop_idx
        }).cloned()
        .collect::<Vec<UserHit>>();
    last_loop_hits
}
