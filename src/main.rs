use macroquad::prelude::*;

use kira::{
    clock::{ClockHandle, ClockSpeed, ClockTime},
    manager::{backend::DefaultBackend, AudioManager, AudioManagerSettings},
    sound::static_sound::{StaticSoundData, StaticSoundSettings},
    tween::Tween,
};

fn window_conf() -> Conf {
    Conf {
        window_title: "Window Conf".to_owned(),
        // fullscreen: true,
        window_width: 1280,
        window_height: 720,
        ..Default::default()
    }
}

const BEATS_PER_LOOP: f64 = 16.;

const BEAT_WIDTH_PX: f64 = 64.0;
const BEAT_PADDING: f64 = 4.;

const GRID_WIDTH: f64 = BEAT_WIDTH_PX * 16.;
const ROW_HEIGHT: f64 = BEAT_WIDTH_PX;

const GRID_LEFT_X: f64 = 32.;
const GRID_TOP_Y: f64 = 64.;

const TICK_SCHEDULE_AHEAD: f64 = 4.;

#[macroquad::main(window_conf)]
async fn main() {
    let mut bpm: f64 = 120.0;

    let mut manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).unwrap();
    let clock = manager
        .add_clock(ClockSpeed::TicksPerMinute(bpm as f64))
        .unwrap();

    let closed_hihat_notes = vec![1., 3., 4., 5., 7., 8., 9., 11., 12., 13., 15., 16.]
        .into_iter()
        .map(|x| x - 1.)
        .collect();
    let snare_notes = vec![1., 3., 6., 8., 10., 13., 15.]
        .into_iter()
        .map(|x| x - 1.)
        .collect();
    let kick_notes: Vec<f64> = vec![1., 4., 5., 8., 9., 12., 13., 16.]
        .into_iter()
        .map(|x| x - 1.)
        .collect();
    let open_hihat_note: Vec<f64> = vec![3., 7., 11., 15.].into_iter().map(|x| x - 1.).collect();

    let mut last_scheduled_tick = -1.;
    let mut last_beat = -1;
    loop {
        clear_background(LIGHTGRAY);

        let current_clock_tick = clock.time().ticks as f64 + clock.fractional_position() as f64;

        if current_clock_tick > last_scheduled_tick {
            let tick_to_schedule = current_clock_tick + TICK_SCHEDULE_AHEAD;
            schedule_audio(
                &closed_hihat_notes,
                "closed-hihat",
                &mut manager,
                &clock,
                last_scheduled_tick,
                tick_to_schedule,
            );
            schedule_audio(
                &snare_notes,
                "snare",
                &mut manager,
                &clock,
                last_scheduled_tick,
                tick_to_schedule,
            );
            schedule_audio(
                &kick_notes,
                "kick",
                &mut manager,
                &clock,
                last_scheduled_tick,
                tick_to_schedule,
            );
            schedule_audio(
                &open_hihat_note,
                "open-hihat",
                &mut manager,
                &clock,
                last_scheduled_tick,
                tick_to_schedule,
            );

            last_scheduled_tick = tick_to_schedule;
        }

        if is_key_pressed(KeyCode::Space) {
            if clock.ticking() {
                clock.stop().unwrap();
            } else {
                clock.start().unwrap();
            }
        }

        if is_key_pressed(KeyCode::Up) {
            bpm += 1.;
            clock
                .set_speed(ClockSpeed::TicksPerMinute(bpm), Tween::default())
                .unwrap();
        }

        if is_key_pressed(KeyCode::Down) {
            bpm -= 1.;
            clock
                .set_speed(ClockSpeed::TicksPerMinute(bpm), Tween::default())
                .unwrap();
        }

        if is_key_down(KeyCode::Down) {
            bpm -= 1.;
            clock
                .set_speed(ClockSpeed::TicksPerMinute(bpm), Tween::default())
                .unwrap();
        }

        draw_beat_grid();

        // Get current beat (from 0 to BEATS_PER_LOOP)
        let current_beat = current_clock_tick % BEATS_PER_LOOP;
        // TODO: loop it
        draw_position_line(current_beat);
        if (current_beat as i32) > last_beat {
            info!("Beat: {}", current_beat as i32);
            last_beat = current_beat as i32;
        }

        draw_text(format!("BPM: {bpm}").as_str(), 20.0, 20.0, 30.0, DARKGRAY);
        draw_text(
            format!("Current Beat: {:.1}", current_beat).as_str(),
            20.0,
            40.0,
            30.0,
            DARKGRAY,
        );

        draw_text(
            "Hihat",
            20.0,
            (GRID_TOP_Y + ROW_HEIGHT * 0.5) as f32,
            30.0,
            DARKGRAY,
        );
        draw_text(
            "Snare",
            20.0,
            (GRID_TOP_Y + ROW_HEIGHT * 1.5) as f32,
            30.0,
            DARKGRAY,
        );
        draw_text(
            "Kick",
            20.0,
            (GRID_TOP_Y + ROW_HEIGHT * 2.5) as f32,
            30.0,
            DARKGRAY,
        );
        draw_text(
            "Open hi-hat",
            20.0,
            (GRID_TOP_Y + ROW_HEIGHT * 3.5) as f32,
            30.0,
            DARKGRAY,
        );
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
    info!(
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
    info!(
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

const NUM_ROWS_IN_GRID: f64 = 4.;

fn draw_beat_grid() {
    let start_x = GRID_LEFT_X + BEAT_WIDTH_PX;
    let start_y = GRID_TOP_Y;
    for i in 0..=(NUM_ROWS_IN_GRID as usize) {
        let y = start_y + i as f64 * ROW_HEIGHT;
        draw_line_f64(start_x, y, start_x + GRID_WIDTH, y, 4.0, BLACK);
    }

    // draw vertical lines every 4 beats
    for i in 0..=(BEATS_PER_LOOP as i32) {
        let x = start_x + i as f64 * BEAT_WIDTH_PX;
        draw_line_f64(
            x,
            start_y,
            x,
            start_y + ROW_HEIGHT * NUM_ROWS_IN_GRID,
            4.0,
            BLACK,
        );
    }
    // samba beat!
    let hihat_notes = [1., 3., 4., 5., 7., 8., 9., 11., 12., 13., 15., 16.];
    for note in hihat_notes.iter() {
        draw_note(*note, 0);
    }

    let snare_notes = [1., 3., 6., 8., 10., 13., 15.];
    for note in snare_notes.iter() {
        draw_note(*note, 1);
    }

    // same kick notes but with a lead up to each note
    let kick_notes = [1., 4., 5., 8., 9., 12., 13., 16.];
    for note in kick_notes.iter() {
        draw_note(*note, 2);
    }

    // same kick notes but with a lead up to each note
    let open_hihat_notes = [3., 7., 11., 15.];
    for note in open_hihat_notes.iter() {
        draw_note(*note, 3);
    }
}

fn draw_position_line(current_beat: f64) {
    let start_x = GRID_LEFT_X + BEAT_WIDTH_PX;
    let start_y = GRID_TOP_Y;

    // draw a vertical line at the current positonj
    let x = start_x + current_beat * BEAT_WIDTH_PX;
    draw_line_f64(x, start_y, x, start_y + ROW_HEIGHT * 5., 4.0, RED);
}

fn draw_note(beats_offset: f64, row: usize) {
    let beat_duration = 1 as f64;
    let x = GRID_LEFT_X + beats_offset * BEAT_WIDTH_PX;
    let y = GRID_TOP_Y + row as f64 * ROW_HEIGHT;
    draw_rectangle_f64(
        x + BEAT_PADDING / 2.,
        y + BEAT_PADDING / 2.,
        BEAT_WIDTH_PX * beat_duration - BEAT_PADDING,
        BEAT_WIDTH_PX - BEAT_PADDING,
        ORANGE,
    );
}

fn draw_rectangle_f64(x: f64, y: f64, width: f64, height: f64, color: Color) {
    draw_rectangle(x as f32, y as f32, width as f32, height as f32, color);
}

fn draw_line_f64(x1: f64, y1: f64, x2: f64, y2: f64, thickness: f32, color: Color) {
    draw_line(x1 as f32, y1 as f32, x2 as f32, y2 as f32, thickness, color);
}
