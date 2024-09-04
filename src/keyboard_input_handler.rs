/*
  Capture user input (keyboard, midi) and convert it into events.
*/

use macroquad::prelude::*;

use crate::{consts::*, events::Events};

pub struct KeyboardInputHandler {}

impl KeyboardInputHandler {
    pub fn new() -> Self {
        Self {}
    }

    /// convert any user input from the last frame into Events
    pub fn process(self: &Self) -> Vec<Events> {
        let mut events: Vec<Events> = vec![];

        // Playing the drums //
        let processing_delay = 0.; // TODO: solve this for keyboard input, too.
                                   // Right now we don't know the delay between key press and frame start .. we could improve by guessing midway through the previous frame (1/2 frame duration) without any knowledge

        for (idx, ins) in ALL_INSTRUMENTS.iter().enumerate() {
            let key_code = match idx {
                0 => KeyCode::Key1,
                1 => KeyCode::Key2,
                2 => KeyCode::Key3,
                3 => KeyCode::Key4,
                4 => KeyCode::Key5,
                5 => KeyCode::Key6,
                6 => KeyCode::Key7,
                7 => KeyCode::Key8,
                8 => KeyCode::Key9,
                9 => KeyCode::Key0,
                _ => panic!("more than hard-coded num instruments, failed to map key codes"),
            };
            if is_key_pressed(key_code) {
                events.push(Events::UserHit {
                    instrument: *ins,
                    processing_delay,
                });
            }
        }

        if is_key_pressed(KeyCode::Space) {
            events.push(Events::Pause)
        }

        if is_key_pressed(KeyCode::Equal) {
            events.push(Events::TrackForCalibration);
        }

        if is_key_pressed(KeyCode::LeftBracket) {
            let mut multiplier = 1.;
            if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
                multiplier = 100.;
            }
            events.push(Events::SetAudioLatency {
                delta_s: -0.001 * multiplier,
            });
        }

        if is_key_pressed(KeyCode::RightBracket) {
            let mut multiplier = 1.;
            if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
                multiplier = 100.;
            }
            events.push(Events::SetAudioLatency {
                delta_s: 0.001 * multiplier,
            });
        }

        // Improve UX here
        // Check if down < 0.5s then go fast? (then can use same key incr.. "Up")
        if is_key_pressed(KeyCode::Up) {
            events.push(Events::ChangeBPM { delta: 1. });
        }
        if is_key_down(KeyCode::Right) {
            events.push(Events::ChangeBPM { delta: 1. });
        }

        if is_key_pressed(KeyCode::Down) {
            events.push(Events::ChangeBPM { delta: -1. });
        }

        if is_key_down(KeyCode::Left) {
            events.push(Events::ChangeBPM { delta: -1. });
        }

        if is_key_pressed(KeyCode::M) {
            events.push(Events::ToggleMetronome);
        }

        if is_key_pressed(KeyCode::Z) {
            events.push(Events::ToggleDebugMode);
        }

        if is_key_pressed(KeyCode::Q) {
            events.push(Events::Quit)
        }

        if is_key_pressed(KeyCode::R) {
            events.push(Events::ResetHits)
        }

        if is_key_pressed(KeyCode::X) {
            events.push(Events::SaveLoop);
        }

        events
    }
}
