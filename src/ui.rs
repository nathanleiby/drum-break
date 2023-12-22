use std::collections::HashMap;

use crate::{
    audio::Audio,
    consts::*,
    score::{compute_accuracy, Accuracy},
    Voices,
};

use macroquad::{prelude::*, ui::*};

pub struct UI {
    path_to_loop: String,
}

impl UI {
    pub fn new() -> Self {
        Self {
            path_to_loop: String::new(),
        }
    }

    pub fn render(self: &mut Self, voices: &mut Voices, audio: &Audio) {
        let current_beat = audio.current_beat();
        let audio_latency = audio.get_configured_audio_latency_seconds();
        let bpm = audio.get_bpm();

        clear_background(Color {
            r: 0.9,
            g: 0.9,
            b: 0.9,
            a: 1.0,
        });
        draw_beat_grid(voices);
        draw_user_hits(&audio.user_hits, &voices, audio_latency);
        draw_position_line(current_beat + audio_latency);
        draw_status(bpm, current_beat / 2., audio_latency);

        draw_pulse_beat(current_beat + audio_latency);

        //// TODO: resolve issue with: assertion failed: !QUAD_CONTEXT.is_null()
        // egui_macroquad::ui(|egui_ctx| {
        //     egui::Window::new("egui ❤ macroquad").show(egui_ctx, |ui| {
        //         ui.label("Test");
        //     });
        // });

        // // Draw things before egui

        // egui_macroquad::draw();

        let mut voices_options: Vec<(String, Voices)> = Vec::new();

        // TODO: bootstrap from file elsewhere
        let mut bup1 = Voices::new();
        bup1.kick.push(0.0);
        let mut bup2 = Voices::new();
        bup2.kick.push(4.0);
        voices_options.push(("Bulls on Parade (1)".to_string(), bup1));
        voices_options.push(("Bulls on Parade (2)".to_string(), bup2));

        draw_ui(voices, &mut voices_options);
    }
}

fn draw_status(bpm: f64, current_beat: f64, audio_latency: f64) {
    draw_text(
        format!("BPM: {bpm}").as_str(),
        (GRID_LEFT_X) as f32,
        20.0,
        30.0,
        DARKGRAY,
    );
    draw_text(
        format!("Current Beat: {:.1}", current_beat).as_str(),
        (GRID_LEFT_X) as f32,
        40.0,
        30.0,
        DARKGRAY,
    );
    draw_text(
        format!("Calibrated Latency: {:.3} seconds", audio_latency).as_str(),
        (GRID_LEFT_X) as f32,
        60.0,
        30.0,
        DARKGRAY,
    );
}

fn draw_beat_grid(voices: &Voices) {
    let closed_hihat_notes = &voices.closed_hihat;
    let snare_notes = &voices.snare;
    let kick_notes = &voices.kick;
    let open_hihat_notes = &voices.open_hihat;

    // Labels in top-left of grid
    for (idx, name) in ["Hihat", "Snare", "Kick", "Open hi-hat"].iter().enumerate() {
        draw_text(
            name,
            20.0,
            (GRID_TOP_Y + ROW_HEIGHT * (idx as f64 + 0.5)) as f32,
            20.0,
            DARKGRAY,
        );
    }

    let start_x = GRID_LEFT_X;
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

    for note in closed_hihat_notes.iter() {
        draw_note(*note, 0);
    }

    for note in snare_notes.iter() {
        draw_note(*note, 1);
    }

    // same kick notes but with a lead up to each note
    for note in kick_notes.iter() {
        draw_note(*note, 2);
    }

    // same kick notes but with a lead up to each note
    for note in open_hihat_notes.iter() {
        draw_note(*note, 3);
    }
}

fn draw_user_hits(user_hits: &Voices, desired_hits: &Voices, audio_latency: f64) {
    let closed_hihat_notes = &user_hits.closed_hihat;
    let snare_notes = &user_hits.snare;
    let kick_notes = &user_hits.kick;
    let open_hihat_notes = &user_hits.open_hihat;

    for note in closed_hihat_notes.iter() {
        draw_user_hit(*note, 0, audio_latency, &desired_hits.closed_hihat);
    }

    for note in snare_notes.iter() {
        draw_user_hit(*note, 1, audio_latency, &desired_hits.snare);
    }

    // same kick notes but with a lead up to each note
    for note in kick_notes.iter() {
        draw_user_hit(*note, 2, audio_latency, &desired_hits.kick);
    }

    // same kick notes but with a lead up to each note
    for note in open_hihat_notes.iter() {
        draw_user_hit(*note, 3, audio_latency, &desired_hits.open_hihat);
    }
}

