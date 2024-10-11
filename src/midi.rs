/*
 Connect to midi device. Process midi input.

 Thanks to: https://github.com/TanTanDev/midi_game
*/

use log::info;
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
                        let midi_function = message[0];
                        let note_number = message[1];
                        let v = MidiInputDataRaw {
                            note_number,
                            timestamp: stamp,
                            non_midi_timestamp_ms,
                            status: midi_function,
                            note_velocity: message[2],
                        };
                        info!("{}: {:?} (len = {})", stamp, v, message.len());
                        info!("{}", MIDI_FUNCTION_NAMES[midi_function as usize - 128]);
                        let mut rw: std::sync::MutexGuard<HashMap<u8, MidiInputDataRaw>> =
                            raw_inputs.lock().unwrap();
                        rw.insert(note_number, v);
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

// Midi Spec

// from 128-255, these are the functions corresponding to a Midi Note's 1st byte
const MIDI_FUNCTION_NAMES: [&str; 128] = [
    "Chan 1 Note off",
    "Chan 2 Note off",
    "Chan 3 Note off",
    "Chan 4 Note off",
    "Chan 5 Note off",
    "Chan 6 Note off",
    "Chan 7 Note off",
    "Chan 8 Note off",
    "Chan 9 Note off",
    "Chan 10 Note off",
    "Chan 11 Note off",
    "Chan 12 Note off",
    "Chan 13 Note off",
    "Chan 14 Note off",
    "Chan 15 Note off",
    "Chan 16 Note off",
    "Chan 1 Note on",
    "Chan 2 Note on",
    "Chan 3 Note on",
    "Chan 4 Note on",
    "Chan 5 Note on",
    "Chan 6 Note on",
    "Chan 7 Note on",
    "Chan 8 Note on",
    "Chan 9 Note on",
    "Chan 10 Note on",
    "Chan 11 Note on",
    "Chan 12 Note on",
    "Chan 13 Note on",
    "Chan 14 Note on",
    "Chan 15 Note on",
    "Chan 16 Note on",
    "Chan 1 Polyphonic Aftertouch",
    "Chan 2 Polyphonic Aftertouch",
    "Chan 3 Polyphonic Aftertouch",
    "Chan 4 Polyphonic Aftertouch",
    "Chan 5 Polyphonic Aftertouch",
    "Chan 6 Polyphonic Aftertouch",
    "Chan 7 Polyphonic Aftertouch",
    "Chan 8 Polyphonic Aftertouch",
    "Chan 9 Polyphonic Aftertouch",
    "Chan 10 Polyphonic Aftertouch",
    "Chan 11 Polyphonic Aftertouch",
    "Chan 12 Polyphonic Aftertouch",
    "Chan 13 Polyphonic Aftertouch",
    "Chan 14 Polyphonic Aftertouch",
    "Chan 15 Polyphonic Aftertouch",
    "Chan 16 Polyphonic Aftertouch",
    "Chan 1 Control/Mode Change",
    "Chan 2 Control/Mode Change",
    "Chan 3 Control/Mode Change",
    "Chan 4 Control/Mode Change",
    "Chan 5 Control/Mode Change",
    "Chan 6 Control/Mode Change",
    "Chan 7 Control/Mode Change",
    "Chan 8 Control/Mode Change",
    "Chan 9 Control/Mode Change",
    "Chan 10 Control/Mode Change",
    "Chan 11 Control/Mode Change",
    "Chan 12 Control/Mode Change",
    "Chan 13 Control/Mode Change",
    "Chan 14 Control/Mode Change",
    "Chan 15 Control/Mode Change",
    "Chan 16 Control/Mode Change",
    "Chan 1 Program Change",
    "Chan 2 Program Change",
    "Chan 3 Program Change",
    "Chan 4 Program Change",
    "Chan 5 Program Change",
    "Chan 6 Program Change",
    "Chan 7 Program Change",
    "Chan 8 Program Change",
    "Chan 9 Program Change",
    "Chan 10 Program Change",
    "Chan 11 Program Change",
    "Chan 12 Program Change",
    "Chan 13 Program Change",
    "Chan 14 Program Change",
    "Chan 15 Program Change",
    "Chan 16 Program Change",
    "Chan 1 Channel Aftertouch",
    "Chan 2 Channel Aftertouch",
    "Chan 3 Channel Aftertouch",
    "Chan 4 Channel Aftertouch",
    "Chan 5 Channel Aftertouch",
    "Chan 6 Channel Aftertouch",
    "Chan 7 Channel Aftertouch",
    "Chan 8 Channel Aftertouch",
    "Chan 9 Channel Aftertouch",
    "Chan 10 Channel Aftertouch",
    "Chan 11 Channel Aftertouch",
    "Chan 12 Channel Aftertouch",
    "Chan 13 Channel Aftertouch",
    "Chan 14 Channel Aftertouch",
    "Chan 15 Channel Aftertouch",
    "Chan 16 Channel Aftertouch",
    "Chan 1 Pitch Bend Change",
    "Chan 2 Pitch Bend Change",
    "Chan 3 Pitch Bend Change",
    "Chan 4 Pitch Bend Change",
    "Chan 5 Pitch Bend Change",
    "Chan 6 Pitch Bend Change",
    "Chan 7 Pitch Bend Change",
    "Chan 8 Pitch Bend Change",
    "Chan 9 Pitch Bend Change",
    "Chan 10 Pitch Bend Change",
    "Chan 11 Pitch Bend Change",
    "Chan 12 Pitch Bend Change",
    "Chan 13 Pitch Bend Change",
    "Chan 14 Pitch Bend Change",
    "Chan 15 Pitch Bend Change",
    "Chan 16 Pitch Bend Change",
    "System Exclusive",
    "MIDI Time Code Qtr. Frame",
    "Song Position Pointer",
    "Song Select (Song #)",
    "Undefined (Reserved)",
    "Undefined (Reserved)",
    "Tune request",
    "End of SysEx (EOX)",
    "Timing clock",
    "Undefined (Reserved)",
    "Start",
    "Continue",
    "Stop",
    "Undefined (Reserved)",
    "Active Sensing",
    "System Reset",
];
