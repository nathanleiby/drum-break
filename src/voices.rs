use std::error::Error;

use macroquad::file::load_file;
use serde::{Deserialize, Serialize};

pub enum Instrument {
    ClosedHihat,
    Snare,
    Kick,
    OpenHihat,
}

/// Voices represents the notes to be played on each instrument.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Voices {
    pub metronome: Vec<f64>,
    pub closed_hihat: Vec<f64>,
    pub snare: Vec<f64>,
    pub kick: Vec<f64>,
    pub open_hihat: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Loop {
    bpm: usize,
    voices: Voices,
}

impl Voices {
    pub fn new() -> Self {
        Self {
            metronome: vec![],
            closed_hihat: vec![],
            snare: vec![],
            kick: vec![],
            open_hihat: vec![],
        }
    }

    pub async fn new_from_file(path: &str) -> Result<Self, Box<dyn Error>> {
        let f = load_file(path).await?;
        let out: Self = serde_json::from_reader(&*f)?;
        Ok(out)
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