fn draw_position_line(current_beat: f64) {
    let start_x = GRID_LEFT_X;
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
        Color {
            r: 1.0,
            g: 0.63 + row as f32 * 0.1,
            b: 0.0 + row as f32 * 0.1,
            a: 1.0,
        },
    );
}

fn draw_user_hit(user_beat: f64, row: usize, audio_latency: f64, desired_hits: &Vec<f64>) {
    let user_beat_with_latency = user_beat + audio_latency;

    let acc = compute_accuracy(user_beat_with_latency, desired_hits);

    let beat_duration = 0.2 as f64; // make it thin for easier overlap, for now

    // with audio latency
    let x = GRID_LEFT_X + user_beat_with_latency * BEAT_WIDTH_PX;
    let y = GRID_TOP_Y + row as f64 * ROW_HEIGHT;

    draw_rectangle_f64(
        x + BEAT_PADDING / 2.,
        y + BEAT_PADDING / 2.,
        BEAT_WIDTH_PX * beat_duration - BEAT_PADDING,
        BEAT_WIDTH_PX - BEAT_PADDING,
        match acc {
            Accuracy::Early => ORANGE,
            Accuracy::Late => PURPLE,
            Accuracy::Correct => GREEN,
            Accuracy::Miss => RED,
        },
    );
}

fn draw_rectangle_f64(x: f64, y: f64, width: f64, height: f64, color: Color) {
    draw_rectangle(x as f32, y as f32, width as f32, height as f32, color);
}

fn draw_line_f64(x1: f64, y1: f64, x2: f64, y2: f64, thickness: f32, color: Color) {
    draw_line(x1 as f32, y1 as f32, x2 as f32, y2 as f32, thickness, color);
}

fn draw_pulse_beat(current_beat: f64) {
    // every other beat
    if current_beat.floor() % 2. == 0. {
        return;
    }

    // get the distance from the current beat center
    let dist = (1. - current_beat % 1.).abs();
    if dist < 0.05 {
        draw_circle(screen_width() / 2., screen_height() / 2. + 100., 100., RED);
    }
}

const UI_TOP_LEFT: Vec2 = vec2(100., 400.);

// maybe macroquad built in UI is somewhat useful
// https://docs.rs/macroquad/latest/macroquad/ui/index.html
// https://github.com/not-fl3/macroquad/blob/master/examples/ui.rs
fn draw_ui<'a, 'b: 'a>(mut voices: &'a mut Voices, voices_options: &'b mut Vec<(String, Voices)>) {
    widgets::Window::new(hash!(), UI_TOP_LEFT, vec2(320., 200.))
        .label("Loops")
        .titlebar(true)
        .ui(&mut *root_ui(), |ui| {
            // if ui.button(None, "Bulls on Parade (1)") {
            //     *path_to_loop = "res/loops/bulls_on_parade_1.json".to_string();
            // }

            // iterate over voices options by sorted keys..
            voices_options.iter().for_each(|(name, new_voices)| {
                if ui.button(None, name.as_str()) {
                    *voices = new_voices.clone();
                    info!("Switched to {:?}", name);
                }
            });

            // if ui.button(None, "Bulls on Parade (1b)") {
            //     let path_to_loop = "res/loops/bulls_on_parade_1b.json".to_string();
            //     if let Ok(default_voices) = Voices::new_from_file(&path_to_loop) {
            //         voices = default_voices;
            //     }
            // }

            // if ui.button(None, "Bulls on Parade (2)") {
            //     *path_to_loop = "res/loops/bulls_on_parade_2.json".to_string();
            // }

            // if ui.button(None, "Samba") {
            //         *path_to_loop = "res/loops/samba.json".to_string();
            //     }
        });
}
