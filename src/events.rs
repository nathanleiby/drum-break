use crate::voices::Instrument;

#[derive(Clone, Debug)]
pub enum Events {
    UserHit {
        instrument: Instrument,
        processing_delay: f64,
    },
    Pause,
    ChangeBPM {
        delta: f64,
    },
    SetBPM(f64),
    Quit,
    ResetHits,
    SaveLoop,
    ToggleBeat {
        row: f64,
        beat: f64,
    },
    TrackForCalibration,
    SetAudioLatency {
        delta: f64,
    },
    ToggleDebugMode,
    ChangeLoop(usize), // loop idx
}