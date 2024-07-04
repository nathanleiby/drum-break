# TODO

## working on

- [ ] support more drum types (not just kick, snare, open/closed hat)
  - ride, pedal HH, crash, 3 toms.. or arbitrary mappings
  - hacking in RIDE, by following the compiler. Almost works except JSON is strict and missing field borks it. Can I avoid?

## asap

_what's must-have to make it useful to me?_

- make gold reachable
  - tweak strictness .. just a lil more generous on timing?
  - fix poor signaling of closed HH -- often triggers as MISSED (didn't hit? was Open HH due to midi)

_what's very important to make it engaging to me?_

- quick start + gets you into flow
  - idea: saves whatever loop, BPM you were doing last time -- recovers on next start
- capture progress over time (graph it, etc)

- [..] add better debugging for midi signals, so I can filter to important ones (e.g. can ignore polyphonic aftertouch 167 on changing HH pedal in terms of hitting notes on the beat)
  - can translate to names from here https://midi.org/expanded-midi-1-0-messages-list, then log better
  - proximate reason.. to figure out problem with closed HH not triggering

## soon

- (bug) explore triggering
  - [ ] double triggering of some TD17 notes (e.g. 2x hihat hits or 2x open hihat hits, esp on hard hits?)
  - [ ] non triggering (hit too soft? event getting dropped?)
- tracking loop accuracy: "perfect" vs "great" vs etc
  - give partial credit in "% acc" summary for close hits (e.g. 75% of the note)
  - add simple adjustment for tolerance (i.e. '% of beat' offset allowed for perfect vs great vs miss)
    - could be a slider. could be config file adjustment
    - see `CORRECT_MARGIN` in `score.rs`
- In "golden" practice mode.. you can tweak knobs for shiny-ness of gold (N and X) -- could be consts at start
  - idea: try https://docs.rs/cvars/latest/cvars/ to allow changing these in the UI during development

## future

- [..] Capture EXACT timing of the midi note for use in timing.
  - [..] UserHit model should include real ClockTime and (computed from that) corresponding beat.. this way we can determine "age" of a beat and expire it if needed (from looping perspective). Currently, UserHit is just re-using `Voices` as its data model
  - high precision input https://github.com/not-fl3/macroquad/issues/1 vs per frame
    - maybe could PR this? https://github.com/not-fl3/miniquad/issues/117
    - maybe separate thread for midi is enough, if i capture timing .. I have `raw_input.timestamp` in `midi.rs` .. could compare that vs frame start time
- [ ] refactor so i don't need explicit branches for each of 4 instruments everywhere..
      e.g. in `voices.rs`, moving from `Voices` to `Voice`
      e.g. for `config.rs`:
  ```
  // TODO: Use a hashmap of {instrument : HashSet } instead of hard-coded list of instruments
  // type GeneralizedInputConfigMidi = HashMap<Instrument, HashSet<u8>>;
  ```
- [ ] unit tests
  - [ ] consider + document which pieces can be unit tested (and iterated on more effectively than manual testing)
    - [ ] ex. write unit tests re: the accuracy summary metric
- save all input data
  - when?
    - on exit (click "x")
    - on "save" (press "s" explicitly)
  - save user timing data to a file (e.g. start with flatfile, someday a DB)
    - e.g. dump to a JSON
      1. the loop voices itself
      2. the users's input data
      3. worry about visualizing and cleaning later.. this is first pass on session over session data
- [ ] easily import midi
  - [ ] e.g. from Groove Scribe
- [..] explore Rust GUI options
  - [..] convert input to Event-based model .. better for new UI layer migration
  - [ ] egui https://www.egui.rs/ .. https://github.com/optozorax/egui-macroquad
    - had trouble getting egui-macroquad to build due to audio lib issues. version outdated? tried to pull in file and build locally, but had trouble with that too b/c of macroquad/miniquad version mismatch
    - [..] `iced` https://lib.rs/crates/iced (.. with `coffee` game engine too? https://github.com/hecrj/coffee .. or not that part, it's 4y old)
      - custom widget for the sequencer
        - https://github.com/iced-rs/iced/blob/master/examples/custom_widget/src/main.rs`
        - https://discourse.iced.rs/t/custom-widget-for-chess-board/325
      - input subscription https://www.reddit.com/r/rust/comments/wtzkx6/need_help_iced_subscriptions/ .. rdev has some MacOS permissions [caveats](https://crates.io/crates/rdev)
      - minimal audio focused app https://github.com/AWBroch/metronome/blob/main/src/main.rs .. could use kira for clock instead of iced's `time::every` which supports this metronome
        - static audio data to include it binary seems handy
    - `slint`: https://github.com/slint-ui/slint
    - [..] try using Tauri and build a web UI
      - can we have a Rust "engine" (process keyboard/midi events, play sound, etc) with the FE (draw UI, etc)
      - [..] Explore porting the "core" audio to Rust and UI in TS (https://tauri.app/)
- cleanup input UI, which quickly gets noisy
  - [x] e.g. hacky is a button to reset -> press "r"
  - another idea is "fade out" by age (e.g. just keep last K loops, or actually fade over time until gone by Kth loop)
- [x] log levels that allow easy filtering
- input improvements
  - [x] support >1 midi value per voice
  - [ ] allow easy rebinding within the app
- (bug) on changing loop, the voices aren't scheduled immediately. this means first few notes don't make sounds because of schedule ahead logic
  - this means even on first run.. when you choose an initial track and press play.. its sounds aren't scheduled yet.
- [ ] support a flexible length loop
  - longer is needed. ideally you could have a "song" and loop any segment of it for practice
- [ ] save calibrated offset (latency) config per connected midi device / system (TD17 = -0.01) .. i have multiple for testing
- design v2 UX for the app (prototype in Figma): core interactions, colors, layout, etc
- get better at using rust (+VSCode), e.g. debugger, cargo fix, etc https://code.visualstudio.com/docs/languages/rust
- allow printing version. use include str / include bytes from VERSION file
- press ? to show help (e.g. see all key bindings)
- accuracy
  - [ ] figure out how to allow first beat to get measured correct. since space starts clock right away... need a click in or empty space before the notes
  - [ ] Allow tuning margin for correctness in FE, until it feels dialied in. (see `score.rs`)
  - [ ] visualize correctness across multiple attempts of the loop
    - [ ] idea: box and whisker for each note
    - [ ] idea: color for each note (e.g. red for bad, green for good .. could also have a color to indicate early/late/miss trends)
  - [ ] since you started (press a button to reset)
  - [ ] all time
- midi - how does it work?
  - [ ] https://computermusicresource.com/MIDI.Commands.html
- [ ] Explore macroquad featureset, including [experimental](https://docs.rs/macroquad/latest/macroquad/experimental/index.html) like state machine and scenes
  - [ ] Also explore community extension https://github.com/ozkriff/awesome-quads
  - [ ] tune config w cvars approach? https://github.com/martin-t/cvars
- [ ] Explore similar existing offerings
  - [ ] Drum specific..
    - [ ] https://www.mutedrums.com/ / https://www.playdrumsonline.com/ (https://www.playdrumsonline.com/songs/create)
    - [ ] Melodics
    - [ ] Clone Hero
  - [ ] Rhythm games
- [ ] Explore deployment options
  - [..] deploy to web / WASM (possible? latency??)
    - [ ] KIRA example https://github.com/Moxinilian/kira-web-demo/tree/main
    - [ ] `confy` for config may not work out of the box https://github.com/search?q=repo%3Adirs-dev%2Fdirs-rs+wasm&type=code -- can't save?
    - [ ] maybe some of these? https://github.com/ozkriff/awesome-quads?tab=readme-ov-file#libraries-plugins
  - [ ] build with Tauri https://tauri.app
- [ ] toggle metronome on/off
- [ ] volume control
  - [ ] global
  - [ ] per voice (inl metronome)
- [..] bundle so it can be shared
  - [ ] as DMG? via [Tauri distribution](https://tauri.app/v1/guides/distribution/publishing)?
  - [..] include loops JSON data, or fetch them remotely (e.g. from public GH link)
  - [..] include audio data so we can play sounds?
    - [ ] `include_bytes!` https://doc.rust-lang.org/std/macro.include_bytes.html
  - [ ] sign code for easier local running without security override on Mac
- [ ] quality
  - [..] add unit tests
  - [ ] run build + tests in Github CI
- [ ] shipping artifacts
  - [ ] on git tag, ship a release in Github CI
- [ ] Make "voices" data model more generic.
  - [ ] support different numbers of voices (not just 4, as today)
  - [ ] capture loop config like tempo, length, etc. (++ for tempo ASAP)
  ```
  name:
  bpm:
  beats_total:
  beats_per_measure: # optional, will draw lines if so
  voices: # TODO: instruments?
    [
        sound: required
        override_name: # optional:
        notes: [] # a series of numbers, 0 indexes, corresponding to the beats to play on.
    ]
  ```
- [ ] show music as sheet music notation (e.g. https://github.com/jaredforth/lilypond-rs or various others)
- [ ] explore improving Kira's interface around the clock and looping to support my sequencer like use-case
  - [ ] idea: editing clock while it's playing

## done

- [x] MVP: "golden" practice mode (play it perfectly N times and then speeds up by X bpm)
- [x] MVP UX: display stats for (last loop, last 5 loops, since you started session) - showing last 3 loops as of now
- [x] handle beat 0 edge case -- q: is this working already? -> seems like it
- [x] handle idea of "miss" due to not playing a desired note at all -- probably a change to in `score.rs`
- [x] change note color -- orange note is too similar to "early" orange color
- [..] make it shareable
  - [x] set keybindings (midi bindings) for each drum
  - [..] windows build? or bring a mac downstairs to drums
- [x] attach to my drumkit and test
- [x] (bug) hard crash if no midi device is attached
- [x] add UI to save / open a loop file from your machine
- [x] enforce MIN_BPM and MAX_BPM (ex: 40 - 240)
