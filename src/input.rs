use std::process;

use macroquad::prelude::*;

use crate::{audio::Audio, consts::*, Voices};

pub fn handle_user_input(voices: &mut Voices, audio: &mut Audio) {
    if is_key_pressed(KeyCode::Space) {
        audio.toggle_pause();
    }

    // Improve UX here
    // Check if down < 0.5s then go fast? (then can use same key incr.. "Up")
    if is_key_pressed(KeyCode::Up) {
        audio.set_bpm(audio.get_bpm() + 1.);
    }
    if is_key_down(KeyCode::Right) {
        audio.set_bpm(audio.get_bpm() + 1.);
    }

    if is_key_pressed(KeyCode::Down) {
        audio.set_bpm(audio.get_bpm() - 1.);
    }

    if is_key_down(KeyCode::Left) {
        audio.set_bpm(audio.get_bpm() - 1.);
    }

    // if is_key_pressed(KeyCode::M) {
    //     // TODO: pause metronome click sound
    // }

    if is_key_pressed(KeyCode::Q) {
        process::exit(0);
    }

    if is_mouse_button_pressed(MouseButton::Left) {
        // TODO: doesn't work on initial window load.. but if i alt-tab away and back it does work?!
        let (mouse_x, mouse_y) = mouse_position();
        // is on a beat?
        let beat = ((mouse_x as f64 - GRID_LEFT_X) / BEAT_WIDTH_PX).floor();
        let row = ((mouse_y as f64 - GRID_TOP_Y) / ROW_HEIGHT).floor();
        if beat >= 0. && beat < BEATS_PER_LOOP && row >= 0. && row < NUM_ROWS_IN_GRID {
            debug!("Clicked on row={}, beat={}", row, beat);
            voices.toggle_beat(row, beat);
        }
    }
}
