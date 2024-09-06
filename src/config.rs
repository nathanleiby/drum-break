use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub audio_latency_seconds: f64,
}

impl AppConfig {
    pub fn new() -> Self {
        // loads or initializes
        match confy::load("macroix", "AppConfig") {
            Ok(cfg) => cfg,
            Err(_) => AppConfig::default(),
        }
    }

    pub fn save(&self) {
        match confy::store("macroix", "AppConfig", self) {
            // ignore failures. these happen in web builds
            _ => (),
        }
    }
}
