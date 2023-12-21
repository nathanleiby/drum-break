use macroquad::miniquad::info;

use crate::consts::BEATS_PER_LOOP;

#[derive(Debug)]
pub enum Accuracy {
    Correct,
    Early,
    Late,
    Miss,
}

// General approach:
// - for each voice
// - maintain a reference to the next target_beat
//      - if the user provides input, compute the accuracy
// - if it's a hit (non-Miss), register it and update the target_beat
// - it it's a miss, register it but DO NOT update the target_beat

pub const CORRECT_MARGIN: f64 = 0.1;
pub const MISS_MARGIN: f64 = 0.2;

pub struct Score {
    // input_by_voice: Vec<Vec<f64>>,
    // target_beat_per_voice: Vec<f64>,
    // hits_by_voice: Vec<HashMap<f64, Accuracy>>,
    // misses_by_voice: Vec<HashMap<f64, Accuracy>>,
}

impl Score {
    pub fn new(voices: usize) -> Self {
        // let target_beat_per_voice = Vec::<f64>::with_capacity(voices);

        // let mut hits_by_voice = Vec::<HashMap<f64, Accuracy>>::with_capacity(voices);
        // for _ in 0..voices {
        //     hits_by_voice.push(HashMap::<f64, Accuracy>::new());
        // }

        // let mut misses_by_voice = Vec::<HashMap<f64, Accuracy>>::with_capacity(voices);
        // for _ in 0..voices {
        //     misses_by_voice.push(HashMap::<f64, Accuracy>::new());
        // }

        // Self {
        //     target_beat_per_voice,
        //     hits_by_voice,
        //     misses_by_voice,
        // }
        Self {}
    }

    pub fn register_hit(&mut self, voice_index: usize, beat: f64) {
        // let target_beat = self.target_beat_per_voice[voice_index];
        // let accuracy = compute_accuracy(target_beat, beat);
        // self.hits_by_voice[voice_index].insert(beat, accuracy);
        // self.target_beat_per_voice[voice_index] += 1.0;
    }

    pub fn register_miss(&mut self, voice_index: usize, beat: f64) {
        // let accuracy = Accuracy::Miss;
        // self.misses_by_voice[voice_index].insert(beat, accuracy);
    }
}

pub fn compute_accuracy(user_beat_with_latency: f64, desired_hits: &Vec<f64>) -> Accuracy {
    // find the nearest desired_hit
    let mut target_beat = -MISS_MARGIN - 1.; // always a miss
    for desired in desired_hits.iter() {
        let prev_dist = (target_beat - desired).abs();
        let dist = (user_beat_with_latency - desired).abs();
        if dist < prev_dist {
            target_beat = *desired;
        }
    }

    // handle end of loop wrap-around case
    if desired_hits.contains(&0.) {
        let desired = 0. + BEATS_PER_LOOP;
        let prev_dist = (target_beat - desired).abs();
        let dist = (user_beat_with_latency - desired).abs();
        if dist < prev_dist {
            target_beat = desired;
        }
    }

    let distance = user_beat_with_latency - target_beat;
    let acc = match distance {
        d if d.abs() > MISS_MARGIN => Accuracy::Miss,
        d if d < -CORRECT_MARGIN => Accuracy::Early,
        d if d > CORRECT_MARGIN => Accuracy::Late,
        _ => Accuracy::Correct,
    };
    info!(
        "Accuracy: {:?} .. user_input_beat = {:?} .. target_beat = {:?}",
        acc, user_beat_with_latency, target_beat
    );
    acc
}
