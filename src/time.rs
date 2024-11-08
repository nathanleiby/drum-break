/*
 Helper functions for time.

 Notably, we have several kinds of time: Audio manager, Macroquad, and Rust time.
*/

use web_time::SystemTime;

// get curent time in milliseconds
pub fn current_time_millis() -> u128 {
    SystemTime::now()
        .duration_since(web_time::UNIX_EPOCH)
        .unwrap()
        .as_millis()
}
