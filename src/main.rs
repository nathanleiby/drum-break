mod audio;
mod config;
mod consts;
mod egui_ui;
mod events;
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
use std::sync::mpsc::{self, Receiver};

use crate::config::AppConfig;
use crate::fps::FPS;
use crate::ui::*;
use crate::voices::Voices;

use audio::Audio;
use consts::{TxMsg, ALL_INSTRUMENTS, WINDOW_HEIGHT, WINDOW_WIDTH};
use egui_ui::UIState;
use events::Events;
use input::Input;
use score::compute_last_loop_summary;
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

    let selector_vec = loops.iter().map(|(name, _)| name.to_string()).collect();
    let mut selected_loop_idx = 0;
    // initialize UI state
    //  // TODO: consider bundling the below into a Game struct or similar
    //  desired_hits: &mut Voices
    //  audio: &mut Audio
    //  loops: &Vec<(String, Loop)>
    //  gold_mode: &GoldMode
    //  flags: &Flags

    // debug
    let mut fps_tracker = FPS::new();

    // gameplay loop
    loop {
        // read user's input and translate to events
        let events = input.process();
        // TODO: just add to events?
        let ui_events = ui.flush_events();

        // change game state
        process_system_events(&rx, &mut audio, &voices, &mut gold_mode);
        for e in [&events, &ui_events] {
            process_user_events(
                &mut voices,
                &mut audio,
                &mut flags,
                &loops,
                &mut selected_loop_idx,
                &e,
                &dir_name,
            )?;
        }

        audio.schedule(&voices).await?;

        // render UI
        let mut ui_state = UIState::default().selector_vec(&selector_vec);
        ui_state.set_selected_idx(selected_loop_idx);
        ui_state.set_current_beat(audio.current_beat());
        ui_state.set_current_loop(audio.current_loop() as usize);
        ui_state.set_enabled_beats(&voices);
        ui_state.set_is_playing(!audio.is_paused());
        ui_state.set_bpm(audio.get_bpm() as f32);
        ui_state.set_audio_latency_s(audio.get_configured_audio_latency_seconds() as f32);
        ui_state.set_user_hits(&audio.user_hits);
        ui_state.set_desired_hits(&voices);
        ui_state.set_metronome_enabled(audio.is_metronome_enabled());

        ui.render(&ui_state);
        if flags.ui_debug_mode {
            fps_tracker.update();
            fps_tracker.render();
        }

        // wait for next frame from game engine
        next_frame().await
    }
}

fn process_system_events(
    rx: &Receiver<TxMsg>,
    audio: &mut Audio,
    voices: &Voices,
    gold_mode: &mut GoldMode,
) {
    // read events
    loop {
        match rx.try_recv() {
            Ok(msg) => {
                println!("[event] {:?}", msg);
                match msg {
                    TxMsg::AudioNew => (),
                    TxMsg::StartingLoop(loop_num) => {
                        // TODO: UPDATE TO ONLY RUN THIS CODE FOR "on loop complete" events
                        let last_loop_hits = get_hits_from_nth_loop(
                            &audio.user_hits,
                            (audio.current_loop() - 1) as usize,
                        );
                        let audio_latency = audio.get_configured_audio_latency_seconds();
                        let summary_data =
                            compute_last_loop_summary(&last_loop_hits, &voices, audio_latency);
                        info!("last loop summary = {:?}", summary_data);
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
                }
            }
            Err(_) => break,
        }
    }
}

/// update application state based on events (that came from user input)
fn process_user_events(
    voices: &mut Voices,
    audio: &mut Audio,
    flags: &mut Flags,
    loops: &Vec<(String, Loop)>,
    selected_loop_idx: &mut usize,
    events: &Vec<Events>,
    dir_name: &str,
) -> Result<(), Box<dyn Error>> {
    for event in events {
        info!("Processing event: {:?}", event);
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
            Events::SetBPM(val) => {
                audio.set_bpm(*val);
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
                let res = ALL_INSTRUMENTS
                    .iter()
                    .enumerate()
                    .find(|x| x.0 == *row as usize);
                let ins = match res {
                    Some(x) => x.1,
                    None => panic!("invalid instrument idx"),
                };

                info!("toggling beat: {:?} {:?}", *ins, *beat);
                voices.toggle_beat(*ins, *beat);
            }
            Events::TrackForCalibration => {
                let updated_val = audio.track_for_calibration();
                audio.set_configured_audio_latency_seconds(updated_val);

                let cfg = AppConfig {
                    audio_latency_seconds: updated_val,
                };
                cfg.save()?;
            }
            Events::SetAudioLatency { delta_s: delta } => {
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
            Events::ToggleMetronome => {
                audio.toggle_metronome();
            }
            Events::ChangeLoop(loop_num) => {
                // voices_options.iter().for_each(|(name, new_loop)| {
                // if ui.button(None, format!("{:?} ({:?})", name.as_str(), new_loop.bpm)) {
                let new_loop = loops.as_slice()[*loop_num].clone().1;
                *voices = Voices::new_from_voices_old_model(&new_loop.voices);
                audio.set_bpm(new_loop.bpm as f64);

                *selected_loop_idx = *loop_num;
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
