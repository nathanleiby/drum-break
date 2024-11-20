use egui::{
    self,
    emath::{self, RectTransform},
    pos2, CollapsingHeader, Color32, Shape, Widget,
};

use egui_plot::{Line, Plot};

use log::info;
use macroquad::color::{DARKBLUE, GREEN, LIGHTGRAY, ORANGE, PURPLE, RED};

use crate::{
    consts::{UserHit, ALL_INSTRUMENTS},
    events::Events,
    score::{
        compute_accuracy_of_single_hit, compute_last_loop_summary,
        compute_loop_performance_for_voice, get_hits_from_nth_loop,
        get_user_hit_timings_by_instrument, Accuracy, MISS_MARGIN,
    },
    voices::{Instrument, Voices},
};

// This resource holds information about the game:
pub struct UIState {
    selector_vec: Vec<String>,
    selected_idx: usize,

    is_playing: bool,
    bpm: f32,
    is_metronome_enabled: bool,
    #[allow(dead_code)]
    volume_metronome: f32,
    #[allow(dead_code)]
    volume_target_notes: f32,

    // audio
    current_loop: usize, // nth loop
    current_beat: f32,
    beats_per_loop: usize,

    latency_offset_s: f32,

    user_hits: Vec<UserHit>,
    desired_hits: Voices,

    is_help_visible: bool,
    are_side_panels_visible: bool,

    is_dev_tools_visible: bool,
    correct_margin: f64,
    miss_margin: f64,

    hide_empty_tracks: bool,
    midi_device_name: String,
    // user interaction state
    // is_dragging,
}

impl Default for UIState {
    fn default() -> Self {
        Self {
            // Example stuff:
            is_playing: false,

            selector_vec: vec![
                // TODO: String::from() vs .. "".to_owned() ?
                String::from("Rock"),
                String::from("Breakbeat"),
                String::from("Samba"),
            ],
            selected_idx: 0,

            current_loop: 2,
            current_beat: 2.3,

            beats_per_loop: 16,
            bpm: 120.,

            is_metronome_enabled: false,
            volume_metronome: 0.75,
            volume_target_notes: 0.75,

            latency_offset_s: 0.,

            user_hits: vec![],
            desired_hits: Voices::new(),

            is_help_visible: false,
            are_side_panels_visible: false,

            is_dev_tools_visible: false,
            correct_margin: 0.,
            miss_margin: 0.,

            hide_empty_tracks: false,

            midi_device_name: "".to_string(),
        }
    }
}

impl UIState {
    // TODO: rename related to choosing a loop
    // TODO: investigate a fix for the clippy error. Changing to a slice threw errors elsewhere in code.
    #[allow(clippy::ptr_arg)]
    pub fn selector_vec(mut self, selector_vec: &Vec<String>) -> Self {
        self.selector_vec = selector_vec.clone();
        self
    }

    pub fn set_selected_idx(&mut self, idx: usize) {
        self.selected_idx = idx;
    }

    pub fn set_is_playing(&mut self, is_playing: bool) {
        self.is_playing = is_playing;
    }

    pub fn set_current_beat(&mut self, beat: f64) {
        self.current_beat = beat as f32;
    }

    pub fn set_current_loop(&mut self, val: usize) {
        self.current_loop = val;
    }

    pub fn set_bpm(&mut self, bpm: f32) {
        self.bpm = bpm;
    }

    pub fn set_beats_per_loop(&mut self, value: usize) {
        self.beats_per_loop = value;
    }

    pub fn set_audio_latency_s(&mut self, offset: f32) {
        self.latency_offset_s = offset;
    }

    pub fn set_user_hits(&mut self, hits: &[UserHit]) {
        self.user_hits = hits.to_vec().clone();
    }

    pub fn set_desired_hits(&mut self, voices: &Voices) {
        self.desired_hits = voices.clone();
    }

    pub fn set_metronome_enabled(&mut self, enabled: bool) {
        self.is_metronome_enabled = enabled;
    }

