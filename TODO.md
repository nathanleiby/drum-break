# TODO

- [..] capture and display user timing input for beat -- see `input.rs` (will want a new data model for user activity, e.g. note/timing/velocity data)
  - [..] handle audio sync / delay
    - [..] hard-coded calibration (for me: speakers (0.02), bluetooth headphones (0.180))
    - [..] calibration step
      - [ ] inspiration: https://ddrkirbyisq.medium.com/rhythm-quest-devlog-10-latency-calibration-fb6f1a56395c
        https://rhythmquestgame.com/devlog/devlog.html
      - [ ] https://exceed7.com/native-audio/rhythm-game-crash-course/index.html
      - [ ] watch: https://www.youtube.com/watch?v=JTuZvRF-OgE&t=41s
  - [ ] save user timing data to a file (e.g. start with flatfile, someday a DB)
- [ ] use a midi library for input -- see `input.rs`
- [ ] akai mini working as input

# TODO (future)
- [ ] Make "voices" data model more generic.
  - [ ] support more drum sounds (not just kick, snare, hat, clap)
  - [ ] support different numbers of voices (not just 4, as today)
  - [ ] capture loop config like tempo, length, etc.
- [ ] show music as sheet music notation (e.g. https://github.com/jaredforth/lilypond-rs or various others)
- [ ] explore improving Kira's interface around the clock and looping to support my sequencer like use-case
  - [ ] idea: editing clock while it's playing
- [ ] Explore porting the "core" audio to Rust and UI in TS (https://tauri.app/)
