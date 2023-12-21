use std::{
    error::Error,
    fs::File,
    io::{BufWriter, Write},
    process,
};

use macroquad::prelude::*;

use crate::{
    audio::Audio, config::AppConfig, consts::*, midi::MidiInput, voices::Instrument, Voices,
};

pub fn handle_user_input(
    voices: &mut Voices,
    audio: &mut Audio,
    midi: &MidiInput,
) -> Result<(), Box<dyn Error>> {
    // Playing the drums //
    if is_key_pressed(KeyCode::Z) || midi.is_button_pressed(48) {
        audio.track_user_hit(Instrument::ClosedHihat);
    }

    if is_key_pressed(KeyCode::X) || midi.is_button_pressed(49) {
        audio.track_user_hit(Instrument::Snare);
    }

    if is_key_pressed(KeyCode::C) || midi.is_button_pressed(50) {
        audio.track_user_hit(Instrument::Kick);
    }

    if is_key_pressed(KeyCode::V) || midi.is_button_pressed(51) {
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
        let file = File::create(format!("res/loops/voices-{}.json", get_time()))?;
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