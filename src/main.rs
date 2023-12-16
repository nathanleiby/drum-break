mod consts;
mod ui;

use crate::consts::*;
use crate::ui::*;
use macroquad::prelude::*;

use kira::{
    clock::{ClockHandle, ClockSpeed, ClockTime},
    manager::{backend::DefaultBackend, AudioManager, AudioManagerSettings},
    sound::static_sound::{StaticSoundData, StaticSoundSettings},
    tween::Tween,
};

fn window_conf() -> Conf {
    Conf {
        window_title: "Macroix".to_owned(),
        // fullscreen: true,
        window_width: 1280,
        window_height: 720,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut bpm: f64 = 120.0;

    let mut manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).unwrap();
    let clock = manager
        .add_clock(ClockSpeed::TicksPerMinute(bpm as f64))
        .unwrap();

    // samba beat!
    // let lambda = |x: f64| (x - 1.) / 2.; // 8 quarter note beats per loop
    let lambda = |x: f64| (x - 1.);
    let mut closed_hihat_notes = vec![1., 3., 4., 5., 7., 8., 9., 11., 12., 13., 15., 16.]
        .into_iter()
        .map(lambda)
        .collect();
    let mut snare_notes = vec![1., 3., 6., 8., 10., 13., 15.]
        .into_iter()
        .map(lambda)
        .collect();
    let mut kick_notes: Vec<f64> = vec![1., 4., 5., 8., 9., 12., 13., 16.]
        .into_iter()
        .map(lambda)
        .collect();
    let mut open_hihat_note: Vec<f64> = vec![3., 7., 11., 15.].into_iter().map(lambda).collect();
    let metronome_notes: Vec<f64> = (0..16).into_iter().map(|x| x as f64).collect();

    let mut last_scheduled_tick = -1.;
    let mut last_beat = -1;

    loop {
        ////////////////////////////
        // Schedule audio
        ////////////////////////////
        let current_clock_tick = clock.time().ticks as f64 + clock.fractional_position() as f64;
        if current_clock_tick > last_scheduled_tick {
            let tick_to_schedule = current_clock_tick + TICK_SCHEDULE_AHEAD;

            for pair in [
                (&metronome_notes, "click"),
                (&closed_hihat_notes, "closed-hihat"),
                (&snare_notes, "snare"),
                (&kick_notes, "kick"),
                (&open_hihat_note, "open-hihat"),
            ] {
                let (notes, instrument_name) = pair;
                schedule_audio(
                    &notes,
                    &instrument_name,
                    &mut manager,
                    &clock,
                    last_scheduled_tick,
                    tick_to_schedule,
                );
            }
            last_scheduled_tick = tick_to_schedule;
        }

        ////////////////////////////
        // Handle User Input
        ////////////////////////////

        if is_key_pressed(KeyCode::Space) {
            if clock.ticking() {
                clock.pause().unwrap();
            } else {
                clock.start().unwrap();
            }
        }

        // Improve UX here
        // Check if down < 0.5s then go fast? (then can use same key incr.. "Up")
        if is_key_pressed(KeyCode::Up) {
            bpm += 1.;
            bpm = bpm.max(MIN_BPM).min(MAX_BPM);
            clock
                .set_speed(ClockSpeed::TicksPerMinute(bpm), Tween::default())
                .unwrap();
        }
        if is_key_down(KeyCode::Right) {
            bpm += 1.;
            bpm = bpm.max(MIN_BPM).min(MAX_BPM);
            clock
                .set_speed(ClockSpeed::TicksPerMinute(bpm), Tween::default())
                .unwrap();
        }

        if is_key_pressed(KeyCode::Down) {
            bpm -= 1.;
            bpm = bpm.max(MIN_BPM).min(MAX_BPM);
            clock
                .set_speed(ClockSpeed::TicksPerMinute(bpm), Tween::default())
                .unwrap();
        }
        if is_key_down(KeyCode::Left) {
            // Check if down < 0.5s then go fast?
            bpm -= 1.;
            bpm = bpm.max(MIN_BPM).min(MAX_BPM);
            clock
                .set_speed(ClockSpeed::TicksPerMinute(bpm), Tween::default())
                .unwrap();
        }

        if is_key_pressed(KeyCode::M) {
            // TODO: pause metronome click sound
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            // is on a beat?
            let beat = ((mouse_x as f64 - GRID_LEFT_X) / BEAT_WIDTH_PX).floor();
            let row = ((mouse_y as f64 - GRID_TOP_Y) / ROW_HEIGHT).floor();
            if beat < BEATS_PER_LOOP && row < NUM_ROWS_IN_GRID {
                debug!("Clicked on row={}, beat={}", row, beat);
                if row == 0. {
                    if let Some(pos) = closed_hihat_notes.iter().position(|x| *x == beat) {
                        closed_hihat_notes.remove(pos);
                    } else {
                        closed_hihat_notes.push(beat);
                    }
                } else if row == 1. {
                    if let Some(pos) = snare_notes.iter().position(|x| *x == beat) {
                        snare_notes.remove(pos);
                    } else {
                        snare_notes.push(beat);
                    }
                } else if row == 2. {
                    if let Some(pos) = kick_notes.iter().position(|x| *x == beat) {
                        kick_notes.remove(pos);
                    } else {
                        kick_notes.push(beat);
                    }
                } else if row == 3. {
                    if let Some(pos) = open_hihat_note.iter().position(|x| *x == beat) {
                        open_hihat_note.remove(pos);
                    } else {
                        open_hihat_note.push(beat);
                    }
                }
            }
        }

        ////////////////////////////
        // Render UI
        ////////////////////////////
        clear_background(LIGHTGRAY);
        draw_beat_grid(
            &closed_hihat_notes,
            &snare_notes,
            &kick_notes,
            &open_hihat_note,
        );

        // Get current beat (from 0 to BEATS_PER_LOOP)
        let current_beat = current_clock_tick % BEATS_PER_LOOP;
        draw_position_line(current_beat);
        if (current_beat as i32) > last_beat {
            debug!("Beat: {}", current_beat as i32);
            last_beat = current_beat as i32;
        }

        draw_status(bpm, current_beat);

        next_frame().await
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
