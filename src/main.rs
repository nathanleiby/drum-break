mod audio;
mod config;
mod consts;
mod fps;
mod input;
mod midi;
mod score;
mod time;
mod ui;
mod voices;

use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::sync::mpsc;

use crate::audio::*;
use crate::config::AppConfig;
use crate::fps::FPS;
use crate::input::*;
use crate::ui::*;
use crate::voices::Voices;

use score::compute_last_loop_summary;
use simple_logger;

use macroquad::prelude::*;
use voices::Loop;

fn window_conf() -> Conf {
    Conf {
        window_title: "Macroix".to_owned(),
        // fullscreen: true,
        window_width: 1280,
        window_height: 720,
        ..Default::default()
    }
}

const GOLD_MODE_BPM_STEP: f64 = 2.;
const GOLD_MODE_CORRECT_TAKES: i32 = 3;

struct Game {
    gold_mode: GoldMode,
}

struct GoldMode {
    current_bpm: f64,
    correct_takes: i32,
}

#[macroquad::main(window_conf)]
async fn main() -> Result<(), Box<dyn Error>> {
    // simple_logger::init_with_level(log::Level::Info).unwrap();
    simple_logger::init_with_env().unwrap();
    let version = include_str!("../VERSION");
    log::info!("version: {}", version);

    let conf = AppConfig::new()?;
    log::debug!("{:?}", &conf);

    let mut input = Input::new();

    // Setup game state
    // read loops
    let dir_name = process_cli_args();
    let mut loops: Vec<(String, Loop)> = Vec::new();
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

    let (tx, rx) = mpsc::channel();

    let mut voices = Voices::new();
    let mut audio = Audio::new(&conf, tx.clone());
    let mut ui = UI::new();

    // Design:
    // Emit an event whenever a loop completes. [x]
    // This event should include the loop's summary.
    //
    // Gold mode tracking maintains state of `num_consecutive_aces`
    // If event was 100%, the `num_consecutive_aces += 1`
    // else `num_consecutive_aces = 0`
    // if num_consecutive_aces == GOLD_MODE_CORRECT_TAKES {
    //      increase BPM
    //      num_consecutive_aces = 0
    //      // notify user in UI - "aced! BPM += {GOLD_MODE_BPM_STEP}" or similar
    // }

    // let mut Game{
    //     gold_mode: GoldMode{
    //         current_bpm
    //     }
    // }

    // debug
    let mut fps_tracker = FPS::new();

    // event reader
    // event writer

    // gameplay loop
    loop {
        // println!("About to read events...");
        // read events
        loop {
            // TODO: handle different kinds of events. Enum it up!
            match rx.try_recv() {
                Ok(msg) => {
                    println!("[event] {msg}");
                    // TODO Get last loop summary
                    let last_loop_hits =
                        get_hits_from_nth_loop(&audio.user_hits, audio.current_loop() - 1);
                    let audio_latency = audio.get_configured_audio_latency_seconds();
                    let lls = compute_last_loop_summary(&last_loop_hits, &voices, audio_latency);
                    println!("last loop summary = {:?}", lls);
                }
                Err(_) => break,
            }
        }

        // process input
        let events = input.process();

        // change state
        process_events(&mut voices, &mut audio, &events, &dir_name)?;
        audio.schedule(&voices).await?;
        fps_tracker.update();

        // render UI
        ui.render(&mut voices, &mut audio, &loops);
        fps_tracker.render();

        // // TODO: pass this deeper, but for now just send event here
        // match tx.send(String::from("testing123")) {
        //     Ok(_) => (),
        //     Err(_) => warn!("error sending tx"),
        // }

        // wait for next frame from game engine
        next_frame().await
    }
}

/// update application state based on events (that came from user input)
fn process_events(
    voices: &mut Voices,
    audio: &mut Audio,
    events: &Vec<Events>,
    dir_name: &str,
) -> Result<(), Box<dyn Error>> {
    for event in events {
        match event {
            Events::UserHit {
                instrument,
                processing_delay,
            } => {
                audio.track_user_hit(*instrument, *processing_delay);
            }
            Events::Pause => {
                audio.toggle_pause();
            }
            Events::ChangeBPM { delta } => {
                audio.set_bpm(audio.get_bpm() + delta);
            }
            Events::Quit => {
                std::process::exit(0);
            }
            Events::ResetHits => {
                audio.user_hits = vec![];
            }
            Events::SaveLoop => {
                // write serialized JSON output to a file
                let dir_name = dir_name.trim_end_matches('/');
                let file = File::create(format!("{}/loop-{}.json", dir_name, get_time()))?;
                let mut writer = BufWriter::new(file);
                let my_loop = Loop {
                    bpm: audio.get_bpm() as usize,
                    voices: voices.clone(),
                };
                serde_json::to_writer(&mut writer, &my_loop)?;
                writer.flush()?;
            }
            Events::ToggleBeat { row, beat } => {
                voices.toggle_beat(*row, *beat);
            }
            Events::TrackForCalibration => {
                let updated_val = audio.track_for_calibration();
                audio.set_configured_audio_latency_seconds(updated_val);

                let cfg = AppConfig {
                    audio_latency_seconds: updated_val,
                };
                cfg.save()?;
            }
            Events::SetAudioLatency { delta } => {
                let updated_val = audio.get_configured_audio_latency_seconds() + delta;
                audio.set_configured_audio_latency_seconds(updated_val);

                let cfg = AppConfig {
                    audio_latency_seconds: updated_val,
                };
                cfg.save()?;
            }
        }
    }

    Ok(())
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
