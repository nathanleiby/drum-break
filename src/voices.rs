/*
  Data structures describing the notes to be played on each instrument.
*/
use std::error::Error;

use macroquad::file::load_file;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Instrument {
    ClosedHihat,
    Snare,
    Kick,
    OpenHihat,
    // PedalHiHat,
    Ride,
    // RideBell,
    // LTom,
    // MTom,
    // HTom,
    // Crash,
}

/// Voice represents the notes to be played on an instrument.
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
    pub ride: Vec<f64>,
}

impl Voices {
    pub fn new() -> Self {
        Self {
            closed_hihat: vec![],
            snare: vec![],
            kick: vec![],
            open_hihat: vec![],
            ride: vec![],
        }
    }

    pub fn toggle_beat(&mut self, row: f64, beat: f64) {
        let ins_vec = match row as usize {
            0 => &mut self.closed_hihat,
            1 => &mut self.snare,
            2 => &mut self.kick,
            3 => &mut self.open_hihat,
            4 => &mut self.ride,
            _ => panic!("invalid instrument idx"),
        };
        if let Some(pos) = ins_vec.iter().position(|x| *x == beat) {
            ins_vec.remove(pos);
        } else {
            ins_vec.push(beat);
        }
    }
}

/// Loop is the full information required to play a loop. It can be read/written to a file.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Loop {
    pub bpm: usize,
    pub voices: Voices,
}

impl Loop {
    pub async fn new_from_file_async(path: &str) -> Result<Self, Box<dyn Error>> {
        log::info!("Loop::new_from_file .. {}", path);
        let f = load_file(path).await?;
        let out: Self = serde_json::from_reader(&*f)?;
        Ok(out)
    }

    pub fn new_from_file(path: &str) -> Result<Self, Box<dyn Error>> {
        // load file at path
        let f = std::fs::File::open(path)?;
        let out: Self = serde_json::from_reader(f)?;
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use crate::voices::Loop;

    #[test]
    fn it_can_load_a_loop_from_file() {
        let result = Loop::new_from_file("res/loops/samba.json");
        let loop_data = result.unwrap();
        assert_eq!(loop_data.bpm, 120);
        assert_eq!(loop_data.voices.closed_hihat.len(), 12);
        assert_eq!(loop_data.voices.snare.len(), 7);
        assert_eq!(loop_data.voices.kick.len(), 8);
        assert_eq!(loop_data.voices.open_hihat.len(), 4);
    }
}
