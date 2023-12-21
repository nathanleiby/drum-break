# TODO

## soon

- [..] capture and display user timing input for beat -- see `input.rs` (will want a new data model for user activity, e.g. note/timing/velocity data)
  - [ ] save user timing data to a file (e.g. start with flatfile, someday a DB)
- measure your accuracy

  - [x] show correctness (in the current loop)
  - [ ] visualize correctness across multiple attempts of the loop
  - [ ] idea: box and whisker for each note
  - [ ] idea: color for each note (e.g. red for bad, green for good .. could also have a color to indicate early/late/miss trends)
  - [ ] since you started (press a button to reset)
  - [ ] all time

- [ ] make it shareable
  - [ ] decouple loops data (look in local directory, or fetch them remotely.. e.g. from public GH link)
- [ ] set keybindings (midi bindings) for each drum
- [ ]
- [ ] add UI to save / open a loop file from your machine

## future

- accuracy
  - [ ] figure out how to allow first beat to get measured correct. since space starts clock right away... need a click in or empty space before the notes
  - [ ] Allow tuning margin for correctness in FE, until it feels dialied in. (see `score.rs`)
- midi - how does it work?
  - [ ] https://computermusicresource.com/MIDI.Commands.html
- [ ] explore Rust GUI options
  - [ ] egui https://www.egui.rs/ .. https://github.com/optozorax/egui-macroquad
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
- [ ] bundle so it can be shared
  - [ ] as DMG?
  - [ ] include loops JSON data, or fetch them remotely (e.g. from public GH link)
  - [ ] sign code for easier local running without security override on Mac
- [ ] quality
  - [ ] add unit tests
  - [ ] run build + tests in Github CI
- [ ] shipping artifacts
  - [ ] on git tag, ship a release in Github CI
- [ ] Make "voices" data model more generic.
  - [ ] support more drum sounds (not just kick, snare, hat, clap)
  - [ ] support different numbers of voices (not just 4, as today)
  - [ ] capture loop config like tempo, length, etc.
- [ ] show music as sheet music notation (e.g. https://github.com/jaredforth/lilypond-rs or various others)
- [ ] explore improving Kira's interface around the clock and looping to support my sequencer like use-case
  - [ ] idea: editing clock while it's playing
- [ ] Explore porting the "core" audio to Rust and UI in TS (https://tauri.app/)
