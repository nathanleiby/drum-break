mod audio;
mod config;
mod consts;
mod egui_ui;
mod events;
mod fps;
mod game;
mod keyboard_input_handler;

#[cfg(not(target_arch = "wasm32"))]
mod midi;
#[cfg(not(target_arch = "wasm32"))]
mod midi_input_handler;
use cvars_console_macroquad::MacroquadConsole;
#[cfg(not(target_arch = "wasm32"))]
use midi_input_handler::MidiInputHandler;

mod score;
mod time;
mod ui;
mod voices;

use std::error::Error;
use std::sync::mpsc::{self};

use crate::config::AppConfig;
use crate::fps::FPS;
use crate::ui::*;

use audio::Audio;
use consts::{WINDOW_HEIGHT, WINDOW_WIDTH};
use game::{compute_ui_state, process_system_events, process_user_events, GameState, Loops};
use keyboard_input_handler::KeyboardInputHandler;
use simple_logger;

use macroquad::prelude::*;
use voices::Loop;

fn window_conf() -> Conf {
    Conf {
        window_title: "Macroix".to_owned(),
        // fullscreen: true,
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        high_dpi: true,
        ..Default::default()
    }
}

use cvars::cvars;

// TODO: Can I wrap make a static var wrapped with std::sync::LazyLock, so this let's me effectively edit globals
// Or do I need to pass the value down via dependency injection?
cvars! {
    //! Documentation for the generated struct
    #![derive(Debug, Clone)]
    #![cvars(sorted)]

    /// Documentation for the cvar
    g_test_bool: bool = false,
}

impl Cvars {
    /// Create a new Cvars object with the default RecWars settings.
    pub fn new() -> Self {
        Self::default()
    }
}

// TOOD: move this to an env var controlled flag or similar
const MOCK_INITIAL_STATE: bool = false;

#[macroquad::main(window_conf)]
async fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(not(target_arch = "wasm32"))]
    simple_logger::init_with_env().unwrap();

    let mut my_cvars = Cvars::new();
    let mut macroquad_console = MacroquadConsole::new();

    let version = include_str!("../VERSION");
    log::info!("version: {}", version);

    let dir_name = process_cli_args();

    // Setup game state
    // read loops
    let mut loops: Loops = Vec::new();
    // TODO: does this need to be async still?
    match read_loops(&dir_name).await {
        Ok(loops_from_dir) => loops = loops_from_dir,
        Err(e) => {
            log::warn!(
                "warning: unable to read loops from given directory ({}) due to '{}'",
                &dir_name,
                e
            )
        }
    }

    let keyboard_input = KeyboardInputHandler::new();
    #[cfg(not(target_arch = "wasm32"))]
    let mut midi_input = MidiInputHandler::new();

    let mut gs = if MOCK_INITIAL_STATE {
        GameState::new_mock_game_state()
    } else {
        GameState::new(loops)
    };

    // Setup audio, which runs on a separate thread and passes messages back.
    // TODO: Get rid of the shared state here (see how we compute_ui_state()), and just use message passing to update the GameState
    let (tx, rx) = mpsc::channel();
    // let conf = AppConfig::new()?; // TODO: Get rid of conf lib for now to simplify? This is the only usage
    let conf = AppConfig::new();
    log::debug!("App Config: {:?}", &conf);

    let mut audio = if MOCK_INITIAL_STATE {
        Audio::new_mock(&conf, tx.clone())
    } else {
        Audio::new(&conf, tx.clone())
    };

    // debug
    let mut fps_tracker = FPS::new();

    let mut ui = UI::new();
    loop {
        let mut events = Vec::new();
        // read user's input and translate to events
        if !macroquad_console.is_open() {
            events.extend(keyboard_input.process());
        }
        events.extend(ui.flush_events());

        #[cfg(not(target_arch = "wasm32"))]
        events.extend(midi_input.process());

        // change game state
        process_system_events(&rx, &mut audio, &gs.voices, &mut gs.gold_mode);
        process_user_events(
            &mut gs.voices,
            &mut audio,
            &mut gs.flags,
            &gs.loops,
            &mut gs.selected_loop_idx,
            &events,
            &dir_name,
            &mut gs.correct_margin,
            &mut gs.miss_margin,
        )?;

        audio.schedule(&gs.voices).await?;

        // render UI
        ui.render(&compute_ui_state(&gs, &audio));
        macroquad_console.update(&mut my_cvars);
        if gs.flags.ui_debug_mode {
            fps_tracker.update();
            fps_tracker.render();
        }

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

async fn read_loops(dir_name: &str) -> Result<Vec<(String, Loop)>, Box<dyn Error>> {
    // get all file names from the dir
    let paths = std::fs::read_dir(dir_name)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, std::io::Error>>()?;

    // for each file name, load the file into Voices
    let mut loops = Vec::<(String, Loop)>::new();
    for path in &paths {
        let p = path.to_str().expect("unable to convert PathBuf to string");
        let v = Loop::new_from_file_async(p).await?;

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
