use kira::{
    clock::{ClockHandle, ClockTime},
    manager::AudioManager,
    sound::static_sound::{StaticSoundData, StaticSoundSettings},
};
use macroquad::prelude::*;

use crate::{
    consts::{BEATS_PER_LOOP, TICK_SCHEDULE_AHEAD},
    current_clock_tick, Voices,
};

pub fn audio(
    voices: &Voices,
    mut manager: &mut AudioManager,
    clock: &ClockHandle,
    last_scheduled_tick: f64,
) -> f64 {
    let current = current_clock_tick(clock);
    if current > last_scheduled_tick {
        let tick_to_schedule = current + TICK_SCHEDULE_AHEAD;

        for pair in [
            (&voices.metronome, "click"),
            (&voices.closed_hihat, "closed-hihat"),
            (&voices.snare, "snare"),
            (&voices.kick, "kick"),
            (&voices.open_hihat, "open-hihat"),
        ] {
            let (voice, instrument_name) = pair;
            schedule_audio(
                &voice,
                &instrument_name,
                &mut manager,
                &clock,
                last_scheduled_tick,
                tick_to_schedule,
            );
        }

        return tick_to_schedule;
    }

    last_scheduled_tick
}

fn schedule_audio(
    notes: &Vec<f64>,
    instrument_name: &str,
    manager: &mut AudioManager,
    clock: &ClockHandle,
    last_scheduled_tick: f64,
    tick_to_schedule: f64,
) {
    let prev_beat = last_scheduled_tick % BEATS_PER_LOOP;
    let next_beat = tick_to_schedule % BEATS_PER_LOOP;
    debug!(
        "Scheduling {} from {} to {}",
        instrument_name, prev_beat, next_beat
    );
    let loop_num = (last_scheduled_tick / BEATS_PER_LOOP) as i32; // floor
    for note in notes.iter() {
        if note > &prev_beat && note <= &next_beat {
            schedule_note(note, loop_num, clock, manager, instrument_name);
        };

        // handle wrap-around case
        if next_beat < prev_beat {
            // from prev_beat to end of loop
            if *note > prev_beat && *note <= BEATS_PER_LOOP as f64 {
                schedule_note(note, loop_num, clock, manager, instrument_name);
            }
            // from start of loop to next beat
            if *note >= 0. && *note <= next_beat {
                schedule_note(note, loop_num + 1, clock, manager, instrument_name);
            }
        }
    }
}

fn schedule_note(
    note: &f64,
    loop_num: i32,
    clock: &ClockHandle,
    manager: &mut AudioManager,
    instrument_name: &str,
) {
    let note_tick = (*note + (loop_num as f64) * BEATS_PER_LOOP) as u64;
    debug!(
        "\tScheduling {} ({}) at {}",
        instrument_name, note, note_tick
    );
    let sound = StaticSoundData::from_file(
        format!("res/{}.wav", instrument_name),
        StaticSoundSettings::new().start_time(ClockTime {
            clock: clock.id(),
            ticks: note_tick,
        }),
    )
    .unwrap();
    manager.play(sound).unwrap();
}
