// mod app;

use egui::{self, emath, pos2, Color32, Widget};
// EguiContexts, EguiPlugin,
use egui_plot::{Legend, Line, Plot};
use log::info;

use crate::{
    events::Events,
    voices::{Instrument, Voices},
};

// fn main() {
//     App::new()
//         .init_resource::<GameState>()
//         .add_plugins(DefaultPlugins)
//         .add_plugins(EguiPlugin)
//         // Systems that create Egui widgets should be run during the `CoreSet::Update` set,
//         // or after the `EguiSet::BeginFrame` system (which belongs to the `CoreSet::PreUpdate` set).
//         .add_systems(Update, ui_example_system)
//         .run();
// }

pub const GRID_ROWS: usize = 10;
pub const GRID_COLS: usize = 16;
const BEATS_PER_LOOP: f32 = 16.;

// This resource holds information about the game:
pub struct UIState {
    selector_vec: Vec<String>,
    selected_idx: usize,

    is_playing: bool,
    bpm: f32,
    is_metronome_enabled: bool,
    volume_metronome: f32,
    volume_target_notes: f32,

    current_loop: usize,
    current_beat: f32,

    enabled_beats: [[bool; GRID_COLS]; GRID_ROWS],

    latency_offset: f32,
}

// impl Default for GameState {
//     // TODO: due to serialization, weird behavior can occur on app reload..
//     // Need to sepaerate out things that should save to disk (settings? user state that should resume?) vs things that need to reset or be overwritten.
//     // From debugging on web, running `localStorage.clear()` is sufficient to reset.
//     fn default() -> Self {
//         Self {
//             // Example stuff:
//             is_playing: false,

//             selector_vec: vec![
//                 // TODO: String::from() vs .. "".to_owned() ?
//                 String::from("Rock"),
//                 String::from("Breakbeat"),
//                 String::from("Samba"),
//             ],
//             selected_idx: 0,

//             current_loop: 2,
//             current_beat: 2.3,

//             bpm: 120.,

//             is_metronome_enabled: false,
//             volume_metronome: 0.75,
//             volume_target_notes: 0.75,

//             latency_offset: 100.,

//             enabled_beats: [[false; GRID_COLS]; GRID_ROWS],
//         }
//     }
// }

impl Default for UIState {
    // TODO: due to serialization, weird behavior can occur on app reload..
    // Need to sepaerate out things that should save to disk (settings? user state that should resume?) vs things that need to reset or be overwritten.
    // From debugging on web, running `localStorage.clear()` is sufficient to reset.
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

            bpm: 120.,

            is_metronome_enabled: false,
            volume_metronome: 0.75,
            volume_target_notes: 0.75,

            latency_offset: 100.,

            enabled_beats: [[false; GRID_COLS]; GRID_ROWS],
        }
    }
}

impl UIState {
    // TODO: rename related to choosing a loop
    pub fn selector_vec(mut self, selector_vec: Vec<String>) -> Self {
        self.selector_vec = selector_vec;
        self
    }

    pub fn set_is_playing(&mut self, is_playing: bool) {
        self.is_playing = is_playing;
    }

    pub fn set_current_beat(&mut self, beat: f64) {
        self.current_beat = beat as f32;
    }

    pub fn set_enabled_beats(&mut self, voices: &Voices) {
        self.enabled_beats = voices.to_enabled_beats();
    }

    pub fn set_bpm(&mut self, bpm: f32) {
        self.bpm = bpm;
    }
}

// fn ui_example_system(mut contexts: EguiContexts, mut game_state: ResMut<GameState>) {
// fn ui_example_system(mut contexts: EguiContexts, mut game_state: ResMut<GameState>) {
// let ctx = contexts.ctx_mut();

