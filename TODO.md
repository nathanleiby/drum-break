# TODO

- [..] capture and display user timing input for beat -- see `input.rs` (will want a new data model for user activity, e.g. note/timing/velocity data)
  - [..] handle audio sync / delay
    - [..] hard-coded calibration (for me: speakers (-0.1), bluetooth headphones (-0.5))
    - [..] calibration step
      - [ ] inspiration: https://ddrkirbyisq.medium.com/rhythm-quest-devlog-10-latency-calibration-fb6f1a56395c
        https://rhythmquestgame.com/devlog/devlog.html
      - [ ] https://exceed7.com/native-audio/rhythm-game-crash-course/index.html
      - [ ] watch: https://www.youtube.com/watch?v=JTuZvRF-OgE&t=41s
  - [ ] save user timing data to a file (e.g. start with flatfile, someday a DB)
  - [x] consider audio latency when displaying user input
  - [x] save calibration data to disk, so it exists on next run -> saves to `$HOME/Library/Application\ Support/rs.macroix/AppConfig.toml` on mac .. see: https://github.com/rust-cli/confy/issues/89 .. https://github.com/dirs-dev/directories-rs#basedirs
- [ ] use a midi library for input -- see `input.rs`
- [ ] akai mini working as input
- [ ] make it shareable
  - [ ] decouple loops data (look in local directory, or fetch them remotely.. e.g. from public GH link)
- [ ] add UI to save / open a loop file from your machine
- [ ] Explore similar existin offerings
  - [ ] Drum specific..
    - [ ] https://www.mutedrums.com/ / https://www.playdrumsonline.com/ (https://www.playdrumsonline.com/songs/create)
    - [ ] Melodics
    - [ ] Clone Hero
  - [ ] Rhythm games

# TODO (future)

- [ ] deploy to web (possible? latency??)
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
