/*
  Computes score by comparing timings of user's hits vs desired hits.
*/

use std::{collections::HashMap, vec};

use crate::{
    consts::UserHit,
    consts::ALL_INSTRUMENTS,
    voices::{Instrument, Voices},
};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Accuracy {
    Correct,
    Early,
    Late,
    Miss,
    Unknown,
}

// TODO: consider using Decimal type for exact math on beats.
// - Floating point math has comparison/equality challenges
// - Can't hash floating point numbers out of the gate

// TODO: Can I use cvars to tweak these vals at runtime?
pub const CORRECT_MARGIN: f64 = 0.151; // TODO: hacky fix 0.15 -> 0.151 due to floating point comparison. let's try Decimal later
pub const MISS_MARGIN: f64 = 0.3;

/// returns a tuple of (accuracy rating, a bool of whether not this measurement is wrapping around to the _next_ loop)
pub fn compute_accuracy_of_single_hit(
    user_beat_with_latency: f64,
    desired_hits: &[f64],
    beats_per_loop: usize,
) -> (Accuracy, bool) {
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
        let desired = 0. + beats_per_loop as f64;
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
        None => (Accuracy::Miss, false),
        Some((b, _)) => {
            log::debug!("Target beat found: {:?}", b);
            let distance = user_beat_with_latency - b;
            let acc = match distance {
                d if d.abs() > MISS_MARGIN => Accuracy::Miss,
                d if d < -CORRECT_MARGIN => Accuracy::Early,
                d if d > CORRECT_MARGIN => Accuracy::Late,
                _ => Accuracy::Correct,
            };

            (acc, is_next_loop)
        }
    }
}

#[derive(Debug)]
pub struct Accuracies {
    pub accuracies: Vec<Accuracy>,
}

impl Accuracies {
    fn new() -> Self {
        Self { accuracies: vec![] }
    }

    // score is given as a ratio, from 0 to 1
    pub fn score(&self) -> f64 {
        let num_correct = self
            .accuracies
            .iter()
            .map(|acc| *acc == Accuracy::Correct)
            .filter(|b| *b)
            .count();
        let num_close = self
            .accuracies
            .iter()
            .map(|acc| *acc == Accuracy::Early || *acc == Accuracy::Late)
            .filter(|b| *b)
            .count();

        let num_notes = self.accuracies.len();

        // Consider near-hits as partial success instead of ONLY correct
        (1. * num_correct as f64 + 0.5 * num_close as f64) / num_notes as f64
    }
}

#[derive(Debug)]
pub struct LastLoopSummary {
    data: HashMap<Instrument, Accuracies>,
}

impl LastLoopSummary {
    pub fn new() -> Self {
        let mut data = HashMap::new();
        for ins in ALL_INSTRUMENTS.iter() {
            data.insert(*ins, Accuracies::new());
        }

        Self { data }
    }

    pub fn get_score_tracker(&self, instrument: &Instrument) -> &Accuracies {
        let st = self.data.get(instrument);
        if let Some(st) = st {
            st
        } else {
            panic!("invalid -- Accuracies should be defined for all instruments at startup")
        }
    }

    pub fn set_score_tracker(&mut self, instrument: &Instrument, score_tracker: Accuracies) {
        let to_update: &mut Accuracies = self.get_mut_score_tracker(instrument);
        *to_update = score_tracker;
    }

    fn get_mut_score_tracker(&mut self, instrument: &Instrument) -> &mut Accuracies {
        let st = self.data.get_mut(instrument);
        if let Some(st) = st {
            st
        } else {
            panic!("invalid -- Accuracies should be defined for all instruments at startup")
        }
    }

    pub fn combined(self) -> Accuracies {
        let mut all_acc = vec![];

        for ins in ALL_INSTRUMENTS.iter() {
            let st = self.get_score_tracker(ins);
            for acc in &st.accuracies {
                all_acc.push(*acc);
            }
        }

        Accuracies {
            accuracies: all_acc,
        }
    }
}

