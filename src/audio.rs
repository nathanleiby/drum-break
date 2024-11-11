use std::{
    collections::{HashMap, VecDeque},
    error::Error,
    io::Cursor,
    sync::mpsc::Sender,
};

use kira::{
    clock::{ClockHandle, ClockSpeed, ClockTime},
    manager::{backend::DefaultBackend, AudioManager, AudioManagerSettings},
    sound::static_sound::{StaticSoundData, StaticSoundSettings},
    tween::Tween,
};

use macroquad::prelude::*;

use crate::{
    config::AppConfig,
    consts::{TxMsg, UserHit, ALL_INSTRUMENTS, BEATS_PER_LOOP, TICK_SCHEDULE_AHEAD},
    voices::{Instrument, Voices},
};

/// Audio is the audio player and tracks the user's hits in relation to the audio timing.
///
/// These two responsibilities co-exist so that the audio player's subtle timing issues
/// can be measured and corrected for.
pub struct Audio {
    manager: AudioManager<DefaultBackend>,
    clock: ClockHandle,
    last_scheduled_tick: f64,
    bpm: f64,
    metronome_enabled: bool,

    sounds: HashMap<Instrument, StaticSoundData>,
    pub user_hits: Vec<UserHit>,
    calibration_input: VecDeque<f64>,
    configured_audio_latency_seconds: f64,

    tx: Sender<TxMsg>,

    // debug only
    last_beat: i32,
}

const DEFAULT_BPM: f64 = 60.;
const MIN_BPM: f64 = 40.;
const MAX_BPM: f64 = 240.;

impl Audio {
    pub fn new(conf: &AppConfig, tx: Sender<TxMsg>) -> Self {
        let mut manager =
            AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).unwrap();
        let clock = manager
            // TODO: investigate bpm * 2 stuff
            .add_clock(ClockSpeed::TicksPerMinute(DEFAULT_BPM * 2_f64))
            .unwrap();

        tx.send(TxMsg::AudioNew).unwrap();

