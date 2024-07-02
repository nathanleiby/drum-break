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

#[derive(Debug, Clone)]
pub struct UserHit {
    pub instrument: Instrument,
    pub clock_tick: f64,
}

impl UserHit {
    pub fn new(instrument: Instrument, clock_tick: f64) -> Self {
        Self {
            instrument,
            clock_tick,
        }
    }

    pub fn beat(self: &Self) -> f64 {
        self.clock_tick % BEATS_PER_LOOP
    }
}

/// Audio is the audio player and tracks the user's hits in relation to the audio timing.
///
/// These two responsibilities co-exist so that the audio player's subtle timing issues
/// can be measured and corrected for.
pub struct Audio {
    manager: AudioManager<DefaultBackend>,
    clock: ClockHandle,
    last_scheduled_tick: f64,
    bpm: f64,

    pub user_hits: Vec<UserHit>,
    calibration_input: VecDeque<f64>,
    configured_audio_latency_seconds: f64,

    // debug only
    last_beat: i32,
}

const DEFAULT_BPM: f64 = 60.;
const MIN_BPM: f64 = 40.;
const MAX_BPM: f64 = 240.;

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

            user_hits: vec![],
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

    // TODO: Move this outside and then use it to summary loop accuracy
    fn print_if_new_beat(self: &mut Self) {
        // For debugging, print when we pass an integer beat
        let current_beat = self.current_beat() as i32;
        if current_beat != self.last_beat {
            // log::debug!("Beat: {}", current_beat as i32);
            self.last_beat = current_beat as i32;
            // if new loop, print that too
            if current_beat == 0 {
                // log::debug!("Starting loop num #{:?}", self.current_loop());
            }
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

        log::debug!(
            "Scheduling from {} to {}",
            self.last_scheduled_tick,
            tick_to_schedule
        );
        for pair in [
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

        // if metronome on, schedule it
        // (&voices.metronome, "res/sounds/click.wav"),

        self.last_scheduled_tick = tick_to_schedule;

        Ok(())
    }

    fn current_clock_tick(self: &Self) -> f64 {
        self.clock.time().ticks as f64 + self.clock.fractional_position()
    }

    pub fn current_beat(self: &Self) -> f64 {
        self.current_clock_tick() % BEATS_PER_LOOP
    }

    pub fn current_loop(self: &Self) -> i32 {
        (self.current_clock_tick() / BEATS_PER_LOOP) as i32
    }

    fn get_seconds_per_tick(self: &Self) -> f64 {
        60. / self.bpm / 2.
    }

    pub fn get_bpm(self: &Self) -> f64 {
        self.bpm
    }

    pub fn set_bpm(self: &mut Self, bpm: f64) {
        self.bpm = clamp(bpm, MIN_BPM, MAX_BPM);
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

    /// saves a user's hits, so they can be displayed and checked for accuracy
    pub fn track_user_hit(self: &mut Self, instrument: Instrument, processing_delay_s: f64) {
        // convert processing delay to ticks, based on BPM
        let ticks_per_second = 1. / self.get_seconds_per_tick();
        let processing_delay_ticks = ticks_per_second * processing_delay_s;

        self.user_hits.push(UserHit::new(
            instrument,
            self.current_clock_tick() - processing_delay_ticks,
        ));

        // // play sound effect
        // let sound = StaticSoundData::from_file(
        //     "res/sounds/metronome.ogg",
        //     StaticSoundSettings::new().start_time(ClockTime {
        //         clock: self.clock.id(),
        //         ticks: self.current_clock_tick() as u64,
        //     }),
        // );
        // self.manager.play(sound.unwrap()).unwrap();

        log::debug!(
            "Capture at beat = {}, clock = {}",
            self.current_beat(),
            self.current_clock_tick()
        );
    }

    /// allows for hitting a single key repeatedly on the heard beat to calibrate the audio latency
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

        log::debug!(
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
        log::debug!(
            "Average distance from integer beats: {} beats ({} seconds)",
            avg_dist,
            avg_dist / self.bpm * 60.
        );
        avg_dist
    }
}

/// schedules notes for a single sound to be played between last_scheduled_tick and tick_to_schedule
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

/// schedules a single note to be played at a specific tick
async fn schedule_note(
    note: &f64,
    loop_num: i32,
    clock: &ClockHandle,
    manager: &mut AudioManager,
    sound_path: &str,
) -> Result<(), Box<dyn Error>> {
    let note_tick = (*note + (loop_num as f64) * BEATS_PER_LOOP) as u64;
    // log::debug!("\tScheduling {} ({}) at {}", sound_path, note, note_tick);
    let f = load_file(sound_path).await?;
    let sound = StaticSoundData::from_cursor(
        Cursor::new(f),
        match sound_path {
            // TODO: why are we playing hihat sound more quietly?
            "res/sounds/open-hihat.wav" => StaticSoundSettings::new().volume(0.5),
            _ => StaticSoundSettings::new(),
        }
        .start_time(ClockTime {
            clock: clock.id(),
            ticks: note_tick,
        }),
    );
    if let Ok(sound) = sound {
        manager.play(sound).unwrap();
    }

    Ok(())
}
