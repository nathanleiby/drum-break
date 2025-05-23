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
        ins: Instrument,
        beat: f64,
    },
    TrackForCalibration,
    SetAudioLatency {
        delta_s: f64,
    },
    ToggleMetronome,
    ChangeLoop(usize), // loop idx

    ToggleHelpVisibility,
    ToggleEmptyTrackVisibility,
    ToggleSidePanelVisibility,

    RefreshConnectedMidiDevice,

    // Dev Tools
    ToggleDebugMode,
    ToggleDevToolsVisibility,
    SetCorrectMargin(f64),
    SetMissMargin(f64),
}