pub fn get_user_hit_timings_by_instrument(
    user_hits: &[UserHit],
    instrument: Instrument,
    beats_per_loop: usize,
) -> Vec<f64> {
    user_hits
        .iter()
        .filter(|hit| hit.instrument == instrument)
        .map(|hit| hit.beat(beats_per_loop))
        .collect::<Vec<f64>>()
}

/// given timings for desired hits vs user hits, gives an accuracy for each desired hit
/// the accuracy is based on the first user hit that's within "non miss" range of a desired hit
/// TODO: This system doesn't work if beats are closer together than MISS_MARGIN (perhaps: 32nd notes?)
pub fn compute_loop_performance_for_voice(
    user_hits: &Vec<f64>,
    desired_hits: &Vec<f64>,
    loop_current_beat: f64,
    beats_per_loop: usize,
    // TODO: consider audio_latency
) -> Vec<Accuracy> {
    let mut out = Vec::new();

    // compare that to desired hits for hihat
    for desired_hit in desired_hits {
        if *desired_hit > loop_current_beat {
            out.push(Accuracy::Unknown);
            continue;
        }

        // find the first user hit that a non-miss
        let mut was_miss = true;
        for user_hit in user_hits {
            let (acc, _) =
                compute_accuracy_of_single_hit(*user_hit, &[*desired_hit], beats_per_loop);
            if acc != Accuracy::Miss {
                was_miss = false;
                out.push(acc);
                break;
            }
        }
        if was_miss {
            out.push(Accuracy::Miss);
        }
    }

    out
}

pub fn compute_last_loop_summary(
    user_hits: &[UserHit],
    desired_hits: &Voices,
    beats_per_loop: usize,
) -> LastLoopSummary {
    let mut out = LastLoopSummary::new();

    for instrument in ALL_INSTRUMENTS.iter() {
        // get accuracy of hihat
        let user_timings =
            get_user_hit_timings_by_instrument(user_hits, *instrument, beats_per_loop);
        let desired_timings = desired_hits.get_instrument_beats(instrument);

        let accuracies = compute_loop_performance_for_voice(
            &user_timings,
            desired_timings,
            beats_per_loop as f64, // "current beat" is the end of the loop
            beats_per_loop,
        );

        out.set_score_tracker(instrument, Accuracies { accuracies });
    }

    out
}

pub fn get_hits_from_nth_loop(
    user_hits: &[UserHit],
    desired_loop_idx: usize,
    beats_per_loop: usize,
) -> Vec<UserHit> {
    let last_loop_hits: Vec<UserHit> = user_hits
        .iter()
        .filter(|hit| {
            // include hits from just before start of loop (back to 0 - MISS), since those could be early or on-time hits
            let loop_num_for_hit =
                ((hit.clock_tick + MISS_MARGIN) / beats_per_loop as f64) as usize;
            loop_num_for_hit == desired_loop_idx
        })
        .cloned()
        .collect::<Vec<UserHit>>();
    last_loop_hits
}

#[cfg(test)]
mod tests {
    use crate::{
        consts::{UserHit, DEFAULT_BEATS_PER_LOOP},
        score::{
            compute_accuracy_of_single_hit, compute_last_loop_summary, Accuracy, CORRECT_MARGIN,
            MISS_MARGIN,
        },
        voices::{Instrument, Voices},
    };

    use super::compute_loop_performance_for_voice;

    //
    // compute_accuracy_of_single_hit
    //

