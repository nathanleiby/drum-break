use macroquad::miniquad::info;

use crate::consts::BEATS_PER_LOOP;

#[derive(Debug, PartialEq)]
pub enum Accuracy {
    Correct,
    Early,
    Late,
    Miss,
}

// TODO: consider using Decimal type for exact math on beats.
// - Floating point math has comparison/equality challenges
// - Can't hash pointing point numbers out of the gate
pub const CORRECT_MARGIN: f64 = 0.1;
pub const MISS_MARGIN: f64 = 0.3;

/// returns a tuple of (accuracy rating, a bool of whether not this measurement is wrapping around to the _next_ loop)
pub fn compute_accuracy(user_beat_with_latency: f64, desired_hits: &Vec<f64>) -> (Accuracy, bool) {
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
    let mut is_next_loop = false;
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
                    is_next_loop = true;
                    target_beat = Some((desired, new_dist));
                }
            }
        }
    }

    match target_beat {
        None => {
            info!("No target beat found, returning Miss");
            return (Accuracy::Miss, false);
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
                "Accuracy: {:?} .. user_input_beat = {:?} .. target_beat = {:?} .. distance = {:?} .. is_next_loop = {:?}",
                acc, user_beat_with_latency, target_beat, distance, is_next_loop
            );
            return (acc, is_next_loop);
        }
    }
}

// pub get_aced_loops_vs_total(hits: &Vec<f64>, desired_hits: &Vec<f64>) {
//     let mut aced: usize = 0;
//     let mut total: usize = 0;

//     // // for each voice
//     // for (voice_index, desired_hits) in desired_hits.iter().enumerate() {
//     //     // for each desired hit
//     //     for desired in desired_hits.iter() {
//     //         // if there's no target_beat yet, set it to the first desired hit
//     //         match target_beat {
//     //             None => {
//     //                 target_beat = Some((*desired, user_beat_with_latency - desired));
//     //                 continue;
//     //             }
//     //             Some((_, prev_dist)) => {
//     //                 let new_dist = user_beat_with_latency - desired;
//     //                 if new_dist.abs() < prev_dist.abs() {
//     //                     target_beat = Some((*desired, new_dist));
//     //                 }
//     //             }
//     //         }
//     //     }
//     // }

//     // split user hits by loop
//     // check if loop was aced
//     (currect, total)
// }

#[cfg(test)]
mod tests {
    use std::f64::EPSILON;

    use crate::{
        consts::BEATS_PER_LOOP,
        score::{compute_accuracy, Accuracy, CORRECT_MARGIN, MISS_MARGIN},
    };
    #[test]
    fn it_computes_accuracy_against_one_note() {
        let compute_accuracy_legacy = |user_beat_with_latency: f64, desired_hits: &Vec<f64>| {
            compute_accuracy(user_beat_with_latency, desired_hits).0
        };

        // exactly correct
        let result = compute_accuracy_legacy(0.0, &vec![0.0]);
        assert_eq!(result, Accuracy::Correct);

        // within (at) the correct margin
        let result = compute_accuracy_legacy(CORRECT_MARGIN, &vec![0.0]);
        assert_eq!(result, Accuracy::Correct);

        let result = compute_accuracy_legacy(-CORRECT_MARGIN, &vec![0.0]);
        assert_eq!(result, Accuracy::Correct);

        // between the correct margin and the miss margin
        let late = CORRECT_MARGIN + (MISS_MARGIN - CORRECT_MARGIN) / 2.;
        let result = compute_accuracy_legacy(late, &vec![0.0]);
        assert_eq!(result, Accuracy::Late);

        let result = compute_accuracy_legacy(-late, &vec![0.0]);
        assert_eq!(result, Accuracy::Early);

        // exactly at the mss margin
        let almost_miss = MISS_MARGIN;
        let result = compute_accuracy_legacy(almost_miss, &vec![0.0]);
        assert_eq!(result, Accuracy::Late);

        let result = compute_accuracy_legacy(-almost_miss, &vec![0.0]);
        assert_eq!(result, Accuracy::Early);

        // beyond the miss margin
        let miss = MISS_MARGIN + EPSILON;
        let result = compute_accuracy_legacy(miss, &vec![0.0]);
        assert_eq!(result, Accuracy::Miss);

        let result = compute_accuracy_legacy(-miss, &vec![0.0]);
        assert_eq!(result, Accuracy::Miss);
    }

    #[test]
    fn it_computes_accuracy_against_correct_target_note_from_many() {
        let compute_accuracy_legacy = |user_beat_with_latency: f64, desired_hits: &Vec<f64>| {
            compute_accuracy(user_beat_with_latency, desired_hits).0
        };

        // should check if it's closer to the nearest note: 0.0, not 1.0
        let result = compute_accuracy_legacy(CORRECT_MARGIN, &vec![0.0, 1.0]);
        assert_eq!(result, Accuracy::Correct);

        // handle wrap-around case
        let result = compute_accuracy_legacy(BEATS_PER_LOOP - CORRECT_MARGIN, &vec![0.0, 1.0]);
        assert_eq!(result, Accuracy::Correct);

        let result = compute_accuracy_legacy(BEATS_PER_LOOP - 2. * CORRECT_MARGIN, &vec![0.0, 1.0]);
        assert_eq!(result, Accuracy::Early);

        let result = compute_accuracy_legacy(BEATS_PER_LOOP - MISS_MARGIN, &vec![0.0, 1.0]);
        assert_eq!(result, Accuracy::Miss);
    }

    #[test]
    fn it_computes_accuracy_considering_is_next_loop() {
        let result = compute_accuracy(BEATS_PER_LOOP - CORRECT_MARGIN, &vec![0.0]);
        assert_eq!(result, (Accuracy::Correct, true));

        let result = compute_accuracy(BEATS_PER_LOOP - 2. * CORRECT_MARGIN, &vec![0.0]);
        assert_eq!(result, (Accuracy::Early, true));
    }
}
