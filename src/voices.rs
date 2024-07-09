/*
  Data structures describing the notes to be played on each instrument.
*/
use std::error::Error;

use macroquad::file::load_file;
use serde::{Deserialize, Serialize};

use crate::consts::ALL_INSTRUMENTS;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Instrument {
    ClosedHihat,
    Snare,
    Kick,
    OpenHihat,
    PedalHiHat,
    Ride,
    // RideBell,
    Tom1,
    Tom2,
    Tom3,
    Crash,
}

/// Voice represents the notes to be played on an instrument.
#[derive(Debug, Clone)]
pub struct Voice {
    instrument: Instrument,
    beat_timings: Vec<f64>,
}

impl Voice {
    pub fn new(instrument: Instrument) -> Self {
        Self {
            instrument,
            beat_timings: vec![],
        }
    }
}

/// VoicesOld represents the notes to be played on each instrument.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VoicesFromJSON {
    closed_hihat: Vec<f64>,
    snare: Vec<f64>,
    kick: Vec<f64>,
    open_hihat: Vec<f64>,
    ride: Vec<f64>,
    crash: Vec<f64>,
}

/// Voices represents the notes to be played on each instrument.
#[derive(Debug, Clone)]
pub struct Voices {
    data: Vec<Voice>,
}

impl Voices {
    pub fn new() -> Self {
        let mut data = vec![];
        for ins in ALL_INSTRUMENTS.iter() {
            data.push(Voice::new(*ins))
        }
        Self { data }
    }

    pub fn new_from_voices_old_model(vo: &VoicesFromJSON) -> Self {
        let mut data = vec![];
        for ins in ALL_INSTRUMENTS.iter() {
            let beat_timings = match ins {
                Instrument::ClosedHihat => vo.closed_hihat.clone(),
                Instrument::Snare => vo.snare.clone(),
                Instrument::Kick => vo.kick.clone(),
                Instrument::OpenHihat => vo.open_hihat.clone(),
                Instrument::Ride => vo.ride.clone(),
                Instrument::Crash => vo.crash.clone(),
                Instrument::Tom1 => vec![],
                Instrument::Tom2 => vec![],
                Instrument::Tom3 => vec![],
                Instrument::PedalHiHat => vec![],
            };
            data.push(Voice {
                instrument: *ins,
                beat_timings,
            });
        }
        Self { data }
    }

    pub fn toggle_beat(&mut self, ins: Instrument, beat: f64) {
        let ins_vec = self.get_instrument_beats_mut(&ins);
        if let Some(pos) = ins_vec.iter().position(|x| *x == beat) {
            ins_vec.remove(pos);
        } else {
            ins_vec.push(beat);
        }
    }

    pub fn get_instrument_beats(self: &Self, ins: &Instrument) -> &Vec<f64> {
        if let Some(pos) = self.data.iter().position(|x| x.instrument == *ins) {
            return &self.data[pos].beat_timings;
        } else {
            panic!("couldn't find instrument, though ALL_INSTRUMENTS should be present");
        }
    }

    pub fn get_instrument_beats_mut(self: &mut Self, ins: &Instrument) -> &mut Vec<f64> {
        if let Some(pos) = self.data.iter().position(|x| x.instrument == *ins) {
            return &mut self.data[pos].beat_timings;
        } else {
            panic!("couldn't find instrument, though ALL_INSTRUMENTS should be present");
        }
    }

    pub fn get_audio_file_for_instrument(ins: &Instrument) -> &str {
        // TODO: verify required sound files exist on startup- right now it fails during runtime
        match ins {
            Instrument::ClosedHihat => "res/sounds/closed-hihat.wav",
            Instrument::Snare => "res/sounds/snare.wav",
            Instrument::Kick => "res/sounds/kick.wav",
            Instrument::OpenHihat => "res/sounds/open-hihat.wav",
            Instrument::Ride => "res/sounds/ride.wav",
            Instrument::Crash => "res/sounds/crash.wav",
            Instrument::Tom1 => "res/sounds/tom-hi.wav",
            Instrument::Tom2 => "res/sounds/tom-med.wav",
            Instrument::Tom3 => "res/sounds/tom-low.wav",
            Instrument::PedalHiHat => "res/sounds/pedal-hihat.wav",
            // Instrument::Metronome => "res/sounds/click.wav",
        }
    }

    pub fn to_voices_old_model(&self) -> VoicesFromJSON {
        VoicesFromJSON {
            closed_hihat: self.get_instrument_beats(&Instrument::ClosedHihat).clone(),
            snare: self.get_instrument_beats(&Instrument::Snare).clone(),
            kick: self.get_instrument_beats(&Instrument::Kick).clone(),
            open_hihat: self.get_instrument_beats(&Instrument::OpenHihat).clone(),
            ride: self.get_instrument_beats(&Instrument::Ride).clone(),
            crash: self.get_instrument_beats(&Instrument::Crash).clone(),
        }
    }
}

/// Loop is the full information required to play a loop. It can be read/written to a file.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Loop {
    pub bpm: usize,
    pub voices: VoicesFromJSON,
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
    use crate::voices::{Instrument, Loop, Voices};

    #[test]
    fn it_can_load_a_loop_from_file() {
        let result = Loop::new_from_file("res/loops/samba.json");
        let loop_data = result.unwrap();
        assert_eq!(loop_data.bpm, 120);
        let voices = Voices::new_from_voices_old_model(&loop_data.voices);
        assert_eq!(
            voices.get_instrument_beats(&Instrument::ClosedHihat).len(),
            12
        );
        assert_eq!(voices.get_instrument_beats(&Instrument::Snare).len(), 7);

        assert_eq!(voices.get_instrument_beats(&Instrument::Kick).len(), 8);
        assert_eq!(voices.get_instrument_beats(&Instrument::OpenHihat).len(), 4);
        assert_eq!(voices.get_instrument_beats(&Instrument::Ride).len(), 0);
    }
}
