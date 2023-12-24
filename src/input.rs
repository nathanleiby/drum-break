use std::{
    collections::HashSet,
    error::Error,
    fs::File,
    io::{BufWriter, Write},
    process,
};

use macroquad::prelude::*;

use crate::{
    audio::Audio,
    config::{AppConfig, InputConfigMidi},
    consts::*,
    midi::MidiInput,
    voices::Instrument,
    Voices,
};

pub struct Input {
    midi_input: MidiInput,
}

impl Input {
    pub fn new() -> Self {
        let mut midi_input = MidiInput::new();
        midi_input.connect();

        Self { midi_input }
    }

    pub fn process(
        self: &mut Self,
        voices: &mut Voices,
        audio: &mut Audio,
        dir_name: &str,
    ) -> Result<(), Box<dyn Error>> {
        // midi device: "MPK Mini Mk II"
        let mpk_mini_mk_ii = InputConfigMidi {
            closed_hi_hat: HashSet::from_iter(vec![44, 48]),
            snare: HashSet::from_iter(vec![45, 49]),
            kick: HashSet::from_iter(vec![46, 50]),
            open_hi_hat: HashSet::from_iter(vec![47, 51]),
        };
        let td17 = InputConfigMidi {
            closed_hi_hat: HashSet::from_iter(vec![42, 22]),
            snare: HashSet::from_iter(vec![38]),
            kick: HashSet::from_iter(vec![36]),
            open_hi_hat: HashSet::from_iter(vec![46, 26]),
        };
        let alesis_nitro = InputConfigMidi {
            closed_hi_hat: HashSet::from_iter(vec![42]),
            snare: HashSet::from_iter(vec![38]),
            kick: HashSet::from_iter(vec![36]),
            open_hi_hat: HashSet::from_iter(vec![46, 23]), // allow half-open (23)
        };
        let ic_midi = match self.midi_input.get_device_name() {
            s if s == "MPK Mini Mk II" => mpk_mini_mk_ii,
            s if s.contains("TD-17") => td17,
            s if s.contains("Nitro") => alesis_nitro,
            _ => {
                warn!("warning: unknown midi device, using default of 'alesis nitro'");
                alesis_nitro
            }
        };

        let pressed_midi = HashSet::from_iter(self.midi_input.get_pressed_buttons());
        self.midi_input.flush();

        // Playing the drums //
        if is_key_pressed(KeyCode::Z)
            || ic_midi.closed_hi_hat.intersection(&pressed_midi).count() > 0
        {
            audio.track_user_hit(Instrument::ClosedHihat);
        }

        if is_key_pressed(KeyCode::X) || ic_midi.snare.intersection(&pressed_midi).count() > 0 {
            audio.track_user_hit(Instrument::Snare);
        }

        if is_key_pressed(KeyCode::C) || ic_midi.kick.intersection(&pressed_midi).count() > 0 {
            audio.track_user_hit(Instrument::Kick);
        }

        if is_key_pressed(KeyCode::V) || ic_midi.open_hi_hat.intersection(&pressed_midi).count() > 0
        {
            audio.track_user_hit(Instrument::OpenHihat);
        }

        if is_key_pressed(KeyCode::Space) {
            audio.toggle_pause();
        }

        if is_key_pressed(KeyCode::Equal) {
            // capture note timing data
            let updated_val = audio.track_for_calibration();
            audio.set_configured_audio_latency_seconds(updated_val);

            let cfg = AppConfig {
                audio_latency_seconds: updated_val,
            };
            cfg.save()?;
        }

        if is_key_pressed(KeyCode::LeftBracket) {
            let mut multiplier = 1.;
            if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
                multiplier = 100.;
            }

            let updated_val = audio.get_configured_audio_latency_seconds() - (0.001 * multiplier);
            audio.set_configured_audio_latency_seconds(updated_val);

            let cfg = AppConfig {
                audio_latency_seconds: updated_val,
            };
            cfg.save()?;
        }

        if is_key_pressed(KeyCode::RightBracket) {
            let mut multiplier = 1.;
            if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
                multiplier = 100.;
            }

            let updated_val = audio.get_configured_audio_latency_seconds() + (0.001 * multiplier);
            audio.set_configured_audio_latency_seconds(updated_val);

            let cfg = AppConfig {
                audio_latency_seconds: updated_val,
            };
            cfg.save()?;
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

        if is_key_pressed(KeyCode::S) {
            // write serialized JSON output to a file
            let dir_name = dir_name.trim_end_matches('/');
            let file = File::create(format!("{}/loop-{}.json", dir_name, get_time()))?;
            let mut writer = BufWriter::new(file);
            serde_json::to_writer(&mut writer, &voices)?;
            writer.flush()?;
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

        Ok(())
    }
}
