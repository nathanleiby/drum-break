mod audio;
mod consts;
mod input;
mod ui;
mod voices;

use std::error::Error;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

use crate::audio::*;
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
async fn main() -> Result<(), Box<dyn Error>> {
    // Setup global game state
    let l = "res/loops/bulls_on_parade_1b.json";
    // let l = "res/loops/samba.json";
    let mut voices = Voices::new_from_file(l)?;

    let mut audio = Audio::new();
    let ui = UI::new(); // Consider passing in audio and voices here?

    loop {
        audio.schedule(&voices);
        handle_user_input(&mut voices, &mut audio)?;
        ui.render(&voices, &audio);

        // wait for next frame from game engine
        next_frame().await
    }
}
