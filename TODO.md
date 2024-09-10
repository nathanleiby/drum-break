# TODO

## working on

- [ ] allow tweaking strictness within the UI
  - why:
    - make gold reachable .. thus make practicing via the app more fun/meaningful
    - still hard. not dialed in! be just a lil more generous on timing?
  - where: UI (egui: bottom panel, overlay), config file
    - idea: try https://docs.rs/cvars/latest/cvars/ to allow changing these in the UI during development (or EGUI [overlay](https://rodneylab.com/macroquad-egui-devtools/))
  - how: press backtick button to toggle "dev view" ^
  - what:
    - (1) num correct (2) Bpm step (3) Correctness sensitivity
    - tolerance (i.e. '% of beat' offset allowed for perfect vs great vs miss)
      - see `CORRECT_MARGIN` in `score.rs`

## soon

- [ ] quick start + gets you into flow
  - idea: saves whatever loop, BPM you were doing last time -- recovers on next start
  - capture progress over time (graph it, etc)
- [..] add better debugging for midi signals, so I can filter to important ones (e.g. can ignore polyphonic aftertouch 167 on changing HH pedal in terms of hitting notes on the beat)
  - can translate to names from here https://midi.org/expanded-midi-1-0-messages-list, then log better
  - proximate reason.. to figure out problem with closed HH not triggering
