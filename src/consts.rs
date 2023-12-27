pub const NUM_ROWS_IN_GRID: f64 = 4.;

pub const BEATS_PER_LOOP: f64 = 16.;

// UI only?
pub const BEAT_WIDTH_PX: f64 = 64.0;
pub const BEAT_PADDING: f64 = 4.;

pub const GRID_WIDTH: f64 = BEAT_WIDTH_PX * 16.;
pub const ROW_HEIGHT: f64 = BEAT_WIDTH_PX;

pub const GRID_LEFT_X: f64 = 128.;
pub const GRID_TOP_Y: f64 = 64.;

pub const TICK_SCHEDULE_AHEAD: f64 = 2.; // schedule audio this many (N) ticks ahead of time (i.e. N seconds ahead if at 60bpm)
