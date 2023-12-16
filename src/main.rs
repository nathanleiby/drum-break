mod audio;
mod consts;
mod ui;

use std::process;

use crate::audio::*;
use crate::consts::*;
use crate::ui::*;

use macroquad::prelude::*;

use kira::{
    clock::{ClockHandle, ClockSpeed},
    manager::{backend::DefaultBackend, AudioManager, AudioManagerSettings},
    tween::Tween,
};

fn window_conf() -> Conf {
    Conf {
        window_title: "Macroix".to_owned(),
        // fullscreen: true,
        window_width: 1280,
        window_height: 720,
        ..Default::default()
    }
}

/// Voices represents the notes to be played on each instrument.
pub struct Voices {
    metronome: Vec<f64>,
    closed_hihat: Vec<f64>,
    snare: Vec<f64>,
    kick: Vec<f64>,
    open_hihat: Vec<f64>,
}

impl Voices {
    fn new() -> Self {
        Self {
            metronome: vec![],
            closed_hihat: vec![],
            snare: vec![],
            kick: vec![],
            open_hihat: vec![],
        }
    }

    fn samba() -> Self {
        // let lambda = |x: f64| (x - 1.) / 2.; // 8 quarter note beats per loop
        let lambda = |x: f64| (x - 1.);
        let closed_hihat_notes = vec![1., 3., 4., 5., 7., 8., 9., 11., 12., 13., 15., 16.]
            .into_iter()
            .map(lambda)
            .collect();
        let snare_notes = vec![1., 3., 6., 8., 10., 13., 15.]
            .into_iter()
            .map(lambda)
            .collect();
        let kick_notes: Vec<f64> = vec![1., 4., 5., 8., 9., 12., 13., 16.]
            .into_iter()
            .map(lambda)
            .collect();
        let open_hihat_notes: Vec<f64> = vec![3., 7., 11., 15.].into_iter().map(lambda).collect();
        let metronome_notes: Vec<f64> = (0..16).into_iter().map(|x| x as f64).collect();
        Self {
            metronome: metronome_notes,
            closed_hihat: closed_hihat_notes,
            snare: snare_notes,
            kick: kick_notes,
            open_hihat: open_hihat_notes,
        }
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // Setup global game state
    let mut bpm: f64 = 120.0;

    let mut manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).unwrap();
    let clock = manager
        .add_clock(ClockSpeed::TicksPerMinute(bpm * 2. as f64))
        .unwrap();

    let mut last_scheduled_tick = -1.;
    let mut last_beat = -1;

    let mut voices = Voices::samba();

    loop {
        ////////////////////////////
        // Schedule audio
        ////////////////////////////
        last_scheduled_tick = audio(&voices, &mut manager, &clock, last_scheduled_tick);

        ////////////////////////////
        // Handle User Input
        ////////////////////////////

        if is_key_pressed(KeyCode::Space) {
            if clock.ticking() {
                clock.pause().unwrap();
            } else {
                clock.start().unwrap();
            }
        }

        // Improve UX here
        // Check if down < 0.5s then go fast? (then can use same key incr.. "Up")
        if is_key_pressed(KeyCode::Up) {
            bpm += 1.;
            bpm = bpm.max(MIN_BPM).min(MAX_BPM);
            clock
                .set_speed(ClockSpeed::TicksPerMinute(bpm * 2.), Tween::default())
                .unwrap();
        }
        if is_key_down(KeyCode::Right) {
            bpm += 1.;
            bpm = bpm.max(MIN_BPM).min(MAX_BPM);
            clock
                .set_speed(ClockSpeed::TicksPerMinute(bpm * 2.), Tween::default())
                .unwrap();
        }

        if is_key_pressed(KeyCode::Down) {
            bpm -= 1.;
            bpm = bpm.max(MIN_BPM).min(MAX_BPM);
            clock
                .set_speed(ClockSpeed::TicksPerMinute(bpm * 2.), Tween::default())
                .unwrap();
        }
        if is_key_down(KeyCode::Left) {
            // Check if down < 0.5s then go fast?
            bpm -= 1.;
            bpm = bpm.max(MIN_BPM).min(MAX_BPM);
            clock
                .set_speed(ClockSpeed::TicksPerMinute(bpm * 2.), Tween::default())
                .unwrap();
        }

        if is_key_pressed(KeyCode::M) {
            // TODO: pause metronome click sound
        }

        if is_key_pressed(KeyCode::Q) {
            process::exit(0);
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            // TODO: doesn't work on initial window load.. but if i alt-tab away and back it does work?!
            let (mouse_x, mouse_y) = mouse_position();
            // is on a beat?
            let beat = ((mouse_x as f64 - GRID_LEFT_X) / BEAT_WIDTH_PX).floor();
            let row = ((mouse_y as f64 - GRID_TOP_Y) / ROW_HEIGHT).floor();
            if beat >= 0. && beat < BEATS_PER_LOOP && row >= 0. && row < NUM_ROWS_IN_GRID {
                debug!("Clicked on row={}, beat={}", row, beat);
                if row == 0. {
                    if let Some(pos) = voices.closed_hihat.iter().position(|x| *x == beat) {
                        voices.closed_hihat.remove(pos);
                    } else {
                        voices.closed_hihat.push(beat);
                    }
                } else if row == 1. {
                    if let Some(pos) = voices.snare.iter().position(|x| *x == beat) {
                        voices.snare.remove(pos);
                    } else {
                        voices.snare.push(beat);
                    }
                } else if row == 2. {
                    if let Some(pos) = voices.kick.iter().position(|x| *x == beat) {
                        voices.kick.remove(pos);
                    } else {
                        voices.kick.push(beat);
                    }
                } else if row == 3. {
                    if let Some(pos) = voices.open_hihat.iter().position(|x| *x == beat) {
                        voices.open_hihat.remove(pos);
                    } else {
                        voices.open_hihat.push(beat);
                    }
                }
            }
        }

        ////////////////////////////
        // Render UI
        ////////////////////////////
        // Get current beat (from 0 to BEATS_PER_LOOP)
        let current_beat = current_clock_tick(&clock) % BEATS_PER_LOOP;
        if (current_beat as i32) > last_beat {
            debug!("Beat: {}", current_beat as i32);
            last_beat = current_beat as i32;
        }

        render_ui(&voices, bpm, current_beat);

        ////////////////////////////
        // Game Loop -- next frame
        ////////////////////////////
        next_frame().await
    }
}

fn current_clock_tick(clock: &ClockHandle) -> f64 {
    clock.time().ticks as f64 + clock.fractional_position() as f64
}