        Self {
            manager,
            clock,
            last_scheduled_tick: -1.,
            bpm: DEFAULT_BPM,
            metronome_enabled: false,
            sounds: HashMap::new(),

            user_hits: vec![],
            calibration_input: VecDeque::new(),
            configured_audio_latency_seconds: conf.audio_latency_seconds,
            last_beat: -1,

            tx,
        }
    }

    pub fn new_mock(conf: &AppConfig, tx: Sender<TxMsg>) -> Self {
        let mut audio = Audio::new(conf, tx);
        audio.user_hits = vec![UserHit {
            instrument: Instrument::ClosedHihat,
            clock_tick: 1.0,
        }];
        audio
    }

    // initialize() loads required resources, like audio data
    // I've separated this from new() because it's async and it may error.
    pub async fn initialize(&mut self) -> Result<(), Box<dyn Error>> {
        for ins in ALL_INSTRUMENTS {
            let sound_path = Voices::get_audio_file_for_instrument(&ins);
            let f = load_file(sound_path).await?;
            let sound = StaticSoundData::from_cursor(Cursor::new(f))?;
            self.sounds.insert(ins, sound);
        }

        Ok(())
    }

    // audio latency
    pub fn get_configured_audio_latency_seconds(&self) -> f64 {
        self.configured_audio_latency_seconds
    }

    pub fn set_configured_audio_latency_seconds(&mut self, latency: f64) {
        self.configured_audio_latency_seconds = latency;
    }

    // TODO: Move this outside and then use it to summary loop accuracy
    fn check_if_new_beat_or_new_loop(&mut self) {
        // For debugging, print when we pass an integer beat
        let current_beat = self.current_beat() as i32;
        if current_beat != self.last_beat {
            // log::debug!("Beat: {}", current_beat as i32);
            self.last_beat = current_beat;
            // if new loop, print that too
            if current_beat == 0 {
                self.tx
                    .send(TxMsg::StartingLoop(self.current_loop()))
                    .unwrap();
                // log::debug!("Starting loop num #{:?}", self.current_loop());
            }
        }
    }

    /// schedule should be run within each game tick to schedule the audio
    pub async fn schedule(&mut self, voices: &Voices) -> Result<(), Box<dyn Error>> {
        self.check_if_new_beat_or_new_loop();

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

        for ins in ALL_INSTRUMENTS.iter() {
            let notes = voices.get_instrument_beats(ins);
            // fetch sound data from hashmap and the clone() it to re-use
            let sound = self
                .sounds
                .get(ins)
                .expect("Failed to load sound for instrument: {} ... was audio.initialize() run?")
                .clone();

            schedule_audio(
                notes,
                &sound,
                get_volume(ins),
                &mut self.manager,
                &self.clock,
                self.last_scheduled_tick,
                tick_to_schedule,
            )?;
        }

        if self.is_metronome_enabled() {
            // TODO: play a different sound at start of each measure
            // clicks on quarter notes
            let metronome_notes = vec![0., 2., 4., 6., 8., 10., 12., 14.];
            let sound_path = "res/sounds/click.wav"; // TODO: metronome.ogg?
            let f = load_file(sound_path).await?;
            let sound = StaticSoundData::from_cursor(Cursor::new(f))?;
            let volume = 1.;
            schedule_audio(
                &metronome_notes,
                &sound,
                volume,
                &mut self.manager,
                &self.clock,
                self.last_scheduled_tick,
                tick_to_schedule,
            )?;
        }

        self.last_scheduled_tick = tick_to_schedule;

        Ok(())
    }

    fn current_clock_tick(&self) -> f64 {
        self.clock.time().ticks as f64 + self.clock.time().fraction
    }

    pub fn current_beat(&self) -> f64 {
        self.current_clock_tick() % BEATS_PER_LOOP
    }

    pub fn current_loop(&self) -> i32 {
        (self.current_clock_tick() / BEATS_PER_LOOP) as i32
    }

    fn get_seconds_per_tick(&self) -> f64 {
        60. / self.bpm / 2.
    }

    pub fn get_bpm(&self) -> f64 {
        self.bpm
    }

    pub fn set_bpm(&mut self, bpm: f64) {
        self.bpm = clamp(bpm, MIN_BPM, MAX_BPM);
        self.clock
            .set_speed(ClockSpeed::TicksPerMinute(bpm * 2.), Tween::default())
    }

    pub fn toggle_pause(&mut self) {
        if self.clock.ticking() {
            self.clock.pause();
        } else {
            self.clock.start();
        }
    }

    pub fn is_paused(&self) -> bool {
        !self.clock.ticking()
    }

    pub fn toggle_metronome(&mut self) {
        self.metronome_enabled = !self.metronome_enabled;
    }

    pub fn is_metronome_enabled(&self) -> bool {
        self.metronome_enabled
    }

    // TODO: Feels like this could be moved elsewhere, with a quick lookup against audio if needed (e.g. get_seconds_per_tick)

    /// saves a user's hits, so they can be displayed and checked for accuracy
    pub fn track_user_hit(&mut self, instrument: Instrument, processing_delay_s: f64) {
        // convert processing delay to ticks, based on BPM
        let ticks_per_second = 1. / self.get_seconds_per_tick();
        let processing_delay_ticks = ticks_per_second * processing_delay_s;

        self.user_hits.push(UserHit::new(
            instrument,
            self.current_clock_tick() - processing_delay_ticks,
        ));

        log::debug!(
            "Capture at beat = {}, clock = {}",
            self.current_beat(),
            self.current_clock_tick()
        );
    }

    /// allows for hitting a single key repeatedly on the heard beat to calibrate the audio latency
    pub fn track_for_calibration(&mut self) -> f64 {
        self.calibration_input.push_back(self.current_beat());

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
fn schedule_audio(
    notes: &[f64],
    sound: &StaticSoundData,
    volume: f64,
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
            schedule_note(note, loop_num, clock, manager, sound, volume)?;
        };

        // handle wrap-around case
        if next_beat < prev_beat {
            // from prev_beat to end of loop
            if *note > prev_beat && *note <= BEATS_PER_LOOP {
                schedule_note(note, loop_num, clock, manager, sound, volume)?;
            }
            // from start of loop to next beat
            if *note >= 0. && *note <= next_beat {
                schedule_note(note, loop_num + 1, clock, manager, sound, volume)?;
            }
        }
    }

    Ok(())
}

/// schedules a single note to be played at a specific tick
fn schedule_note(
    note: &f64,
    loop_num: i32,
    clock: &ClockHandle,
    manager: &mut AudioManager,
    sound: &StaticSoundData,
    volume: f64,
) -> Result<(), Box<dyn Error>> {
    let note_tick = (*note + (loop_num as f64) * BEATS_PER_LOOP) as u64;
    log::debug!(
        "\tScheduling {:?} ({}) at {}",
        sound.settings,
        note,
        note_tick
    );

    // Set volume and timing
    let settings = StaticSoundSettings::new()
        .volume(volume)
        .start_time(ClockTime {
            clock: clock.id(),
            ticks: note_tick,
            fraction: 0.,
        });

    manager.play(sound.with_settings(settings))?;

    Ok(())
}

fn get_volume(ins: &Instrument) -> f64 {
    match ins {
        Instrument::OpenHihat => 0.5,
        Instrument::PedalHiHat => 0.5,
        Instrument::Ride => 0.15,
        Instrument::Crash => 0.4,
        Instrument::Tom1 | Instrument::Tom2 | Instrument::Tom3 => 0.25,
        _ => 1.0,
    }
}
