use std::{collections::HashSet, error::Error};

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

pub struct InputConfigMidi {
    pub kick: HashSet<u8>,
    pub snare: HashSet<u8>,
    pub closed_hi_hat: HashSet<u8>,
    pub open_hi_hat: HashSet<u8>,
    pub ride: HashSet<u8>,
}