    pub fn get_audio_latency_in_beats(&self) -> f32 {
        let beats_per_second = self.bpm / 60.;
        self.latency_offset_s * beats_per_second
    }

    pub fn set_is_help_visible(&mut self, val: bool) {
        self.is_help_visible = val;
    }

    pub fn set_is_dev_tools_visible(&mut self, enabled: bool) {
        self.is_dev_tools_visible = enabled;
    }

    pub fn set_are_side_panels_visible(&mut self, visible: bool) {
        self.are_side_panels_visible = visible;
    }

    pub fn set_correct_margin(&mut self, val: f64) {
        self.correct_margin = val;
    }

    pub fn set_miss_margin(&mut self, val: f64) {
        self.miss_margin = val;
    }

    pub fn set_hide_empty_tracks(&mut self, val: bool) {
        self.hide_empty_tracks = val;
    }

    pub fn set_midi_device_name(&mut self, val: &str) {
        self.midi_device_name = val.to_owned();
    }
}

pub fn layout_ui(ctx: &egui::Context, ui_state: &UIState, events: &mut Vec<Events>) {
    // make everything bigger, so text is legible
    ctx.set_pixels_per_point(2.0);

    dev_tools(ctx, ui_state, events);

    draw_top_panel(ctx, ui_state, events);

    draw_left_panel(ctx, ui_state, events);

    draw_right_panel(ctx, ui_state, events);

    draw_central_panel(ctx, ui_state, events);

    help_window(ctx, ui_state);
}

fn help_window(ctx: &egui::Context, ui_state: &UIState) {
    if !ui_state.is_help_visible {
        return;
    }

    egui::Window::new("Help").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.label("Show Help");
            ui.label("?");
        });
        ui.horizontal(|ui| {
            ui.label("Show Dev Tools");
            ui.label("a");
        });
        ui.horizontal(|ui| {
            ui.label("Show FPS");
            ui.label("z");
        });
    });
}

fn dev_tools(ctx: &egui::Context, ui_state: &UIState, events: &mut Vec<Events>) {
    if !ui_state.is_dev_tools_visible {
        return;
    }

    egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.heading("Dev Tools");
        });
        CollapsingHeader::new("Accuracy")
            .default_open(true)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Correct Margin");
                    let mut local_correct_margin = ui_state.correct_margin;
                    let correct_margin_widget = egui::DragValue::new(&mut local_correct_margin);
                    let correct_margin_widget_resp = correct_margin_widget.ui(ui);
                    if correct_margin_widget_resp.changed() {
                        events.push(Events::SetCorrectMargin(local_correct_margin));
                    }

                    ui.label("Miss Margin");
                    let mut local_miss_margin = ui_state.miss_margin;
                    let miss_margin_widget = egui::DragValue::new(&mut local_miss_margin);
                    let miss_margin_widget_resp = miss_margin_widget.ui(ui);
                    if miss_margin_widget_resp.changed() {
                        events.push(Events::SetMissMargin(local_miss_margin));
                    }
                });
            });
    });
}

fn draw_top_panel(ctx: &egui::Context, ui_state: &UIState, events: &mut Vec<Events>) {
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            let button_text = match ui_state.is_playing {
                true => "Pause",
                false => "Play",
            };
            if ui.button(button_text).clicked() {
                events.push(Events::Pause);
            }

            ui.separator();

            ui.add(egui::Label::new("BPM"));

            let mut local_bpm = ui_state.bpm;
            let bpm_slider = egui::Slider::new(&mut local_bpm, 40.0..=240.0);
            let bpm_slider_resp = bpm_slider.ui(ui);
            if bpm_slider_resp.changed() {
                events.push(Events::SetBPM(local_bpm as f64));
            }
            if ui.button("-").clicked() {
                events.push(Events::ChangeBPM { delta: -1. });
            }
            if ui.button("+").clicked() {
                events.push(Events::ChangeBPM { delta: 1. });
            }

            ui.separator();

            let selector_text = if !ui_state.selector_vec.is_empty() {
                &ui_state.selector_vec[ui_state.selected_idx]
            } else {
                "No loops"
            };
            egui::ComboBox::from_label("")
                .selected_text(selector_text.to_string())
                .show_ui(ui, |ui| {
                    for i in 0..ui_state.selector_vec.len() {
                        let mut current_value = &ui_state.selector_vec[i];
                        let value = ui.selectable_value(
                            &mut current_value,
                            &ui_state.selector_vec[ui_state.selected_idx],
                            &ui_state.selector_vec[i],
                        );
                        if value.clicked() {
                            events.push(Events::ChangeLoop(i));
                        }
                    }
                });

            ui.separator();

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("Advanced View").clicked() {
                    events.push(Events::ToggleSidePanelVisibility);
                }
            });
        });
    });
}

