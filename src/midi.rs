// Thanks to: https://github.com/TanTanDev/midi_game

use macroquad::miniquad::info;
use midir;
use std::collections::HashMap;
use std::string::*;
use std::sync::{Arc, Mutex};

pub struct MidiInput {
    input_port: midir::MidiInputPort,
    device_name: String,
    // optional because it needs to be consumed and sent to the connection thread
    midi_input: Option<midir::MidiInput>,
    connection: Option<midir::MidiInputConnection<()>>,

    raw_inputs: Arc<Mutex<HashMap<u8, MidiInputDataRaw>>>,
    previous_raw_inputs: Arc<Mutex<HashMap<u8, MidiInputDataRaw>>>,
}

#[derive(Eq, Clone, Debug, Copy, PartialEq)]
struct MidiInputDataRaw {
    timestamp: u64,
    value: u8,
}

impl MidiInput {
    pub fn new() -> Self {
        let midi_input = midir::MidiInput::new("Input device").unwrap();
        // grab first device
        let input_port = match midi_input.ports().into_iter().next() {
            Some(port) => port,
            None => panic!("NO MIDI DEVICE FOUND!"),
        };

        let device_name = midi_input
            .port_name(&input_port)
            .expect("can't get name of port");

        Self {
            midi_input: Some(midi_input),
            input_port,
            device_name,
            connection: None,
            raw_inputs: Arc::new(Mutex::new(HashMap::with_capacity(16))),
            previous_raw_inputs: Arc::new(Mutex::new(HashMap::with_capacity(16))),
        }
    }

    pub fn is_button_held(&self, id: u8) -> bool {
        let mut raw_inputs = self.raw_inputs.lock().unwrap();
        let mut previous_raw_inputs = self.previous_raw_inputs.lock().unwrap();
        if let Some(raw_input) = raw_inputs.get_mut(&id) {
            if let Some(previous_raw_input) = previous_raw_inputs.get_mut(&id) {
                raw_input.value == 127 && previous_raw_input.value == 127
            } else {
                raw_input.value == 127
            }
        } else {
            if let Some(previous_raw_input) = previous_raw_inputs.get_mut(&id) {
                previous_raw_input.value == 127
            } else {
                false
            }
        }
    }

    pub fn is_button_pressed(&self, id: u8) -> bool {
        let mut raw_inputs = self.raw_inputs.lock().unwrap();
        if let Some(raw_input) = raw_inputs.get_mut(&id) {
            raw_input.value == 127
        } else {
            false
        }
    }

    pub fn is_button_released(&self, id: u8) -> bool {
        !self.is_button_pressed(id)
    }

    pub fn get_pressed_buttons(&self) -> Vec<u8> {
        let mut pressed = Vec::new();
        let mut raw_inputs = self.raw_inputs.lock().unwrap();
        for (id, raw_input) in raw_inputs.iter_mut() {
            if raw_input.value == 127 {
                pressed.push(*id);
            }
        }
        pressed
    }

    // clear all inputs, update previous values
    pub fn flush(&mut self) {
        let mut prev_raw_inputs = self.previous_raw_inputs.lock().unwrap();
        let mut raw_inputs = self.raw_inputs.lock().unwrap();
        // store latests values as previous
        for (id, raw_input) in raw_inputs.iter_mut() {
            if let Some(mut prev_raw) = prev_raw_inputs.get_mut(&id) {
                *prev_raw = *raw_input;
            } else {
                prev_raw_inputs.insert(*id, *raw_input);
            }
        }
        raw_inputs.clear();
    }

    pub fn connect(&mut self) {
        info!("Connecting to midi device: {}", self.device_name);
        let raw_inputs = self.raw_inputs.clone();
        self.connection = Some(
            self.midi_input
                .take() // consume midi_input because it will be sent to thread
                .unwrap()
                .connect(
                    &self.input_port,
                    self.device_name.as_str(),
                    move |stamp, message, _| {
                        // println!("{}: {:?} (len = {})", stamp, message, message.len());
                        let mut rw = raw_inputs.lock().unwrap();
                        let identifier = message[1];
                        rw.insert(
                            identifier,
                            MidiInputDataRaw {
                                timestamp: stamp,
                                value: message[2],
                            },
                        );
                    },
                    (),
                )
                .expect("can't connect to midi device"),
        );
    }
}
