use macroquad::{
    audio::{load_sound, play_sound_once},
    prelude::*,
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

const MOVE_SPEED: f32 = 200.0;

#[macroquad::main(window_conf)]
async fn main() {
    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;
    let mut seconds: f32 = 0.0;

    let hihat_sound = load_sound("res/closed-hihat.wav").await.unwrap();
    let open_hihat_sound = load_sound("res/open-hihat.wav").await.unwrap();
    let snare_sound = load_sound("res/snare.wav").await.unwrap();
    let kick_sound = load_sound("res/kick.wav").await.unwrap();

    loop {
        clear_background(LIGHTGRAY);

        // using delta time
        let delta = get_frame_time();
        seconds += delta;

        // testing keyboard input
        if is_key_down(KeyCode::Right) {
            x += delta * MOVE_SPEED;
        }
        if is_key_down(KeyCode::Left) {
            x -= delta * MOVE_SPEED;
        }
        if is_key_down(KeyCode::Down) {
            y += delta * MOVE_SPEED;
        }
        if is_key_down(KeyCode::Up) {
            y -= delta * MOVE_SPEED;
        }

        // playing sounds
        if is_key_pressed(KeyCode::A) {
            play_sound_once(&hihat_sound)
        }
        if is_key_pressed(KeyCode::S) {
            play_sound_once(&snare_sound)
        }
        if is_key_pressed(KeyCode::D) {
            play_sound_once(&kick_sound)
        }
        if is_key_pressed(KeyCode::F) {
            play_sound_once(&open_hihat_sound)
        }

        draw_beat_grid();
        draw_position_line(seconds);

        // TIL: If you alt-tab while a button is pressed, Macroquad will not treat it as released
        draw_user_controlled_object(x, y);

        next_frame().await
    }
}

const BEAT_WIDTH: f32 = 64.0;
const BEAT_PADDING: f32 = 4.;

const GRID_WIDTH: f32 = BEAT_WIDTH * 16.;
const ROW_HEIGHT: f32 = BEAT_WIDTH;

const GRID_LEFT_X: f32 = 32.;
const GRID_TOP_Y: f32 = 32.;

const BPM: f32 = 120.0;
const BEAT_DURATION_SECONDS: f32 = 60. / BPM;

fn draw_beat_grid() {
    let start_x = GRID_LEFT_X + BEAT_WIDTH;
    let start_y = GRID_TOP_Y;
    for i in 1..=5 {
        let y = start_y + i as f32 * ROW_HEIGHT;
        draw_line(start_x, y, start_x + GRID_WIDTH, y, 4.0, BLACK);
    }

    // draw vertical lines every 4 beats
    for i in 0..=16 {
        let x = start_x + i as f32 * BEAT_WIDTH;
        draw_line(x, start_y, x, start_y + ROW_HEIGHT * 5., 4.0, BLACK);
    }
    // samba beat!
    let hihat_notes = [1., 3., 4., 5., 7., 8., 9., 11., 12., 13., 15., 16.];
    for note in hihat_notes.iter() {
        draw_note(*note, 1);
    }

    let snare_notes = [1., 3., 6., 8., 10., 13., 15.];
    for note in snare_notes.iter() {
        draw_note(*note, 2);
    }

    // same kick notes but with a lead up to each note
    let kick_notes = [1., 4., 5., 8., 9., 12., 13., 16.];
    for note in kick_notes.iter() {
        draw_note(*note, 3);
    }

    // same kick notes but with a lead up to each note
    let open_hihat_notes = [3., 7., 11., 15.];
    for note in open_hihat_notes.iter() {
        draw_note(*note, 4);
    }
}

fn draw_user_controlled_object(x: f32, y: f32) {
    draw_rectangle(x, y, 32., 32., BLUE);
}

fn draw_position_line(seconds: f32) {
    let start_x = GRID_LEFT_X + BEAT_WIDTH;
    let start_y = GRID_TOP_Y;

    // total seconds for 16 beats
    let total_seconds = BEAT_DURATION_SECONDS * 16.0;
    let seconds_per_pixel = total_seconds / GRID_WIDTH;

    // draw a vertical line at the current positon
    let x = start_x + seconds / seconds_per_pixel;
    draw_line(x, start_y, x, start_y + ROW_HEIGHT * 5., 4.0, RED);
}

fn draw_note(beats_offset: f32, row: usize) {
    let beat_duration = 1 as f32;
    let x = GRID_LEFT_X + beats_offset * BEAT_WIDTH;
    let y = GRID_TOP_Y + row as f32 * ROW_HEIGHT;
    draw_rectangle(
        x + BEAT_PADDING / 2.,
        y + BEAT_PADDING / 2.,
        BEAT_WIDTH * beat_duration - BEAT_PADDING,
        BEAT_WIDTH - BEAT_PADDING,
        ORANGE,
    );
}

// Try Kira for game audio and sync
// https://github.com/tesselode/kira
