/*
    Generic FPS tracker for Macroquad. For debugging only.
*/

use std::collections::VecDeque;

use macroquad::prelude::*;

pub struct Fps {
    fps_tracker: VecDeque<i32>,
    last_fps: i32,
    last_updated_fps_timestamp: f64,
}

impl Fps {
    pub fn new() -> Self {
        Self {
            fps_tracker: VecDeque::<i32>::with_capacity(10),
            last_fps: 0,
            last_updated_fps_timestamp: get_time(),
        }
    }

    pub fn update(&mut self) {
        // debug
        self.fps_tracker.push_front(get_fps());
        if self.fps_tracker.len() > 10 {
            self.fps_tracker.pop_back();
        }
        let avg_fps: i32 = self.fps_tracker.iter().sum::<i32>() / self.fps_tracker.len() as i32;
        if get_time() - self.last_updated_fps_timestamp > 1. {
            self.last_updated_fps_timestamp = get_time();
            self.last_fps = avg_fps;
        }
    }

    pub fn render(&self) {
        draw_text(
            format!("FPS: {}", self.last_fps).as_str(),
            8.,
            24.,
            32.,
            GRAY,
        );
    }
}