fn draw_left_panel(ctx: &egui::Context, ui_state: &UIState, events: &mut Vec<Events>) {
    if !ui_state.are_side_panels_visible {
        return;
    }

    egui::SidePanel::left("left_panel")
        .resizable(true)
        .default_width(150.0)
        .width_range(80.0..=240.0)
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Left Panel");
            });

            ui.separator();

            ui.add(egui::Label::new("**Volume**"));
            // TODO: improve volume controls (https://github.com/nathanleiby/drum-break/issues/11)
            ui.add(egui::Label::new("Metronome"));
            let button_text = match ui_state.is_metronome_enabled {
                true => "Disable Metronome",
                false => "Enable Metronome",
            };
            if ui.button(button_text).clicked() {
                events.push(Events::ToggleMetronome);
            }

            ui.separator();

            ui.add(egui::Label::new("**Loop Status**"));
            ui.add(egui::Label::new("Current Loop"));
            ui.add(egui::Label::new(format!("{}", ui_state.current_loop)));
            ui.add(egui::Label::new("Current Beat"));
            ui.add(egui::Label::new(format!("{}", ui_state.current_beat)));

            ui.separator();

            gold_mode(ui, ui_state);
        });
}

fn draw_right_panel(ctx: &egui::Context, ui_state: &UIState, events: &mut Vec<Events>) {
    if !ui_state.are_side_panels_visible {
        return;
    }

    egui::SidePanel::right("right_panel")
        .resizable(true)
        .default_width(150.0)
        .width_range(80.0..=240.0)
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Right Panel");
            });

            ui.group(|ui| {
                ui.add(egui::Label::new("Latency Offset"));
                ui.label(format!("{:?}", ui_state.latency_offset_s));
                if ui.button("-").clicked() {
                    events.push(Events::SetAudioLatency { delta_s: -0.1 });
                }
                if ui.button("+").clicked() {
                    events.push(Events::SetAudioLatency { delta_s: 0.1 });
                }
            });

            ui.separator();
            egui::widgets::global_dark_light_mode_buttons(ui);

            ui.separator();

            ui.add(egui::Label::new("**UI**"));
            let button_text = match ui_state.hide_empty_tracks {
                true => "Show Empty Tracks",
                false => "Hide Empty Tracks",
            };
            if ui.button(button_text).clicked() {
                events.push(Events::ToggleEmptyTrackVisibility);
            }

            ui.separator();

            ui.group(|ui| {
                ui.add(egui::Label::new("**MIDI**"));
                ui.add(egui::Label::new(format!(
                    "attached device: {}",
                    ui_state.midi_device_name
                )));

                if ui.button("Refresh Connected Midi Device").clicked() {
                    events.push(Events::RefreshConnectedMidiDevice);
                }
            });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.add(egui::Hyperlink::from_label_and_url(
                    "Source code.",
                    "https://github.com/nathanleiby/drum-break",
                ));
                egui::warn_if_debug_build(ui);
            });
        });
}

