use crate::{
    consts::BEATS_PER_LOOP,
    voices::{Instrument, Voices},
    UserHit,
};

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
pub fn compute_accuracy_of_single_hit(
    user_beat_with_latency: f64,
    desired_hits: &Vec<f64>,
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
            // log::info!("No target beat found, returning Miss");
            return (Accuracy::Miss, false);
        }
        Some((b, _)) => {
            // log::info!("Target beat found: {:?}", b);
            let distance = user_beat_with_latency - b;
            let acc = match distance {
                d if d.abs() > MISS_MARGIN => Accuracy::Miss,
                d if d < -CORRECT_MARGIN => Accuracy::Early,
                d if d > CORRECT_MARGIN => Accuracy::Late,
                _ => Accuracy::Correct,
            };

            // log::info!(
            //     "Accuracy: {:?} .. user_input_beat = {:?} .. target_beat = {:?} .. distance = {:?} .. is_next_loop = {:?}",
            //     acc, user_beat_with_latency, target_beat, distance, is_next_loop
            // );
            return (acc, is_next_loop);
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct ScoreTracker {
    pub num_correct: usize,
    pub num_notes: usize,
}

impl ScoreTracker {
    fn new() -> Self {
        Self {
            num_correct: 0,
            num_notes: 0,
        }
    }

    pub fn ratio(self: Self) -> f64 {
        self.num_correct as f64 / self.num_notes as f64
    }
}

pub struct LastLoopSummary {
    pub hihat: ScoreTracker,
    pub snare: ScoreTracker,
    pub kick: ScoreTracker,
    pub open_hihat: ScoreTracker,
}

impl LastLoopSummary {
    pub fn new() -> Self {
        Self {
            hihat: ScoreTracker::new(),
            snare: ScoreTracker::new(),
            kick: ScoreTracker::new(),
            open_hihat: ScoreTracker::new(),
        }
    }

    pub fn total(self: Self) -> ScoreTracker {
        let mut combined = ScoreTracker::new();

        combined.num_correct += self.hihat.num_correct;
        combined.num_notes += self.hihat.num_notes;

        combined.num_correct += self.snare.num_correct;
        combined.num_notes += self.snare.num_notes;

        combined.num_correct += self.kick.num_correct;
        combined.num_notes += self.kick.num_notes;

        combined.num_correct += self.open_hihat.num_correct;
        combined.num_notes += self.open_hihat.num_notes;

        combined
    }
}

pub fn get_user_hit_timings_by_instrument(
    user_hits: &Vec<UserHit>,
    instrument: Instrument,
) -> Vec<f64> {
    user_hits
        .iter()
        .filter(|hit| hit.instrument == instrument)
        .map(|hit| hit.beat())
        .collect::<Vec<f64>>()
}

pub fn get_desired_timings_by_instrument<'a>(
    instrument: &Instrument,
    desired_hits: &'a Voices,
) -> &'a Vec<f64> {
    let desired_timings = match instrument {
        Instrument::ClosedHihat => &desired_hits.closed_hihat,
        Instrument::Snare => &desired_hits.snare,
        Instrument::Kick => &desired_hits.kick,
        Instrument::OpenHihat => &desired_hits.open_hihat,
    };
    desired_timings
}

pub fn compute_last_loop_summary(
    user_hits: &Vec<UserHit>,
    desired_hits: &Voices,
    audio_latency: f64,
) -> LastLoopSummary {
    let mut out = LastLoopSummary::new();

    let instruments = [
        Instrument::ClosedHihat,
        Instrument::Snare,
        Instrument::Kick,
        Instrument::OpenHihat,
    ];
    for (_, instrument) in instruments.iter().enumerate() {
        // get accuracy of hihat
        let user_timings = get_user_hit_timings_by_instrument(user_hits, *instrument);
        let desired_timings = get_desired_timings_by_instrument(instrument, desired_hits);

        // compare that to desired hits for hihat
        let mut num_correct: usize = 0;
        for note in user_timings.iter() {
            let (acc, _) = compute_accuracy_of_single_hit(note + audio_latency, desired_timings);
            if acc == Accuracy::Correct {
                num_correct += 1;
            }
        }

        match instrument {
            Instrument::ClosedHihat => {
                out.hihat.num_correct = num_correct;
                out.hihat.num_notes = desired_timings.len();
            }
            Instrument::Snare => {
                out.snare.num_correct = num_correct;
                out.snare.num_notes = desired_timings.len();
            }
            Instrument::Kick => {
                out.kick.num_correct = num_correct;
                out.kick.num_notes = desired_timings.len();
            }
            Instrument::OpenHihat => {
                out.open_hihat.num_correct = num_correct;
                out.open_hihat.num_notes = desired_timings.len();
            }
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use std::f64::EPSILON;

    use log::debug;

    use crate::{
        consts::BEATS_PER_LOOP,
        score::{
            compute_accuracy_of_single_hit, compute_last_loop_summary, Accuracy, ScoreTracker,
            CORRECT_MARGIN, MISS_MARGIN,
        },
        voices::Instrument,
        UserHit,
    };

    //
    // compute_accuracy_of_single_hit
    //

    #[test]
    fn it_computes_accuracy_against_one_note() {
        let compute_accuracy_legacy = |user_beat_with_latency: f64, desired_hits: &Vec<f64>| {
            compute_accuracy_of_single_hit(user_beat_with_latency, desired_hits).0
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
            compute_accuracy_of_single_hit(user_beat_with_latency, desired_hits).0
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
        let result = compute_accuracy_of_single_hit(BEATS_PER_LOOP - CORRECT_MARGIN, &vec![0.0]);
        assert_eq!(result, (Accuracy::Correct, true));

        let result =
            compute_accuracy_of_single_hit(BEATS_PER_LOOP - 2. * CORRECT_MARGIN, &vec![0.0]);
        assert_eq!(result, (Accuracy::Early, true));
    }

    //
    // compute_last_loop_summary
    //

    #[test]
    fn it_computes_last_loop_summary_for_correct_user_htis() {
        let user_hits = vec![UserHit::new(Instrument::Kick, 0.0)];
        let desired_hits = crate::voices::Voices {
            closed_hihat: vec![],
            snare: vec![],
            kick: vec![0.0],
            open_hihat: vec![],
        };

        let result = compute_last_loop_summary(&user_hits, &desired_hits, 0.0);
        assert_eq!(
            result.kick,
            ScoreTracker {
                num_correct: 1,
                num_notes: 1,
            }
        );
    }

    #[test]
    fn it_computes_last_loop_summary_for_incorrect_user_hits() {
        let user_hits = vec![UserHit::new(Instrument::Kick, 0.5)];
        let desired_hits = crate::voices::Voices {
            closed_hihat: vec![],
            snare: vec![],
            kick: vec![0.0],
            open_hihat: vec![],
        };
        let result = compute_last_loop_summary(&user_hits, &desired_hits, 0.0);
        assert_eq!(
            result.kick,
            ScoreTracker {
                num_correct: 0,
                num_notes: 1,
            }
        );
    }
}
