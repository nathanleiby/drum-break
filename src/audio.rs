use kira::{
    clock::{ClockHandle, ClockSpeed, ClockTime},
    manager::{backend::DefaultBackend, AudioManager, AudioManagerSettings},
    sound::static_sound::{StaticSoundData, StaticSoundSettings},
    tween::Tween,
};

use macroquad::prelude::*;

use crate::{
    consts::{BEATS_PER_LOOP, TICK_SCHEDULE_AHEAD},
    Voices,
};

pub struct Audio {
    manager: AudioManager<DefaultBackend>,
    clock: ClockHandle,
    last_scheduled_tick: f64,
    bpm: f64,
}

const DEFAULT_BPM: f64 = 120.;

impl Audio {
    pub fn new() -> Self {
        let mut manager =
            AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).unwrap();
        let clock = manager
            // TODO: investigate bpm * 2 stuff
            .add_clock(ClockSpeed::TicksPerMinute(DEFAULT_BPM * 2. as f64))
            .unwrap();

        Self {
            manager,
            clock,
            last_scheduled_tick: -1.,
            bpm: DEFAULT_BPM,
        }
    }

    /// schedule should be run within each game tick to schedule the audio
    pub fn schedule(self: &mut Self, voices: &Voices) {
        let current = self.current_clock_tick();
        if current <= self.last_scheduled_tick {
            return;
        }

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
                &mut self.manager,
                &self.clock,
                self.last_scheduled_tick,
                tick_to_schedule,
            );
        }

        self.last_scheduled_tick = tick_to_schedule
    }

    pub fn current_clock_tick(self: &Self) -> f64 {
        self.clock.time().ticks as f64 + self.clock.fractional_position()
    }

    pub fn get_bpm(self: &Self) -> f64 {
        self.bpm
    }

    pub fn set_bpm(self: &mut Self, bpm: f64) {
        self.bpm = bpm;
        self.clock
            .set_speed(ClockSpeed::TicksPerMinute(bpm * 2.), Tween::default())
            .unwrap();
    }

    pub fn toggle_pause(self: &Self) {
        if self.clock.ticking() {
            self.clock.pause().unwrap();
        } else {
            self.clock.start().unwrap();
        }
    }
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
