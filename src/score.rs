use macroquad::miniquad::info;

use crate::consts::BEATS_PER_LOOP;

#[derive(Debug, PartialEq)]
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

// TODO: consider using Decimal type for exact math on beats.
// - Floating point math has comparison/equality challenges
// - Can't hash pointing point numbers out of the gate
pub const CORRECT_MARGIN: f64 = 0.1;
pub const MISS_MARGIN: f64 = 0.3;

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
    let mut target_beat = None; // should always be a miss
    for desired in desired_hits.iter() {
        // if there's no target_beat yet, set it to the first desired hit
        match target_beat {
            None => {
                target_beat = Some((*desired, user_beat_with_latency - desired));
                continue;
            }
            Some((_, prev_dist)) => {
                let new_dist = user_beat_with_latency - desired;
                if new_dist.abs() < prev_dist.abs() {
                    target_beat = Some((*desired, new_dist));
                }
            }
        }
    }

    // handle end of loop wrap-around case
    if desired_hits.contains(&0.) {
        let desired = 0. + BEATS_PER_LOOP;
        // if there's no target_beat yet, set it to the first desired hit
        match target_beat {
            None => {
                target_beat = Some((desired, user_beat_with_latency - desired));
            }
            Some((_, prev_dist)) => {
                let new_dist = user_beat_with_latency - desired;
                if new_dist.abs() < prev_dist.abs() {
                    target_beat = Some((desired, new_dist));
                }
            }
        }
    }

    match target_beat {
        None => {
            info!("No target beat found, returning Miss");
            return Accuracy::Miss;
        }
        Some((b, _)) => {
            info!("Target beat found: {:?}", b);
            let distance = user_beat_with_latency - b;
            let acc = match distance {
                d if d.abs() > MISS_MARGIN => Accuracy::Miss,
                d if d < -CORRECT_MARGIN => Accuracy::Early,
                d if d > CORRECT_MARGIN => Accuracy::Late,
                _ => Accuracy::Correct,
            };

            info!(
                "Accuracy: {:?} .. user_input_beat = {:?} .. target_beat = {:?} .. distance = {:?}",
                acc, user_beat_with_latency, target_beat, distance
            );
            acc
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f64::EPSILON;

    use crate::{
        consts::BEATS_PER_LOOP,
        score::{compute_accuracy, Accuracy, CORRECT_MARGIN, MISS_MARGIN},
    };

    #[test]
    fn it_computes_accuracy_against_one_note() {
        // exactly correct
        let result = compute_accuracy(0.0, &vec![0.0]);
        assert_eq!(result, Accuracy::Correct);

        // within (at) the correct margin
        let result = compute_accuracy(CORRECT_MARGIN, &vec![0.0]);
        assert_eq!(result, Accuracy::Correct);

        let result = compute_accuracy(-CORRECT_MARGIN, &vec![0.0]);
        assert_eq!(result, Accuracy::Correct);

        // between the correct margin and the miss margin
        let late = CORRECT_MARGIN + (MISS_MARGIN - CORRECT_MARGIN) / 2.;
        let result = compute_accuracy(late, &vec![0.0]);
        assert_eq!(result, Accuracy::Late);

        let result = compute_accuracy(-late, &vec![0.0]);
        assert_eq!(result, Accuracy::Early);

        // exactly at the mss margin
        let almost_miss = MISS_MARGIN;
        let result = compute_accuracy(almost_miss, &vec![0.0]);
        assert_eq!(result, Accuracy::Late);

        let result = compute_accuracy(-almost_miss, &vec![0.0]);
        assert_eq!(result, Accuracy::Early);

        // beyond the miss margin
        let miss = MISS_MARGIN + EPSILON;
        let result = compute_accuracy(miss, &vec![0.0]);
        assert_eq!(result, Accuracy::Miss);

        let result = compute_accuracy(-miss, &vec![0.0]);
        assert_eq!(result, Accuracy::Miss);
    }

    #[test]
    fn it_computes_accuracy_against_correct_target_note_from_many() {
        // should check if it's closer to the nearest note: 0.0, not 1.0
        let result = compute_accuracy(CORRECT_MARGIN, &vec![0.0, 1.0]);
        assert_eq!(result, Accuracy::Correct);

        // handle wrap-around case
        let result = compute_accuracy(BEATS_PER_LOOP - CORRECT_MARGIN, &vec![0.0, 1.0]);
        assert_eq!(result, Accuracy::Correct);

        let result = compute_accuracy(BEATS_PER_LOOP - 2. * CORRECT_MARGIN, &vec![0.0, 1.0]);
        assert_eq!(result, Accuracy::Early);

        let result = compute_accuracy(BEATS_PER_LOOP - MISS_MARGIN, &vec![0.0, 1.0]);
        assert_eq!(result, Accuracy::Miss);
    }
}