    #[test]
    fn it_computes_accuracy_against_one_note() {
        let compute_accuracy_legacy = |user_beat_with_latency: f64, desired_hits: &Vec<f64>| {
            compute_accuracy_of_single_hit(
                user_beat_with_latency,
                desired_hits,
                DEFAULT_BEATS_PER_LOOP,
            )
            .0
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
        let miss = MISS_MARGIN + f64::EPSILON;
        let result = compute_accuracy_legacy(miss, &vec![0.0]);
        assert_eq!(result, Accuracy::Miss);

        let result = compute_accuracy_legacy(-miss, &vec![0.0]);
        assert_eq!(result, Accuracy::Miss);
    }

    #[test]
    fn it_computes_accuracy_against_correct_target_note_from_many() {
        let compute_accuracy_legacy = |user_beat_with_latency: f64, desired_hits: &Vec<f64>| {
            compute_accuracy_of_single_hit(
                user_beat_with_latency,
                desired_hits,
                DEFAULT_BEATS_PER_LOOP,
            )
            .0
        };

        let beats_per_loop = DEFAULT_BEATS_PER_LOOP as f64;
        // should check if it's closer to the nearest note: 0.0, not 1.0
        let result = compute_accuracy_legacy(CORRECT_MARGIN, &vec![0.0, 1.0]);
        assert_eq!(result, Accuracy::Correct);

        // handle wrap-around case
        let result = compute_accuracy_legacy(beats_per_loop - CORRECT_MARGIN, &vec![0.0, 1.0]);
        assert_eq!(result, Accuracy::Correct);

        let result = compute_accuracy_legacy(
            beats_per_loop - CORRECT_MARGIN - f64::EPSILON * 5.,
            &vec![0.0, 1.0],
        );
        assert_eq!(result, Accuracy::Early);

        let result = compute_accuracy_legacy(beats_per_loop - MISS_MARGIN, &vec![0.0, 1.0]);
        assert_eq!(result, Accuracy::Miss);
    }

    #[test]
    fn it_computes_accuracy_considering_is_next_loop() {
        let beats_per_loop = DEFAULT_BEATS_PER_LOOP as f64;
        let result = compute_accuracy_of_single_hit(
            beats_per_loop - CORRECT_MARGIN,
            &[0.0],
            DEFAULT_BEATS_PER_LOOP,
        );
        assert_eq!(result, (Accuracy::Correct, true));

        let result = compute_accuracy_of_single_hit(
            beats_per_loop - CORRECT_MARGIN - f64::EPSILON * 5.,
            &[0.0],
            DEFAULT_BEATS_PER_LOOP,
        );
        assert_eq!(result, (Accuracy::Early, true));
    }

    //
    // compute_last_loop_summary
    //

    #[test]
    fn it_computes_last_loop_summary_for_correct_user_htis() {
        let user_hits = vec![UserHit::new(Instrument::Kick, 0.0)];
        let mut desired_hits = Voices::new();
        desired_hits.toggle_beat(Instrument::Kick, 0.0);

        let result = compute_last_loop_summary(&user_hits, &desired_hits, DEFAULT_BEATS_PER_LOOP);
        assert_eq!(
            result.get_score_tracker(&Instrument::Kick).accuracies,
            vec![Accuracy::Correct],
        );
    }

    #[test]
    fn it_computes_last_loop_summary_for_incorrect_user_hits() {
        let user_hits = vec![UserHit::new(Instrument::Kick, 0.5)];
        let mut desired_hits = Voices::new();
        desired_hits.toggle_beat(Instrument::Kick, 0.0);

        let result = compute_last_loop_summary(&user_hits, &desired_hits, DEFAULT_BEATS_PER_LOOP);
        assert_eq!(
            result.get_score_tracker(&Instrument::Kick).accuracies,
            vec![Accuracy::Miss],
        );
    }

    #[test]
    fn it_computes_loop_performance_for_voice() {
        let user_hits = vec![0.5, 0.6, 0.8];
        let desired_hits = vec![0.0, 0.5, 1.0];
        let loop_current_beat = 4.;
        let result = compute_loop_performance_for_voice(
            &user_hits,
            &desired_hits,
            loop_current_beat,
            DEFAULT_BEATS_PER_LOOP,
        );
        assert_eq!(
            result,
            vec![Accuracy::Miss, Accuracy::Correct, Accuracy::Early]
        );
    }
}
