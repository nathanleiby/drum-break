use std::{collections::VecDeque, error::Error, io::Cursor};

use kira::{
    clock::{ClockHandle, ClockSpeed, ClockTime},
    manager::{backend::DefaultBackend, AudioManager, AudioManagerSettings},
    sound::static_sound::{StaticSoundData, StaticSoundSettings},
    tween::Tween,
};

use macroquad::prelude::*;

use crate::{
    config::AppConfig,
    consts::{BEATS_PER_LOOP, TICK_SCHEDULE_AHEAD},
    voices::Instrument,
    Voices,
};

pub struct Audio {
    manager: AudioManager<DefaultBackend>,
    clock: ClockHandle,
    last_scheduled_tick: f64,
    bpm: f64,

    pub user_hits: Voices,
    calibration_input: VecDeque<f64>,
    configured_audio_latency_seconds: f64,

    // debug only
    last_beat: i32,
}

// const DEFAULT_BPM: f64 = 120.;
const DEFAULT_BPM: f64 = 60.;

impl Audio {
    pub fn new(conf: &AppConfig) -> Self {
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

            user_hits: Voices::new(),
            calibration_input: VecDeque::new(),
            configured_audio_latency_seconds: conf.audio_latency_seconds,

            last_beat: -1,
        }
    }

    // audio latency
    pub fn get_configured_audio_latency_seconds(self: &Self) -> f64 {
        self.configured_audio_latency_seconds
    }

    pub fn set_configured_audio_latency_seconds(self: &mut Self, latency: f64) {
        self.configured_audio_latency_seconds = latency;
    }

    fn print_if_new_beat(self: &mut Self) {
        // For debugging, print when we pass an integer beat
        let current_beat = self.current_beat();
        if (current_beat as i32) > self.last_beat {
            debug!("Beat: {}", current_beat as i32);
            self.last_beat = current_beat as i32;
        }
    }

    /// schedule should be run within each game tick to schedule the audio
    pub async fn schedule(self: &mut Self, voices: &Voices) -> Result<(), Box<dyn Error>> {
        self.print_if_new_beat();

        let current = self.current_clock_tick();
        if current <= self.last_scheduled_tick {
            return Ok(());
        }

        let tick_to_schedule = current + TICK_SCHEDULE_AHEAD;

        debug!(
            "Scheduling from {} to {}",
            self.last_scheduled_tick, tick_to_schedule
        );
        for pair in [
            (&voices.metronome, "res/sounds/click.wav"),
            (&voices.closed_hihat, "res/sounds/closed-hihat.wav"),
            (&voices.snare, "res/sounds/snare.wav"),
            (&voices.kick, "res/sounds/kick.wav"),
            (&voices.open_hihat, "res/sounds/open-hihat.wav"),
        ] {
            let (voice, instrument_name) = pair;
            schedule_audio(
                &voice,
                &instrument_name,
                &mut self.manager,
                &self.clock,
                self.last_scheduled_tick,
                tick_to_schedule,
            )
            .await?;
        }

        self.last_scheduled_tick = tick_to_schedule;

        Ok(())
    }

    fn current_clock_tick(self: &Self) -> f64 {
        self.clock.time().ticks as f64 + self.clock.fractional_position()
    }

    pub fn current_beat(self: &Self) -> f64 {
        self.current_clock_tick() % BEATS_PER_LOOP
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

    pub fn track_user_hit(self: &mut Self, instrument: Instrument) {
        match instrument {
            Instrument::ClosedHihat => self.user_hits.closed_hihat.push(self.current_beat()),
            Instrument::Snare => self.user_hits.snare.push(self.current_beat()),
            Instrument::Kick => self.user_hits.kick.push(self.current_beat()),
            Instrument::OpenHihat => self.user_hits.open_hihat.push(self.current_beat()),
            Instrument::Metronome => self.user_hits.metronome.push(self.current_beat()),
        }

        // // play sound effect
        // let sound = StaticSoundData::from_file(
        //     "res/sounds/metronome.ogg",
        //     StaticSoundSettings::new().start_time(ClockTime {
        //         clock: self.clock.id(),
        //         ticks: self.current_clock_tick() as u64,
        //     }),
        // );
        // self.manager.play(sound.unwrap()).unwrap();

        debug!(
            "Capture at beat = {}, clock = {}",
            self.current_beat(),
            self.current_clock_tick()
        );
    }

    pub fn track_for_calibration(self: &mut Self) -> f64 {
        self.calibration_input.push_back(self.current_beat());

        // play sound effect
        // let sound = StaticSoundData::from_file(
        //     "res/sounds/metronome.ogg",
        //     StaticSoundSettings::new().start_time(ClockTime {
        //         clock: self.clock.id(),
        //         ticks: self.current_clock_tick() as u64,
        //     }),
        // );
        // self.manager.play(sound.unwrap()).unwrap();

        debug!(
            "Capture + calibrate at beat = {}, clock = {}",
            self.current_beat(),
            self.current_clock_tick()
        );
        // compute average distance from integer beats
        let dists = self.calibration_input.iter().map(|x| x - x.round());
        let sum = dists.clone().sum::<f64>();
        let avg_dist = sum / dists.len() as f64;
        if self.calibration_input.len() > 5 {
            self.calibration_input.pop_front();
        }
        debug!(
            "Average distance from integer beats: {} beats ({} seconds)",
            avg_dist,
            avg_dist / self.bpm * 60.
        );
        avg_dist
    }
}

async fn schedule_audio(
    notes: &Vec<f64>,
    sound_path: &str,
    manager: &mut AudioManager,
    clock: &ClockHandle,
    last_scheduled_tick: f64,
    tick_to_schedule: f64,
) -> Result<(), Box<dyn Error>> {
    let prev_beat = last_scheduled_tick % BEATS_PER_LOOP;
    let next_beat = tick_to_schedule % BEATS_PER_LOOP;
    let loop_num = (last_scheduled_tick / BEATS_PER_LOOP) as i32; // floor
    for note in notes.iter() {
        if note > &prev_beat && note <= &next_beat {
            schedule_note(note, loop_num, clock, manager, sound_path).await?;
        };

        // handle wrap-around case
        if next_beat < prev_beat {
            // from prev_beat to end of loop
            if *note > prev_beat && *note <= BEATS_PER_LOOP as f64 {
                schedule_note(note, loop_num, clock, manager, sound_path).await?;
            }
            // from start of loop to next beat
            if *note >= 0. && *note <= next_beat {
                schedule_note(note, loop_num + 1, clock, manager, sound_path).await?;
            }
        }
    }

    Ok(())
}

async fn schedule_note(
    note: &f64,
    loop_num: i32,
    clock: &ClockHandle,
    manager: &mut AudioManager,
    sound_path: &str,
) -> Result<(), Box<dyn Error>> {
    let note_tick = (*note + (loop_num as f64) * BEATS_PER_LOOP) as u64;
    debug!("\tScheduling {} ({}) at {}", sound_path, note, note_tick);
    let f = load_file(sound_path).await?;
    let sound = StaticSoundData::from_cursor(
        Cursor::new(f),
        StaticSoundSettings::new().start_time(ClockTime {
            clock: clock.id(),
            ticks: note_tick,
        }),
    );
    if let Ok(sound) = sound {
        manager.play(sound).unwrap();
    }

    Ok(())
}
