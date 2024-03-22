use std::error::Error;

use macroquad::{file::load_file, logging::info};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Instrument {
    ClosedHihat,
    Snare,
    Kick,
    OpenHihat,
    // PedalHiHat,
    // Ride,
    // RideBell,
    // LTom,
    // MTom,
    // HTom,
    // Crash,
}

/// Voices represents the notes to be played on each instrument.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Voice {
    pub instrument: Instrument,
    pub beat_timings: Vec<f64>,
}

impl Voice {
    pub fn new() -> Self {
        Self {
            instrument: Instrument::Snare,
            beat_timings: vec![],
        }
    }
}

/// Voices represents the notes to be played on each instrument.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Voices {
    pub closed_hihat: Vec<f64>,
    pub snare: Vec<f64>,
    pub kick: Vec<f64>,
    pub open_hihat: Vec<f64>,
}

impl Voices {
    pub fn new() -> Self {
        Self {
            closed_hihat: vec![],
            snare: vec![],
            kick: vec![],
            open_hihat: vec![],
        }
    }

    pub fn toggle_beat(&mut self, row: f64, beat: f64) {
        if row == 0. {
            if let Some(pos) = self.closed_hihat.iter().position(|x| *x == beat) {
                self.closed_hihat.remove(pos);
            } else {
                self.closed_hihat.push(beat);
            }
        } else if row == 1. {
            if let Some(pos) = self.snare.iter().position(|x| *x == beat) {
                self.snare.remove(pos);
            } else {
                self.snare.push(beat);
            }
        } else if row == 2. {
            if let Some(pos) = self.kick.iter().position(|x| *x == beat) {
                self.kick.remove(pos);
            } else {
                self.kick.push(beat);
            }
        } else if row == 3. {
            if let Some(pos) = self.open_hihat.iter().position(|x| *x == beat) {
                self.open_hihat.remove(pos);
            } else {
                self.open_hihat.push(beat);
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Loop {
    pub bpm: usize,
    pub voices: Voices,
}

impl Loop {
    pub async fn new_from_file(path: &str) -> Result<Self, Box<dyn Error>> {
        info!("Loop::new_from_file .. {}", path);
        let f = load_file(path).await?;
        let out: Self = serde_json::from_reader(&*f)?;
        Ok(out)
    }
}

//// TODO: Get test working.. it's async so needs some special setup

// #[cfg(test)]
// mod tests {
//     use std::f64::EPSILON;

//     use crate::{
//         consts::BEATS_PER_LOOP,
//         score::{compute_accuracy, Accuracy, CORRECT_MARGIN, MISS_MARGIN},
//         voices::Loop,
//     };

//     #[test]
//     fn it_can_load_a_loop_from_file() {
//         let result = Loop::new_from_file("res/loops/samba.json").await;
//         assert_eq!(result, (Accuracy::Correct, true));

//         let result = compute_accuracy(BEATS_PER_LOOP - 2. * CORRECT_MARGIN, &vec![0.0]);
//         assert_eq!(result, (Accuracy::Early, true));
//     }
// }
