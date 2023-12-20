use std::error::Error;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub audio_latency_seconds: f64,
}

impl AppConfig {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        // loads or initializes
        let cfg: AppConfig = confy::load("macroix", "AppConfig")?;
        Ok(cfg)
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        confy::store("macroix", "AppConfig", self)?;
        Ok(())
    }
}

// TODO: support user configurable input mapping
// have defaults for various devices (perhaps Midi standards)
// allow multiple input values for same hit (e.g. if you want various triggers on a midi drum, like a fancy cymbal with regions, to trigger the same logical hit)
// impl InputMapping {
// Midi
// Keyboard
// etc

// snareHit: 48,
// }