- (bug) explore triggering
  - [ ] fix poor signaling of closed HH -- often triggers as MISSED (didn't hit? was Open HH due to midi)- In "golden" practice mode.. you can tweak knobs for shiny-ness of gold (N and X) -- could be consts at start
  - [ ] double triggering of some TD17 notes (e.g. 2x hihat hits or 2x open hihat hits, esp on hard hits?)
  - [ ] non triggering (hit too soft? event getting dropped?)
- tracking loop accuracy: "perfect" vs "great" vs etc
  - give partial credit in "% acc" summary for close hits (e.g. 75% of the note)

## future

- try colored emojis via https://crates.io/crates/egui-twemoji for gold mode status
- Onboarding / Ease of Use
  - [ ] redo the "calibrate latency offset" UX. Look at other models like Rhythm Doctor
  - [ ] press ? to show help (e.g. see all key bindings)
- Web Build (WASM)
  - Why? Way easier to share and sbhip iterative improvements.
  - How? Get any working web build, even if degraded UX (latency? midi?)
    - Dig into specific lib issues
      - midir - https://github.com/Boddlnagg/midir/blob/master/examples/browser/src/lib.rs
      - `confy` for config may not work out of the box https://github.com/search?q=repo%3Adirs-dev%2Fdirs-rs+wasm&type=code -- can't save?
      - kira audio - ...
        - KIRA example https://github.com/Moxinilian/kira-web-demo/tree/main
    - maybe some of these? https://github.com/ozkriff/awesome-quads?tab=readme-ov-file#libraries-plugins
    - maybe this: https://github.com/not-fl3/macroquad/issues/212#issuecomment-1944760503
  - [ ] Publish to GH pages
  - [ ] publish to Itch.io
    - [ ] WASM - https://github.com/brettchalupa/sokoworld/blob/09ce68c690cbae0db242ab1b403c309f8b8482d2/release_wasm.sh
    - [ ] https://mq.agical.se/release-web.html
- Native App - improve "first run" UX
  - [ ] sign code for easier local running without security override on Mac
  - [ ] Make it a DMG?
  - [ ] Mac Store
  - [ ] Auto updater
  - [ ] `include_bytes!` https://doc.rust-lang.org/std/macro.include_bytes.html
    - [..] include loops JSON data, or fetch them remotely (e.g. from public GH link)
    - [..] include audio data so we can play sounds?
      - ... would this also work for web, just makes a fat binary/wasm file?
- Loop editing
  - [ ] Allow easily re-assigning an instrument within a row (e.g. swap hihat to ride)
  - [ ] Allow click-and-drag UX to add/remove beats
  - [ ] File open UX -- open a loop from a file
  - [ ] easily import midi
    - e.g. from Groove Scribe
- Usability
  - make extra BeatGrid rows less distracting -- allow show/hide in UI for unused rows
- UX v2: Design
  - commit UI prototypes (tauri, iced) to Github (optionally, migrate into macroix repo if reasonable to centralize)
  - [ ] Make UI look nice-ish
    - [ ] do a pass in Figma
    - [ ] better font
    - [ ] funklet inspired colors and look
    - [ ] show/hide noisy stuff
  - prototype in Figma: core interactions, colors, layout, etc
    - might we show music as sheet music notation (e.g. https://github.com/jaredforth/lilypond-rs or various others)
    - https://github.com/staff-rs/staff/
  - Explore similar existing offerings
    - Drum specific..
      - https://www.mutedrums.com/ / https://www.playdrumsonline.com/ (https://www.playdrumsonline.com/songs/create)
      - Melodics
      - Clone Hero
    - Rhythm games
      - Rhythm Doctor
      - Stepmania
      - Osu
- Internals
  - [ ] fix BPM
    - assuming each beat in grid is a 16th note, it should be BPM \* 2 (so 120 = 60)
    - I think ideally the data model for user_hits and desired_hits aligns nicely, i.e. 1.0 is beat 1, 2.0 is beat 2. So e.g. 16th notes are 0.25 in length
  - [ ] unit tests
    - consider + document which pieces can be unit tested (and iterated on more effectively than manual testing)
      - ex. write unit tests re: the accuracy summary metric
  - [ ] support a flexible length loop
    - longer is needed. ideally you could have a "song" and loop any segment of it for practice
  - simplify usage of f32 or f64
  - replace some f64 with decimal(?) for better equality support, hashing, etc
- Feature: "Swing"
  - add a "swing" meter like in Funklet https://machine.funklet.com/funklet.html
  - in Funklet, this is a setting from 0 to 12 that pushes beats 2 and 4 slightly later (from 0% to 95% or something). I suspect this is how "% swing" works in other apps, too.
  - there is a data encoding for their beats.. maybe I can reverse engineer to port over the samples easily?
    - https://goodhertz.com/funklet/machine?vals=3232323232323220323232323232323232323232323232203232323232323232;0000400201004000000040020200400200004002010040000000400201000002;3404000000040030340400000004040034040000000400403404000000403400&mods=..............1................................1&b=91&s=1&jd=0,0,0&r=1,1,1&a=000#
- Bugs
  - [ ] (bug) ScoreTracker behaves strangely when you have >1 Correct user hit for a single desired note (e.g. 2/2 or 3/3 could refer to 2 desired notes, just in the latter case we have 3 correct notes total bc two hits were within the Correct margin)
  - [ ] (bug) on changing loop, the voices aren't scheduled immediately. this means first few notes don't make sounds because of schedule ahead logic
  - this means even on first run.. when you choose an initial track and press play.. its sounds aren't scheduled yet.
- Input Precision
  - [..] Capture EXACT timing of the midi note for use in timing.
    - [..] UserHit model should include real ClockTime and (computed from that) corresponding beat.. this way we can determine "age" of a beat and expire it if needed (from looping perspective). Currently, UserHit is just re-using `Voices` as its data model
    - high precision input https://github.com/not-fl3/macroquad/issues/1 vs per frame
      - maybe could PR this? https://github.com/not-fl3/miniquad/issues/117
      - maybe separate thread for midi is enough, if i capture timing .. I have `raw_input.timestamp` in `midi.rs` .. could compare that vs frame start time
- Feature: Long-terms stats
  - save all input data
    - when?
      - on exit (click "x")
      - on "save" (press "s" explicitly)
    - save user timing data to a file (e.g. start with flatfile, someday a DB)
      - e.g. dump to a JSON
        1. the loop voices itself
        2. the users's input data
        3. worry about visualizing and cleaning later.. this is first pass on session over session data
- Input improvements
  - [x] support >1 midi value per voice
  - [ ] allow easy rebinding within the app
  - [ ] save calibrated offset (latency) config per connected midi device / system (TD17 = -0.01) .. i have multiple for testing
- Feature: Volume control
  - [ ] global
  - [ ] per voice (inl metronome)
- [ ] quality
  - [ ] run build + tests in Github CI
- [ ] shipping artifacts
  - [ ] on git tag, ship a release in Github CI

### Research / Learn

- building for android or ios
  - https://macroquad.rs/articles/android/
  - https://macroquad.rs/articles/ios/
- midi - how does it work?
  - [ ] https://computermusicresource.com/MIDI.Commands.html
- get better at using rust (+VSCode), e.g. debugger, cargo fix, etc https://code.visualstudio.com/docs/languages/rust
- [ ] Explore macroquad featureset, including [experimental](https://docs.rs/macroquad/latest/macroquad/experimental/index.html) like state machine and scenes
  - [ ] Also explore community extension https://github.com/ozkriff/awesome-quads
  - [ ] tune config dynamically w cvars approach? https://github.com/martin-t/cvars or egui debug overlay
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
- [ ] explore improving Kira's interface around the clock and looping to support my sequencer like use-case
  - [ ] idea: editing clock while it's playing
- should use kira's built-in Metronome concept? https://github.com/tesselode/kira/blob/main/changelog.md#multiple-metronomes

### NEEDS TRIAGE / CLARITY

- allow printing version. use include str / include bytes from VERSION file
- accuracy
  - [ ] figure out how to allow first beat to get measured correct. since space starts clock right away... need a click in or empty space before the notes
  - [ ] Allow tuning margin for correctness in FE, until it feels dialied in. (see `score.rs`)
  - [ ] visualize correctness across multiple attempts of the loop
    - [ ] idea: box and whisker for each note
    - [ ] idea: color for each note (e.g. red for bad, green for good .. could also have a color to indicate early/late/miss trends)
  - [ ] since you started (press a button to reset)
  - [ ] all time
- visualize beat as sheet music
  - stretch: highlight the active note
  - stretch: moving bar
- support other meters
  - Implied 4/4 now.. What about 6/8?
  - Also, could relate to ability to fine tune note positions
- improve "save loop" UX
  - Make sure it's clear when you save! I hit "s" a few times thinking I saved things, but I didn't. Maybe add a save button to UI?
  - Alow editing name of loop. I often want to keep track of a real beat.
  - Allow grouping/tagging
    - folder system, tags, etc...
    - ex: all beats related to a single song.
    - ex: all beats building up to a more complex beat .. a practice sequence
    - ex: by genre, artist, etc
- sync saved loops across devices - I'm using 2 devices for dev + practice flow today.
  - How?
    - read from remote DB? Firebase, Dexie, etc
    - Simplest CRUD possible
    - consider abstract DB backend for opensource flexibility
- Extract drums from a song (ML), then translate to a loop to practice
- Handle wrong/extra hits (ignored right now)
- layer on complexity
  - for example: https://photos.app.goo.gl/ALXpmdq2ztNAwWY6A
  - Could generate increasingly complex exercises, potentially incorporating skills you need.
    New drums to hit
    New displacement
    Flams
    Whatever

## done

- [x] Refactor message passing .. should be typed (see `main.rs` in `rx.try_recv`)
- [x] UX v2: Determine tooling -> egui
  - try themes .. https://github.com/catppuccin/egui?tab=readme-ov-file .. looks good, but not a big delta so maybe later
- Feature: Metronome
  - [x] toggle metronome on/off
- [x] log levels that allow easy filtering
- [x] (bug) GOLDEN MODE logic is broken. Denominator seems to be correct user_hits instead of desired notes
- [x] refactor so i don't need explicit branches for each of 4 instruments everywhere..
      e.g. in `voices.rs`, moving from `Voices` to `Voice`
      e.g. for `config.rs`.
      note.. I didn't fix this last suggestion re: Midi config but shrug..
  ```
  // TODO: Use a hashmap of {instrument : HashSet } instead of hard-coded list of instruments
  // type GeneralizedInputConfigMidi = HashMap<Instrument, HashSet<u8>>;
  ```
- [x] support more drum types (not just kick, snare, open/closed hat)
  - [x] ride, pedal HH, [x] crash, 3 toms.. or arbitrary mappings
  - hacking in RIDE, by following the compiler. Almost works except JSON is strict and missing field borks it. Can I avoid?
  - [x] Add sounds for more instruments (ride, bell, pedal HH, crash, etc)
  - samples MVP: https://www.reddit.com/r/edmproduction/comments/4ew9ut/free_sample_pack_of_my_acoustic_drum_kit_real/
    - https://www.dropbox.com/scl/fi/60funlj95o1i8hg/Real-Drums-Vol.-1.zip
- [x] fix accuracy bug .. see score.rs unit tests -- not fixed but band-aid
- [x] MVP: "golden" practice mode (play it perfectly N times and then speeds up by X bpm)
- [x] MVP UX: display stats for (last loop, last 5 loops, since you started session) - showing last 3 loops as of now
- [x] handle beat 0 edge case -- q: is this working already? -> seems like it
- [x] handle idea of "miss" due to not playing a desired note at all -- probably a change to in `score.rs`
- [x] change note color -- orange note is too similar to "early" orange color
- [..] make it shareable

  - [x] set keybindings (midi bindings) for each drum
  - [..] windows build? or bring a mac downstairs to drums
  - share it
    - w nick
    - macroquad showcase https://discord.com/channels/710177966440579103/868282388407517214
    - awesome quads apps https://github.com/ozkriff/awesome-quads?tab=readme-ov-file#apps-or-visualizations
    - r/edrums once it's polished enough to share?

- [x] attach to my drumkit and test
- [x] (bug) hard crash if no midi device is attached
- [x] add UI to save / open a loop file from your machine
- [x] enforce MIN_BPM and MAX_BPM (ex: 40 - 240)
