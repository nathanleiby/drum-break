// Thanks to: https://github.com/TanTanDev/midi_game

use midir;
use std::collections::HashMap;
use std::string::*;
use std::sync::{Arc, Mutex};

use crate::time::current_time_millis;

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
pub struct MidiInputDataRaw {
    pub note_number: u8,
    pub timestamp: u64,
    pub non_midi_timestamp_ms: u128,
    // https://www.logosfoundation.org/kursus/1075.html
    status: u8,
    note_velocity: u8,
}

impl MidiInputDataRaw {
    pub fn is_note_on(&self) -> bool {
        self.status >= 144 && self.status <= 159
    }
}

impl MidiInput {
    pub fn new() -> Option<Self> {
        let midi_input = midir::MidiInput::new("Input device").unwrap();
        // grab first device
        let input_port = match midi_input.ports().into_iter().next() {
            Some(port) => port,
            None => return None,
        };

        let device_name = midi_input
            .port_name(&input_port)
            .expect("can't get name of port");

        Some(Self {
            midi_input: Some(midi_input),
            input_port,
            device_name,
            connection: None,
            raw_inputs: Arc::new(Mutex::new(HashMap::with_capacity(16))),
            previous_raw_inputs: Arc::new(Mutex::new(HashMap::with_capacity(16))),
        })
    }

    pub fn get_pressed_buttons(&self) -> Vec<MidiInputDataRaw> {
        let mut pressed = Vec::new();
        let mut raw_inputs = self.raw_inputs.lock().unwrap();
        for (_id, raw_input) in raw_inputs.iter_mut() {
            if raw_input.is_note_on() {
                pressed.push(*raw_input);
            }
        }
        if pressed.len() > 0 {
            log::info!("Pressed midi: {:?}", pressed);
        }
        pressed
    }

    // clear all inputs, update previous values
    pub fn flush(&mut self) {
        let mut prev_raw_inputs = self.previous_raw_inputs.lock().unwrap();
        let mut raw_inputs = self.raw_inputs.lock().unwrap();
        // store latests values as previous
        for (id, raw_input) in raw_inputs.iter_mut() {
            if let Some(prev_raw) = prev_raw_inputs.get_mut(&id) {
                *prev_raw = *raw_input;
            } else {
                prev_raw_inputs.insert(*id, *raw_input);
            }
        }
        raw_inputs.clear();
    }

    pub fn connect(&mut self) {
        log::info!("Connecting to midi device: {}", self.device_name);
        let raw_inputs = self.raw_inputs.clone();
        self.connection = Some(
            self.midi_input
                .take() // consume midi_input because it will be sent to thread
                .unwrap()
                .connect(
                    &self.input_port,
                    self.device_name.as_str(),
                    move |stamp, message, _| {
                        // get timestamp
                        let non_midi_timestamp_ms = current_time_millis();
                        println!("{}: {:?} (len = {})", stamp, message, message.len());
                        let mut rw = raw_inputs.lock().unwrap();
                        let note_number = message[1];
                        rw.insert(
                            note_number,
                            MidiInputDataRaw {
                                note_number,
                                timestamp: stamp,
                                non_midi_timestamp_ms,
                                status: message[0],
                                note_velocity: message[2],
                            },
                        );
                    },
                    (),
                )
                .expect("can't connect to midi device"),
        );
    }

    pub fn get_device_name(&self) -> &str {
        &self.device_name
    }
}