fn draw_central_panel(ctx: &egui::Context, ui_state: &UIState, events: &mut Vec<Events>) {
    egui::CentralPanel::default().show(ctx, |ui| {
        draw_beat_grid(ui_state, ui, events);
    });
}

const VIRTUAL_WIDTH: f32 = 800.;
const VIRTUAL_HEIGHT: f32 = 1000.;

fn draw_beat_grid(ui_state: &UIState, ui: &mut egui::Ui, events: &mut Vec<Events>) {
    let (response, painter) = ui.allocate_painter(
        egui::Vec2::new(ui.available_width(), ui.available_height()),
        egui::Sense::hover(),
    );

    let to_screen = emath::RectTransform::from_to(
        egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::Vec2::new(VIRTUAL_WIDTH, VIRTUAL_HEIGHT),
        ),
        response.rect,
    );
    let from_screen = emath::RectTransform::from_to(
        response.rect,
        egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::Vec2::new(VIRTUAL_WIDTH, VIRTUAL_HEIGHT),
        ),
    );

    let visible_instruments: Vec<&Instrument> = if ui_state.hide_empty_tracks {
        ALL_INSTRUMENTS
            .iter()
            .enumerate()
            .filter(|(_, ins)| !ui_state.desired_hits.get_instrument_beats(ins).is_empty())
            .map(|(_, ins)| ins)
            .collect()
    } else {
        ALL_INSTRUMENTS.iter().filter(|_| true).collect()
    };

    let visible_rows = visible_instruments.len();
    let visible_cols = ui_state.beats_per_loop;

    let width_scale: f32 = VIRTUAL_WIDTH / visible_cols as f32;
    let height_scale: f32 = VIRTUAL_HEIGHT / visible_rows as f32;

    // capture mouse clicks and toggle relevant beat
    ui.input(|i| {
        for event in &i.raw.events {
            if let egui::Event::PointerButton {
                pos, pressed: true, ..
            } = event
            {
                // check if click is within the beat grid's bounds
                if !response.rect.contains(*pos) {
                    continue;
                }

                // Translate to (row, col)
                let tpos = from_screen.transform_pos(*pos);
                let row = (tpos.y * visible_rows as f32 / VIRTUAL_HEIGHT) as usize;
                let col = (tpos.x * visible_cols as f32 / VIRTUAL_WIDTH) as usize;
                info!(
                    "click at position = {:?} [[tpos = {:?}]] (row={:?}, col={:?})",
                    pos, tpos, row, col,
                );

                // map from UI display to instrument
                let res = visible_instruments.iter().enumerate().find(|x| x.0 == row);
                let ins = match res {
                    Some(x) => *x.1,
                    None => panic!("invalid instrument idx"),
                };
                events.push(Events::ToggleBeat {
                    ins: *ins,
                    beat: col as f64,
                });
            }
        }
    });

    let mut shapes = vec![];

    draw_background(to_screen, &mut shapes);

    draw_zebra_stripes(visible_rows, height_scale, to_screen, &mut shapes);

    draw_vertical_lines(visible_cols, width_scale, to_screen, &mut shapes);

    draw_horizontal_lines(visible_rows, height_scale, to_screen, &mut shapes);

    // Draw Note Successes
    let loop_last_completed_beat = ui_state.current_beat - MISS_MARGIN as f32;
    let current_loop_hits = get_hits_from_nth_loop(
        &ui_state.user_hits,
        ui_state.current_loop,
        ui_state.beats_per_loop,
    );
    draw_note_successes(
        &current_loop_hits,
        &ui_state.desired_hits,
        ui_state.get_audio_latency_in_beats() as f64,
        loop_last_completed_beat as f64,
        to_screen,
        &mut shapes,
        width_scale,
        height_scale,
        &visible_instruments,
        ui_state.beats_per_loop,
    );

    // Draw User Hits
    draw_user_hits(
        ui_state,
        to_screen,
        &mut shapes,
        height_scale,
        &visible_instruments,
    );

    draw_current_beat(
        ui_state.current_beat + ui_state.get_audio_latency_in_beats(),
        to_screen,
        ui,
        &mut shapes,
        ui_state.beats_per_loop,
    );

    // render them
    painter.extend(shapes);

    // add instrument names last, so they stay visible
    for (row, item) in visible_instruments.iter().enumerate().take(visible_rows) {
        let name = match item {
            Instrument::ClosedHihat => "Hi-hat",
            Instrument::Snare => "Snare",
            Instrument::Kick => "Kick",
            Instrument::OpenHihat => "Open Hi-hat",
            Instrument::Ride => "Ride",
            Instrument::Crash => "Crash",
            Instrument::Tom1 => "Tom1 (High)",
            Instrument::Tom2 => "Tom2 (Med)",
            Instrument::Tom3 => "Tom3 (Low)",
            Instrument::PedalHiHat => "Pedal Hi-hat",
        };
        // TODO: align text elsewhere
        let t_rect = rect_for_col_row(0., row, to_screen, width_scale, height_scale);
        let label = egui::Label::new(name);
        ui.put(t_rect, label);
    }
}

