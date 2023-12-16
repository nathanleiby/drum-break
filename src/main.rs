mod audio;
mod consts;
mod input;
mod ui;
mod voices;

use crate::audio::*;
use crate::consts::*;
use crate::input::*;
use crate::ui::*;

use crate::voices::Voices;
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Macroix".to_owned(),
        // fullscreen: true,
        window_width: 1280,
        window_height: 720,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // Setup global game state
    let mut voices = Voices::new_samba();
    let mut audio = Audio::new();
    let ui = UI::new(); // Consider passing in audio and voices here?

    // debug state
    let mut last_beat = -1;

    loop {
        // schedule upcoming audio
        audio.schedule(&voices);

        // user input
        handle_user_input(&mut voices, &mut audio);

        // render ui
        let current_beat = audio.current_clock_tick() % BEATS_PER_LOOP;
        ui.render(&voices, audio.get_bpm(), current_beat);

        // print debug info
        if (current_beat as i32) > last_beat {
            debug!("Beat: {}", current_beat as i32);
            last_beat = current_beat as i32;
        }

        // wait for next frame from game engine
        next_frame().await
    }
}