pub fn draw_ui(
    ctx: &egui_macroquad::egui::Context,
    ui_state: &mut UIState,
    events: &mut Vec<Events>,
) {
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        // The top panel is often a good place for a menu bar:

        egui::menu::bar(ui, |ui| {
            // NOTE: no File->Quit on web pages!
            let is_web = cfg!(target_arch = "wasm32");
            if !is_web {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        // ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        println!("quit! but with earlier EGui..");
                    }
                });
                ui.add_space(16.0);
            }
            ui.add(egui::Label::new("BPM"));

            let bpm_slider = egui::Slider::new(&mut ui_state.bpm, 40.0..=240.0);
            let bpm_slider_resp = bpm_slider.ui(ui);
            if bpm_slider_resp.changed() {
                events.push(Events::SetBPM(ui_state.bpm as f64));
            }
            if ui.button("-").clicked() {
                events.push(Events::ChangeBPM { delta: -1. });
                ui_state.bpm -= 1.;
            }
            if ui.button("+").clicked() {
                events.push(Events::ChangeBPM { delta: 1. });
                ui_state.bpm += 1.;
            }

            ui.separator();

            ui.add(
                // egui::ProgressBar::new(game_state.progress)
                egui::ProgressBar::new(ui_state.current_beat / BEATS_PER_LOOP)
                    // .fill(Color32::BROWN)
                    .show_percentage(),
            );
        });
    });

    egui::SidePanel::left("left_panel")
        .resizable(true)
        .default_width(150.0)
        .width_range(80.0..=240.0)
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Left Panel");
            });

            let button_text = match ui_state.is_playing {
                true => "Pause",
                false => "Play",
            };
            if ui.button(button_text).clicked() {
                ui_state.is_playing = !ui_state.is_playing;
            }

            ui.separator();

            ui.add(egui::Label::new("**Volume**"));
            ui.add(egui::Label::new("Metronome"));
            ui.add(egui::Slider::new(&mut ui_state.volume_metronome, 0.0..=1.0));
            ui.add(egui::Label::new("Target Notes"));
            ui.add(egui::Slider::new(
                &mut ui_state.volume_target_notes,
                0.0..=1.0,
            ));

            ui.separator();

            ui.add(egui::Label::new("**Loop Status**"));
            ui.add(egui::Label::new("Current Loop"));
            ui.add(egui::Label::new(format!("{}", ui_state.current_loop)));
            ui.add(egui::Label::new("Current Beat"));
            ui.add(egui::Label::new(format!("{}", ui_state.current_beat)));

            ui.separator();

            gold_mode(ui);
        });

    egui::SidePanel::right("right_panel")
        .resizable(true)
        .default_width(150.0)
        .width_range(80.0..=240.0)
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Right Panel");
            });

            egui::ComboBox::from_label("Choose Loop")
                .selected_text(format!("{}", &ui_state.selector_vec[ui_state.selected_idx]))
                .show_ui(ui, |ui| {
                    for i in 0..ui_state.selector_vec.len() {
                        let value = ui.selectable_value(
                            &mut &ui_state.selector_vec[i],
                            &ui_state.selector_vec[ui_state.selected_idx],
                            &ui_state.selector_vec[i],
                        );
                        if value.clicked() {
                            // TODO: handle with event
                            ui_state.selected_idx = i;
                            // TODO: load the relevant loop's data
                            events.push(Events::ChangeLoop(i));
                        }
                    }
                });

            ui.separator();

            ui.group(|ui| {
                ui.add(egui::Label::new("Latency Offset (ms)"));
                ui.add(egui::Slider::new(
                    &mut ui_state.latency_offset,
                    -1000.0..=1000.0,
                ));
                if ui.button("-").clicked() {
                    ui_state.latency_offset -= 5.;
                }
                if ui.button("+").clicked() {
                    ui_state.latency_offset += 5.;
                }
            });

            ui.separator();
            egui::widgets::global_dark_light_mode_buttons(ui);

            ui.separator();

            // TODO: link to macroix github repo
            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/main/",
                "Source code."
            ));

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });

    egui::CentralPanel::default().show(ctx, |ui| {
        // The central panel the region left after adding TopPanel's and SidePanel's
        ui.heading("Macroix");

        draw_beat_grid(ui_state, ui, events);
    });
}

const VIRTUAL_WIDTH: f32 = 800.;
const VIRTUAL_HEIGHT: f32 = 1000.;

// TODO: convert mouse click to beat