fn draw_background(to_screen: RectTransform, shapes: &mut Vec<Shape>) {
    let bg_rect = egui::Shape::rect_filled(
        to_screen.transform_rect(egui::Rect {
            min: pos2(0., 0.),
            max: pos2(VIRTUAL_WIDTH, VIRTUAL_HEIGHT),
        }),
        egui::Rounding::default(),
        Color32::LIGHT_BLUE,
    );
    shapes.push(bg_rect);
}

/// Draw horizontal "zebra" stripes for row legibility
fn draw_zebra_stripes(
    visible_rows: usize,
    height_scale: f32,
    to_screen: RectTransform,
    shapes: &mut Vec<Shape>,
) {
    const LIGHTER_BLUE: Color32 = Color32::from_rgb(163, 206, 220);
    for visible_row in 0..visible_rows {
        if visible_row % 2 == 0 {
            continue;
        }

        let base_pos = pos2(0., (visible_row as f32) * height_scale);
        let start_pt = to_screen.transform_pos(base_pos);
        let end_pt =
            to_screen.transform_pos(base_pos + egui::Vec2::new(VIRTUAL_WIDTH, 1. * height_scale));
        let shape = egui::Shape::rect_filled(
            egui::Rect::from_two_pos(start_pt, end_pt),
            egui::Rounding::default(),
            LIGHTER_BLUE,
        );
        shapes.push(shape);
    }
}

fn draw_vertical_lines(
    visible_cols: usize,
    width_scale: f32,
    to_screen: RectTransform,
    shapes: &mut Vec<Shape>,
) {
    for col in 0..visible_cols {
        let base_pos = pos2((col as f32) * width_scale, 0.);
        let start_pt = to_screen.transform_pos(base_pos);
        let end_pt = to_screen.transform_pos(base_pos + egui::Vec2::new(0., VIRTUAL_HEIGHT));

        let shape = egui::Shape::line(
            vec![start_pt, end_pt],
            egui::Stroke::new(if col % 4 == 0 { 2. } else { 0.5 }, Color32::DARK_GRAY),
        );
        shapes.push(shape);
    }
}

/// Draw a horizontal line through the middle of each instrument's track
fn draw_horizontal_lines(
    visible_rows: usize,
    height_scale: f32,
    to_screen: RectTransform,
    shapes: &mut Vec<Shape>,
) {
    for visible_row in 0..visible_rows {
        let base_pos = pos2(0., (visible_row as f32 + 0.5) * height_scale);
        let start_pt = to_screen.transform_pos(base_pos);
        let end_pt = to_screen.transform_pos(base_pos + egui::Vec2::new(VIRTUAL_WIDTH, 0.));
        let shape = egui::Shape::line(
            vec![start_pt, end_pt],
            egui::Stroke::new(1., Color32::DARK_GRAY),
        );
        shapes.push(shape);
    }
}

