# TODO

- [ ] capture and display user timing input for beat -- see `input.rs` (will want a new data model for user activity, e.g. note/timing/velocity data)
  - [ ] save user timing data to a file (e.g. start with flatfile, someday a DB)
- [ ] use a midi library for input -- see `input.rs`
- [ ] akai mini working as input

# TODO (future)
- [ ] show music as sheet music notation (e.g. https://github.com/jaredforth/lilypond-rs or various others)
- [ ] explore improving Kira's interface around the clock and looping to support my sequencer like use-case
  - [ ] idea: editing clock while it's playing
- [ ] Explore porting the "core" audio to Rust and UI in TS (https://tauri.app/)
