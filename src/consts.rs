//
// UI
//

use crate::voices::Instrument;

pub const WINDOW_WIDTH: i32 = 1280;
pub const WINDOW_HEIGHT: i32 = 720;

pub const DEFAULT_BEATS_PER_LOOP: usize = 16;

//
// Audio
//
// TODO: I notice that if the BPM is high in a web build (like 180) it seems like things are hiccuping. Maybe could schedule further ahead?
// pub const TICK_SCHEDULE_AHEAD: f64 = 4.; // schedule audio this many (N) ticks ahead of time (i.e. N seconds ahead if at 60bpm)
pub const TICK_SCHEDULE_AHEAD: f64 = 2.; // schedule audio this many (N) ticks ahead of time (i.e. N seconds ahead if at 60bpm)

// General use
pub const ALL_INSTRUMENTS: [Instrument; 10] = [
    Instrument::Crash,
    Instrument::Ride,
    Instrument::OpenHihat,
    Instrument::ClosedHihat,
    Instrument::Tom1,
    Instrument::Tom2,
    Instrument::Tom3,
    Instrument::Snare,
    Instrument::Kick,
    Instrument::PedalHiHat,
];

// Message passing (TODO: move to events?)

#[derive(Debug)]
pub enum TxMsg {
    AudioNew,
    StartingLoop(i32),
}

/// MOVED FROM AUDIO
#[derive(Debug, Clone)]
pub struct UserHit {
    pub instrument: Instrument,
    pub clock_tick: f64,
}

impl UserHit {
    pub fn new(instrument: Instrument, clock_tick: f64) -> Self {
        Self {
            instrument,
            clock_tick,
        }
    }

    pub fn beat(&self, beats_per_loop: usize) -> f64 {
        self.clock_tick % (beats_per_loop as f64)
    }
}
