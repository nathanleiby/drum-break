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

use consts::{WINDOW_HEIGHT, WINDOW_WIDTH};
use score::compute_last_loop_summary;
use simple_logger;

use macroquad::prelude::*;
use voices::{Instrument, Loop};

fn window_conf() -> Conf {
    Conf {
        window_title: "Macroix".to_owned(),
        // fullscreen: true,
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        ..Default::default()
    }
}

const GOLD_MODE_BPM_STEP: f64 = 2.;
const GOLD_MODE_CORRECT_TAKES: i32 = 3;

pub struct GoldMode {
    correct_takes: i32,
    was_gold: bool,
}

pub struct Flags {
    ui_debug_mode: bool,
}

impl Flags {
    pub fn new() -> Self {
        return Self {
            ui_debug_mode: false,
        };
    }
}

#[macroquad::main(window_conf)]
async fn main() -> Result<(), Box<dyn Error>> {
    // simple_logger::init_with_level(log::Level::Info).unwrap();
    simple_logger::init_with_env().unwrap();
    let version = include_str!("../VERSION");
    log::info!("version: {}", version);

    let conf = AppConfig::new()?;
    log::debug!("{:?}", &conf);

    let mut flags = Flags::new();
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

    let mut gold_mode = GoldMode {
        correct_takes: 0,
        was_gold: false,
    };

    // debug
    let mut fps_tracker = FPS::new();

    // gameplay loop
    loop {
        // read events
        loop {
            // TODO: handle different kinds of events. Enum it up!
            match rx.try_recv() {
                Ok(msg) => {
                    println!("[event] {msg}");

                    // TODO: UPDATE TO ONLY RUN THIS CODE FOR "on loop complete" events
                    let last_loop_hits =
                        get_hits_from_nth_loop(&audio.user_hits, audio.current_loop() - 1);
                    let audio_latency = audio.get_configured_audio_latency_seconds();
                    let summary_data =
                        compute_last_loop_summary(&last_loop_hits, &voices, audio_latency);
                    println!("last loop summary = {:?}", summary_data);
                    let totals = summary_data.total();

                    gold_mode.was_gold = false;
                    if totals.ratio() == 1. {
                        gold_mode.correct_takes += 1;
                    } else {
                        gold_mode.correct_takes = 0;
                    }

                    if gold_mode.correct_takes == GOLD_MODE_CORRECT_TAKES {
                        audio.set_bpm(audio.get_bpm() + GOLD_MODE_BPM_STEP);
                        gold_mode.correct_takes = 0;
                        gold_mode.was_gold = true;
                        // TODO: schedule a 1-off "success!" SFX to play
                        // TOOD: Maybe -- clear existing noise from mistaken notes
                    }
                }
                Err(_) => break,
            }
        }

        // process input
        let events = input.process();

        // change state
        process_input_events(&mut voices, &mut audio, &mut flags, &events, &dir_name)?;
        audio.schedule(&voices).await?;
        fps_tracker.update();

        // render UI
        ui.render(&mut voices, &mut audio, &loops, &gold_mode, &flags);
        if flags.ui_debug_mode {
            fps_tracker.render();
        }

        // wait for next frame from game engine
        next_frame().await
    }
}

/// update application state based on events (that came from user input)
fn process_input_events(
    voices: &mut Voices,
    audio: &mut Audio,
    flags: &mut Flags,
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
                    voices: voices.to_voices_old_model(),
                };
                serde_json::to_writer(&mut writer, &my_loop)?;
                writer.flush()?;
            }
            Events::ToggleBeat { row, beat } => {
                // map from UI display to instrument
                let ins = match *row as usize {
                    0 => Instrument::ClosedHihat,
                    1 => Instrument::Snare,
                    2 => Instrument::Kick,
                    3 => Instrument::OpenHihat,
                    4 => Instrument::Ride,
                    5 => Instrument::Crash,
                    _ => panic!("invalid instrument idx"),
                };
                voices.toggle_beat(ins, *beat);
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
            Events::ToggleDebugMode => {
                flags.ui_debug_mode = !flags.ui_debug_mode;
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
