//
// UI
//

use crate::voices::Instrument;

pub const WINDOW_WIDTH: i32 = 1280;
pub const WINDOW_HEIGHT: i32 = 720;

pub const BEATS_PER_LOOP: f64 = 16.;

pub const BEAT_WIDTH_PX: f64 = 48.0;
pub const BEAT_PADDING: f64 = 4.;

pub const GRID_WIDTH: f64 = BEAT_WIDTH_PX * 16.;
pub const ROW_HEIGHT: f64 = BEAT_WIDTH_PX;

pub const GRID_LEFT_X: f64 = 128.;
pub const GRID_TOP_Y: f64 = 64.;

//
// Audio
//
pub const TICK_SCHEDULE_AHEAD: f64 = 2.; // schedule audio this many (N) ticks ahead of time (i.e. N seconds ahead if at 60bpm)

// General use
pub const ALL_INSTRUMENTS: [Instrument; 9] = [
    Instrument::Crash,
    Instrument::Ride,
    Instrument::OpenHihat,
    Instrument::ClosedHihat,
    Instrument::Tom1,
    Instrument::Tom2,
    Instrument::Tom3,
    Instrument::Snare,
    Instrument::Kick,
    // Pedal Hi-hat
];

pub const NUM_ROWS_IN_GRID: usize = ALL_INSTRUMENTS.len();