fn draw_beat_grid(ui_state: &mut UIState, ui: &mut egui::Ui, events: &mut Vec<Events>) {
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

    ui.input(|i| {
        for event in &i.raw.events {
            match event {
                // TODO: what is this syntax
                egui::Event::PointerButton {
                    pos, pressed: true, ..
                } => {
                    // check if click is within the beat grid's bounds
                    if !response.rect.contains(*pos) {
                        continue;
                    }

                    // Translate to (row, col)
                    let tpos = from_screen.transform_pos(*pos);
                    let row = (tpos.y * GRID_ROWS as f32 / VIRTUAL_HEIGHT) as usize;
                    let col = (tpos.x * GRID_COLS as f32 / VIRTUAL_WIDTH) as usize;
                    info!(
                        "click at position = {:?} [[tpos = {:?}]] (row={:?}, col={:?})",
                        pos, tpos, row, col,
                    );
                    // TODO: emit an event to enable this beat
                    events.push(Events::ToggleBeat {
                        row: row as f64,
                        beat: col as f64,
                    });
                    // ui_state.enabled_beats[row][col] = !ui_state.enabled_beats[row][col];
                }
                _ => (),
            }
        }
    });

    // TODO: Add instrument names
    // for (instrument_idx, instrument) in ALL_INSTRUMENTS.iter().enumerate() {
    //     let name = match *instrument {
    //         Instrument::ClosedHihat => "Hi-hat",
    //         Instrument::Snare => "Snare",
    //         Instrument::Kick => "Kick",
    //         Instrument::OpenHihat => "Open Hi-hat",
    //         Instrument::Ride => "Ride",
    //         Instrument::Crash => "Crash",
    //         Instrument::Tom1 => "Tom1 (High)",
    //         Instrument::Tom2 => "Tom2 (Med)",
    //         Instrument::Tom3 => "Tom3 (Low)",
    //         Instrument::PedalHiHat => "Pedal Hi-hat",
    //     };

    //     // Labels in top-left of grid
    //     draw_text(
    //         name,
    //         20.0,
    //         (GRID_TOP_Y + ROW_HEIGHT * (instrument_idx as f64 + 0.5)) as f32,
    //         20.0,
    //         DARKGRAY,
    //     );

    //     let desired = desired_hits.get_instrument_beats(instrument);
    //     for note in desired.iter() {
    //         draw_note(*note, instrument_idx);
    //     }
    // }

    let mut shapes = vec![];
    let width_scale = VIRTUAL_WIDTH / GRID_COLS as f32;
    let height_scale = VIRTUAL_HEIGHT / GRID_ROWS as f32;
    for col in 0..GRID_COLS {
        for row in 0..GRID_ROWS {
            let base_pos = pos2(col as f32 * width_scale, row as f32 * height_scale);

            // TODO: fix scaling to always draw a nicer looking square based grid
            let t_rect = to_screen.transform_rect(egui::Rect {
                min: base_pos,
                max: base_pos + egui::Vec2::new(width_scale * 0.95, height_scale * 0.95),
            });

            // if this beat is enabled (row is instrument, col is beat)..
            if ui_state.enabled_beats[row][col] {
                let shape =
                    egui::Shape::rect_filled(t_rect, egui::Rounding::default(), Color32::GRAY);
                shapes.push(shape)
            }

            let shape = egui::Shape::rect_stroke(
                t_rect,
                egui::Rounding::default(),
                egui::Stroke::new(2., Color32::DARK_GRAY),
            );
            shapes.push(shape);
        }
    }

    let base_pos = pos2((ui_state.current_beat / BEATS_PER_LOOP) * VIRTUAL_WIDTH, 0.);
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

    // render them
    painter.extend(shapes);
}

fn gold_mode(ui: &mut egui::Ui) {
    ui.add(egui::Label::new("**Gold Mode**"));

    // Simpler than chart..
    // 🔴
    // 🟠
    // 🟡
    // 🟢
    // ✅

    // convert scopes to summary

    // https://www.egui.rs/#demo check out FontBook to see supported black and white emoji
    // Can use image instead to get colors
    ui.add(egui::Label::new("✅🟢🟢🟢🟡🟠🟡🔴🔴"));

    // PLOT
    // let points = vec![[0., 0.7], [1., 0.5], [2., 0.3], [3., 0.1], [4., 0.0]];
    // let line = Line::new(points)
    //     .color(Color32::from_rgb(100, 200, 100))
    //     // .style(self.line_style)
    //     .name("gold_mode");

    // let plot = Plot::new("lines_demo")
    //     .legend(Legend::default())
    //     .show_axes(true)
    //     .show_grid(true);

    // plot.show(ui, |plot_ui| {
    //     plot_ui.line(line);
    // });
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