fn rect_for_col_row(
    beat: f64,
    row: usize,
    to_screen: RectTransform,
    width_scale: f32,
    height_scale: f32,
) -> egui::Rect {
    let base_pos = pos2(beat as f32 * width_scale, row as f32 * height_scale);

    let percent_padding = 0.02;
    to_screen.transform_rect(egui::Rect {
        min: base_pos
            + egui::Vec2::new(
                width_scale * percent_padding,
                height_scale * percent_padding,
            ),
        max: base_pos
            + egui::Vec2::new(
                width_scale * (1. - percent_padding),
                height_scale * (1. - percent_padding),
            ),
    })
}

fn draw_current_beat(
    current_beat: f32,
    to_screen: RectTransform,
    ui: &egui::Ui,
    shapes: &mut Vec<Shape>,
    beats_per_loop: usize,
) {
    let base_pos = pos2((current_beat / (beats_per_loop as f32)) * VIRTUAL_WIDTH, 0.);
    let t_rect = to_screen.transform_rect(egui::Rect {
        min: base_pos,
        max: base_pos + egui::Vec2::new(2., VIRTUAL_HEIGHT),
    });

    let bar_color = if ui.visuals().dark_mode {
        Color32::YELLOW
    } else {
        Color32::BLUE
    };
    let shape = egui::Shape::rect_filled(t_rect, egui::Rounding::default(), bar_color);
    shapes.push(shape);
}

