use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub audio_latency_seconds: f64,
}

impl AppConfig {
    pub fn new() -> Self {
        // loads or initializes
        confy::load("macroix", "AppConfig").unwrap_or_default()
    }

    pub fn save(&self) {
        // TODO: We may remove confy. Ignore for now
        #[allow(clippy::match_single_binding)]
        match confy::store("macroix", "AppConfig", self) {
            // ignore failures. these happen in web builds
            _ => (),
        }
    }
}
