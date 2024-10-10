use std::{fs::File, io::BufWriter, io::Write};

use std::error::Error;
use std::sync::mpsc::Receiver;

use crate::audio::Audio;
use crate::config::AppConfig;
use crate::consts::TxMsg;
use crate::egui_ui::UIState;
use crate::score::compute_last_loop_summary;
use crate::ui::*;
use crate::voices::Voices;

use log::info;
use macroquad::prelude::*;

use crate::{consts::ALL_INSTRUMENTS, events::Events, voices::Loop};

const GOLD_MODE_BPM_STEP: f64 = 2.;
const GOLD_MODE_CORRECT_TAKES: i32 = 3;

pub struct GoldMode {
    pub correct_takes: i32,
    pub was_gold: bool,
}

pub struct Flags {
    pub ui_debug_mode: bool,
    pub dev_tools_visible: bool,
    pub help_visible: bool,
}

impl Flags {
    pub fn new() -> Self {
        return Self {
            ui_debug_mode: false,
            dev_tools_visible: false,
            help_visible: false,
        };
    }
}

pub type Loops = Vec<(String, Loop)>;

pub struct GameState {
    pub voices: Voices,
    pub gold_mode: GoldMode,
    pub selected_loop_idx: usize,
    pub loops: Loops,
    pub flags: Flags,
    pub correct_margin: f64,
    pub miss_margin: f64,
}

impl GameState {
    pub fn new(loops: Loops) -> Self {
        Self {
            voices: Voices::new(),
            gold_mode: GoldMode {
                correct_takes: 0,
                was_gold: false,
            },
            selected_loop_idx: 0,
            loops,
            flags: Flags::new(),
            correct_margin: 0.151,
            miss_margin: 0.3,
        }
    }
}

pub fn compute_ui_state(gs: &GameState, audio: &Audio) -> UIState {
    let selector_vec = gs.loops.iter().map(|(name, _)| name.to_string()).collect();
    let mut ui_state = UIState::default().selector_vec(&selector_vec);
    ui_state.set_selected_idx(gs.selected_loop_idx);
    ui_state.set_current_beat(audio.current_beat());
    ui_state.set_current_loop(audio.current_loop() as usize);
    ui_state.set_enabled_beats(&gs.voices);
    ui_state.set_is_playing(!audio.is_paused());
    ui_state.set_bpm(audio.get_bpm() as f32);
    ui_state.set_audio_latency_s(audio.get_configured_audio_latency_seconds() as f32);
    ui_state.set_user_hits(&audio.user_hits);
    ui_state.set_desired_hits(&gs.voices);
    ui_state.set_metronome_enabled(audio.is_metronome_enabled());

    ui_state.set_is_dev_tools_visible(gs.flags.dev_tools_visible);
    ui_state.set_correct_margin(gs.correct_margin);
    ui_state.set_miss_margin(gs.miss_margin);
    ui_state.set_is_help_visible(gs.flags.help_visible);
    ui_state
}

pub fn process_system_events(
    rx: &Receiver<TxMsg>,
    audio: &mut Audio,
    voices: &Voices,
    gold_mode: &mut GoldMode,
) {
    // read events
    loop {
        match rx.try_recv() {
            Ok(msg) => {
                println!("[system event] {:?}", msg);
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
                        if totals.score() == 1. {
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
pub fn process_user_events(
    voices: &mut Voices,
    audio: &mut Audio,
    flags: &mut Flags,
    loops: &Vec<(String, Loop)>,
    selected_loop_idx: &mut usize,
    events: &Vec<Events>,
    dir_name: &str,
    correct_margin: &mut f64,
    miss_margin: &mut f64,
) -> Result<(), Box<dyn Error>> {
    for event in events {
        println!("[user event] {:?}", event);
        // info!("Processing event: {:?}", event);
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
                cfg.save();
            }
            Events::SetAudioLatency { delta_s: delta } => {
                let updated_val = audio.get_configured_audio_latency_seconds() + delta;
                audio.set_configured_audio_latency_seconds(updated_val);

                let cfg = AppConfig {
                    audio_latency_seconds: updated_val,
                };
                cfg.save();
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
            Events::ToggleDevToolsVisibility => {
                flags.dev_tools_visible = !flags.dev_tools_visible;
            }
            Events::SetCorrectMargin(val) => {
                *correct_margin = *val;
            }
            Events::SetMissMargin(val) => {
                *miss_margin = *val;
            }
            Events::ToggleHelpVisibility => {
                flags.help_visible = !flags.help_visible;
            }
        }
    }

    Ok(())
}
