/*
  Capture user input from midi. convert it into events.

  This is stateful because it depends on setting up a connection to a midi input,
  and flushing the internally stored events after the have been consumed via process().
*/

use std::collections::HashSet;

use macroquad::prelude::*;

use crate::{
    consts::*, events::Events, midi::MidiInput, time::current_time_millis, voices::Instrument,
};

pub struct MidiInputHandler {
    midi_input: Option<MidiInput>,
}

impl MidiInputHandler {
    pub fn new() -> Self {
        let mut midi_input = MidiInput::new();
        match midi_input {
            Some(ref mut midi_input) => {
                midi_input.connect();
            }
            None => log::warn!("warning: no midi input device found"),
        }

        Self { midi_input }
    }

    /// convert any user input from the last frame into Events
    pub fn process(&mut self) -> Vec<Events> {
        let mut events: Vec<Events> = vec![];

        // TODO(future): get the current clock time AND audio clock time at the start of a frame, and use that for all downstream calcs
        let _now_ms = current_time_millis();
        if let Some(midi_input) = &mut self.midi_input {
            let hits = get_midi_as_user_hits(midi_input);

            // for each hit, calculate the processing delay and correct the clock time
            for hit in &hits {
                // let processing_delay_ms = now_ms - hit.clock_tick as u128;
                //// TODO: needs work
                let processing_delay_ms = 0;
                events.push(Events::UserHit {
                    instrument: hit.instrument,
                    processing_delay: processing_delay_ms as f64 / 1000.,
                })
            }

            // let processing_delay = now - ; // is this better called "input latency"?
            // let corrected_clock_time = current_clock_time - processing_delay;

            midi_input.flush();
        }

        events
    }
}

struct InputConfigMidi {
    kick: HashSet<u8>,
    snare: HashSet<u8>,
    closed_hi_hat: HashSet<u8>,
    open_hi_hat: HashSet<u8>,
    ride: HashSet<u8>,
    crash: HashSet<u8>,
    tom_1: HashSet<u8>,
    tom_2: HashSet<u8>,
    tom_3: HashSet<u8>,
    pedal_hihat: HashSet<u8>,
}

impl InputConfigMidi {
    pub fn get_note_numbers(&self, ins: &Instrument) -> &HashSet<u8> {
        match ins {
            Instrument::ClosedHihat => &self.closed_hi_hat,
            Instrument::Snare => &self.snare,
            Instrument::Kick => &self.kick,
            Instrument::OpenHihat => &self.open_hi_hat,
            Instrument::Ride => &self.ride,
            Instrument::Crash => &self.crash,
            Instrument::Tom1 => &self.tom_1,
            Instrument::Tom2 => &self.tom_2,
            Instrument::Tom3 => &self.tom_3,
            Instrument::PedalHiHat => &self.pedal_hihat,
        }
    }
}

fn get_midi_as_user_hits(midi_input: &MidiInput) -> Vec<UserHit> {
    let mut out: Vec<UserHit> = vec![];

    // midi device: "MPK Mini Mk II"
    let mpk_mini_mk_ii = InputConfigMidi {
        closed_hi_hat: HashSet::from_iter(vec![44, 48]),
        snare: HashSet::from_iter(vec![45, 49]),
        kick: HashSet::from_iter(vec![46, 50]),
        open_hi_hat: HashSet::from_iter(vec![47, 51]),
        ride: HashSet::from_iter(vec![]),
        crash: HashSet::from_iter(vec![]),
        tom_1: HashSet::from_iter(vec![]),
        tom_2: HashSet::from_iter(vec![]),
        tom_3: HashSet::from_iter(vec![]),
        pedal_hihat: HashSet::from_iter(vec![]),
    };

    // https://support.roland.com/hc/en-us/articles/360005173411-TD-17-Default-Factory-MIDI-Note-Map
    let td17 = InputConfigMidi {
        closed_hi_hat: HashSet::from_iter(vec![42, 22]),
        snare: HashSet::from_iter(vec![38, 40, 37]),
        kick: HashSet::from_iter(vec![36]),
        open_hi_hat: HashSet::from_iter(vec![46, 26]),
        ride: HashSet::from_iter(vec![51, 53, 59]),
        crash: HashSet::from_iter(vec![49, 55, 57, 52]),
        tom_1: HashSet::from_iter(vec![50, 48]),
        tom_2: HashSet::from_iter(vec![47, 45]),
        tom_3: HashSet::from_iter(vec![58, 43]),
        pedal_hihat: HashSet::from_iter(vec![44]),
    };

    // https://support.roland.com/hc/en-us/articles/4407474950811-TD-27-Default-MIDI-Note-Map
    let td27 = InputConfigMidi {
        closed_hi_hat: HashSet::from_iter(vec![42, 22]),
        snare: HashSet::from_iter(vec![38, 40, 37]),
        kick: HashSet::from_iter(vec![36]),
        open_hi_hat: HashSet::from_iter(vec![46, 26]),
        ride: HashSet::from_iter(vec![51, 53, 59]),
        crash: HashSet::from_iter(vec![49, 55, 57, 52]),
        tom_1: HashSet::from_iter(vec![50, 48]),
        tom_2: HashSet::from_iter(vec![47, 45]),
        tom_3: HashSet::from_iter(vec![58, 43]),
        pedal_hihat: HashSet::from_iter(vec![44]),
    };

    let alesis_nitro = InputConfigMidi {
        closed_hi_hat: HashSet::from_iter(vec![42]),
        snare: HashSet::from_iter(vec![38]),
        kick: HashSet::from_iter(vec![36]),
        open_hi_hat: HashSet::from_iter(vec![46, 23]),
        ride: HashSet::from_iter(vec![]),
        crash: HashSet::from_iter(vec![]),
        tom_1: HashSet::from_iter(vec![]),
        tom_2: HashSet::from_iter(vec![]),
        tom_3: HashSet::from_iter(vec![]),
        pedal_hihat: HashSet::from_iter(vec![]),
    };

    let ic_midi = match midi_input.get_device_name() {
        "MPK Mini Mk II" => mpk_mini_mk_ii,
        s if s.contains("TD-17") => td17,
        s if s.contains("TD-27") => td27,
        s if s.contains("Nitro") => alesis_nitro,
        _ => {
            log::warn!("warning: unknown midi device, using default of 'td27'");
            td27
        }
    };

    let pressed_midi = midi_input.get_pressed_buttons();

    // for each pressed_midi, check if it's in the ic_midi and then add to out as a proper UserHit if so
    for midi in pressed_midi {
        log::debug!("midi: {:?}", midi); // TODO: compare timestamps
        let timestamp = midi.timestamp as f64;
        for ins in ALL_INSTRUMENTS.iter() {
            if ic_midi.get_note_numbers(ins).contains(&midi.note_number) {
                out.push(UserHit::new(*ins, timestamp));
            }
        }
    }

    out
}