fn draw_user_hits(
    ui_state: &UIState,
    to_screen: RectTransform,
    shapes: &mut Vec<Shape>,
    height_scale: f32,
    visible_instruments: &[&Instrument],
) {
    for (instrument_idx, instrument) in visible_instruments.iter().enumerate() {
        let user_notes = get_user_hit_timings_by_instrument(
            &ui_state.user_hits,
            **instrument,
            ui_state.beats_per_loop,
        );
        let desired_notes = ui_state.desired_hits.get_instrument_beats(instrument);
        for note in user_notes.iter() {
            draw_user_hit(
                *note,
                instrument_idx,
                ui_state.get_audio_latency_in_beats() as f64,
                desired_notes,
                to_screen,
                shapes,
                height_scale,
                ui_state.beats_per_loop,
            );
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn draw_user_hit(
    user_beat: f64,
    row: usize,
    audio_latency_beats: f64,
    desired_hits: &[f64],
    to_screen: RectTransform,
    shapes: &mut Vec<Shape>,
    height_scale: f32,
    beats_per_loop: usize,
) {
    let user_beat_with_latency = user_beat + audio_latency_beats;

    let (acc, is_next_loop) =
        compute_accuracy_of_single_hit(user_beat_with_latency, desired_hits, beats_per_loop);

    // with audio latency and is_next_loop
    // TODO(bug): hit a note on every beat of 16. Then toggle on and off a note on only beat 1 for that instrument. it causes buggy display of hit timings where the 2nd half (beats 9-16) aren't shown .. bercause it's closer to beat 1 than any other beat, I guess?.
    // TODO(ui): can't see "before" hits because there's no space to left anymore
    let beats_per_loop = beats_per_loop as f32;
    let user_beat_with_latency = user_beat_with_latency as f32;
    let x = if is_next_loop {
        ((user_beat_with_latency - beats_per_loop) / beats_per_loop) * VIRTUAL_WIDTH
    } else {
        (user_beat_with_latency / beats_per_loop) * VIRTUAL_WIDTH
    };

    let base_pos = pos2(x, row as f32 * height_scale);
    let t_rect = to_screen.transform_rect(egui::Rect {
        min: base_pos,
        max: base_pos + egui::Vec2::new(2., height_scale * 0.95),
    });

    let bar_color = match acc {
        Accuracy::Early => ORANGE,
        Accuracy::Late => PURPLE,
        Accuracy::Correct => GREEN,
        Accuracy::Miss => RED,
        Accuracy::Unknown => LIGHTGRAY,
    };
    let bar_color_32 = Color32::from_rgb(
        (bar_color.r * 256.) as u8,
        (bar_color.g * 256.) as u8,
        (bar_color.b * 256.) as u8,
    );

    let shape = egui::Shape::rect_filled(t_rect, egui::Rounding::default(), bar_color_32);
    shapes.push(shape);
}

#[allow(clippy::too_many_arguments)]
fn draw_note_successes(
    user_hits: &[UserHit],
    desired_hits: &Voices,
    audio_latency: f64,
    loop_current_beat: f64,
    to_screen: RectTransform,
    shapes: &mut Vec<Shape>,
    width_scale: f32,
    height_scale: f32,
    visible_instruments: &[&Instrument],
    beats_per_loop: usize,
) {
    for (instrument_idx, instrument) in visible_instruments.iter().enumerate() {
        let actual = get_user_hit_timings_by_instrument(user_hits, **instrument, beats_per_loop);
        // add audio_latency to each note
        let actual_w_latency = actual
            .iter()
            .map(|note| note + audio_latency)
            .collect::<Vec<f64>>();

        let desired = desired_hits.get_instrument_beats(instrument);

        let loop_perf = compute_loop_performance_for_voice(
            &actual_w_latency,
            desired,
            loop_current_beat,
            beats_per_loop,
        );
        for (note_idx, note) in desired.iter().enumerate() {
            let shape = note_success_shape(
                *note,
                instrument_idx,
                loop_perf[note_idx],
                to_screen,
                width_scale,
                height_scale,
            );
            shapes.push(shape);
        }
    }
}

fn note_success_shape(
    beats_offset: f64,
    row: usize,
    acc: Accuracy,
    to_screen: RectTransform,
    width_scale: f32,
    height_scale: f32,
) -> Shape {
    let rect = rect_for_col_row(beats_offset, row, to_screen, width_scale, height_scale);

    let bar_color = match acc {
        Accuracy::Early => ORANGE,
        Accuracy::Late => PURPLE,
        Accuracy::Correct => GREEN,
        Accuracy::Miss => RED,
        Accuracy::Unknown => DARKBLUE,
    };
    let bar_color_32 = Color32::from_rgb(
        (bar_color.r * 256.) as u8,
        (bar_color.g * 256.) as u8,
        (bar_color.b * 256.) as u8,
    );

    egui::Shape::rect_filled(rect, egui::Rounding::default().at_least(5.0), bar_color_32)
}

fn gold_mode(ui: &mut egui::Ui, ui_state: &UIState) {
    ui.add(egui::Label::new("**Gold Mode**"));

    // build up a string
    let mut s = String::new();

    let mut points: Vec<[f64; 2]> = vec![];
    for i in 1..=5 {
        let nth_loop_hits = get_hits_from_nth_loop(
            &ui_state.user_hits,
            (ui_state.current_loop as i32 - i) as usize, // TODO: check for overflow
            ui_state.beats_per_loop,
        );
        let summary_data = compute_last_loop_summary(
            &nth_loop_hits,
            &ui_state.desired_hits,
            ui_state.beats_per_loop,
        );

        // Simpler than chart.. TODO: support for colored emoji
        // 🔴
        // 🟠
        // 🟡
        // 🟢
        // ✅
        let ratio = summary_data.combined().score();
        s.push(if ratio == 1.0 {
            '✅'
        } else if ratio > 0.7 {
            '🟢'
        } else {
            '🔴'
        });
        points.push([i as f64, ratio * 100_f64]);
    }
    ui.add(egui::Label::new(s));

    // PLOT
    let line = Line::new(points)
        .color(Color32::from_rgb(100, 200, 100))
        // .style(self.line_style)
        .name("gold_mode");

    let plot = Plot::new("Gold Mode")
        // .legend(Legend::default())
        .show_axes([true, true])
        .include_y(0.0)
        .include_y(100.0)
        .allow_drag(false);

    plot.show(ui, |plot_ui| {
        plot_ui.line(line);
    });
}
