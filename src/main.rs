mod audio;
mod config;
mod consts;
mod fps;
mod input;
mod midi;
mod score;
mod ui;
mod voices;

use std::error::Error;

use crate::audio::*;
use crate::config::AppConfig;
use crate::fps::FPS;
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
    let version = include_str!("../VERSION");
    debug!("version: {}", version);

    let conf = AppConfig::new()?;
    dbg!(&conf);

    // read loops
    let dir_name = process_cli_args();
    let mut loops: Vec<(String, Voices)> = Vec::new();
    if let Ok(loops_from_dir) = read_loops(&dir_name).await {
        loops = loops_from_dir;
    } else {
        warn!(
            "warning: unable to read loops from given directory ({})",
            &dir_name
        )
    }

    // Setup global game state
    let mut input = Input::new();
    let mut voices = Voices::new();
    let mut audio = Audio::new(&conf);

    let mut ui = UI::new(); // Consider passing in audio and voices here?

    // debug
    let mut fps_tracker = FPS::new();

    loop {
        audio.schedule(&voices).await?;

        input.process(&mut voices, &mut audio, &dir_name)?;

        ui.render(&mut voices, &audio, &loops);

        // debug
        fps_tracker.update();
        fps_tracker.render();

        // wait for next frame from game engine
        next_frame().await
    }
}

fn process_cli_args() -> String {
    // read commnand line arg as directory name
    let dir_name = std::env::args()
        .nth(1)
        .unwrap_or_else(|| return "res/loops/".to_string());

    return dir_name;
}

async fn read_loops(dir_name: &str) -> Result<Vec<(String, Voices)>, Box<dyn Error>> {
    // get all file names from the dir
    let paths = std::fs::read_dir(dir_name)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, std::io::Error>>()?;

    // for each file name, load the file into Voices
    let mut loops = Vec::<(String, Voices)>::new();
    for path in &paths {
        let p = path.to_str().expect("unable to convert PathBuf to string");
        let v = Voices::new_from_file(p).await?;

        // get just the file name from the path
        let n = path
            .file_name()
            .expect("unable to get file name from path")
            .to_str()
            .expect("unable to convert OsStr to str");

        // remove the file extension
        let n = n.split(".json").next().expect("unable to split file name");

        loops.push((n.to_string(), v));
    }

    // sort loops by name
    loops.sort_by(|a, b| a.0.cmp(&b.0));

    Ok(loops)
}
