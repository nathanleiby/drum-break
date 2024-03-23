# TODO

## asap

_what's must-have to make it useful to me?_

- tracking loop accuracy: "perfect" vs "great" vs etc
  - handle beat 0 edge case
- handle idea of "miss" due to not playing a desired note at all
- cleanup input UI, which quickly gets noisy
  -- [x] e.g. hacky is a button to reset
  -- another idea is "fade out" by age (e.g. just keep last K loops, or actually fade over time until gone by Kth loop)

_what's very important to make it engaging to me?_

- quick start + gets you into flow
- "golden" practice mode (play it perfectly N times and then speeds up by X bpm)
  - you can tweak knobs for shiny-ness of gold (N and X)
- capture progress over time (graph it, etc)

## soon

- [ ] support a flexible length loop
  - longer is needed. ideally you could have a "song" and loop any segment of it for practice
- [ ] save calibrated offset (latency) config per connected midi device / system (TD17 = -0.01) .. i have multiple for testing
- [ ] UserHit model should include real ClockTime and (computed from that) corresponding beat.. this way we can determine "age" of a beat and expire it if needed (from looping perspective). Currently, UserHit is just re-using `Voices` as its data model
- (bug) on changing loop, the voices aren't scheduled immediately. this means first few notes don't make sounds because of schedule ahead logic
  - this means even on first run.. when you choose an initial track and press play.. its sounds aren't scheduled yet.
- save all input data
  - when?
    - on exit (click "x")
    - on "save" (press "s" explicitly)
  - save user timing data to a file (e.g. start with flatfile, someday a DB)
    - e.g. dump to a JSON
      1. the loop voices itself
      2. the users's input data
      3. worry about visualizing and cleaning later.. this is first pass on session over session data
- (bug) explore triggering
  - [ ] double triggering of some TD17 notes (e.g. 2x hihat hits or 2x open hihat hits, esp on hard hits?)
  - [ ] non triggering (hit too soft? event getting dropped?)
- input improvements
  - [x] support >1 midi value per voice
  - [ ] allow easy rebinding within the app

## future

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
- high precision input https://github.com/not-fl3/macroquad/issues/1 vs per frame
  - maybe could PR this? https://github.com/not-fl3/miniquad/issues/117
  - maybe separate thread for midi is enough, if i capture timing .. I have `raw_input.timestamp` in `midi.rs` .. could compare that vs frame start time
- midi - how does it work?
  - [ ] https://computermusicresource.com/MIDI.Commands.html
- [ ] explore Rust GUI options
  - [ ] egui https://www.egui.rs/ .. https://github.com/optozorax/egui-macroquad
    - had trouble getting egui-macroquad to build due to audio lib issues. version outdated? tried to pull in file and build locally, but had trouble with that too b/c of macroquad/miniquad version mismatch
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
  - [ ] as DMG?
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
  - [ ] support more drum types (not just kick, snare, open/closed hat)
    - ride, pedal HH, crash, 3 toms.. or arbitrary mappings
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
- [ ] Explore porting the "core" audio to Rust and UI in TS (https://tauri.app/)

## done

- [x] change note color -- orange note is too similar to "early" orange color
- [..] make it shareable
  - [x] set keybindings (midi bindings) for each drum
  - [..] windows build? or bring a mac downstairs to drums
- [x] attach to my drumkit and test
- [x] (bug) hard crash if no midi device is attached
- [x] add UI to save / open a loop file from your machine
- [x] enforce MIN_BPM and MAX_BPM (ex: 40 - 240)
