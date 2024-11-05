# TODO

## working on

TBD

## soon

- [ ] allow easily tweaking game config (or: why I miss dynamic type systems, sometimes)
  - blocker: need internet access to download CVARS and try that approach
  - why:
    - make gold reachable .. thus make practicing via the app more fun/meaningful
    - still hard. not dialed in! be just a lil more generous on timing?
  - where: UI (egui: bottom panel, overlay), config file
    - https://github.com/martin-t/cvars
    - idea: try https://docs.rs/cvars/latest/cvars/ to allow changing these in the UI during development (or EGUI [overlay](https://rodneylab.com/macroquad-egui-devtools/))
      - my handmade approach with egui overlay was more annoying to use b/c I had to wire it up to full event-driven UI. Let's see if cvars would make for easier mutation of global consts, from a "dev console" perspective rather a normal interactive UX
  - how: press backtick button to toggle "dev view" ^
  - what:
    - Correctness tolerance (i.e. '% of beat' offset allowed for correct vs early/late vs miss)
      - see `CORRECT_MARGIN` and `MISS_MARGIN` in `score.rs`
    - Gold Mode (1) num correct (2) Bpm step
      - see `GOLD_MODE_..` in `game.rs`
- "keep flow" MVP: On quit, save chosen loop and BPM you were doing. On start, recover that state.
  - BONUS: extending this 'save state' behavior might lead well to saving a 'UI Story' snapshot that you can resume later. keypress to save state / resume state at will would be dope for development.
- Update README with latest explanation and screenshots
  - manually...
  - idea: can an egui app screenshot itself?
  - idea: can I spawn an app and then automate a screenshot?

## future

- Quick wins
  - add Clippy setup locally, to enforce better rust practices
  - enable CI - https://github.com/nathanleiby/drum-break/commit/8bb53e467e6104d1a68414e85139ad722f7e60c6
    - tests
    - lint (clippy)
    - build
    - release
- Add "Stop" interaction (resets Current Loop = 0, Current Beat = 0)
- [..] add better debugging for midi signals, so I can filter to important ones (e.g. can ignore polyphonic aftertouch 167 on changing HH pedal in terms of hitting notes on the beat)
  - can translate to names from here https://midi.org/expanded-midi-1-0-messages-list, then log better
  - proximate reason.. to figure out problem with closed HH not triggering
  - TODO: I think remaining work here is to log the drum name itself. Cannot test without midi controller hooked up.
- (bug) explore triggering
  - [ ] fix poor signaling of closed HH -- often triggers as MISSED (didn't hit? was Open HH due to midi)- In "golden" practice mode.. you can tweak knobs for shiny-ness of gold (N and X) -- could be consts at start
  - [ ] double triggering of some TD17 notes (e.g. 2x hihat hits or 2x open hihat hits, esp on hard hits?)
  - [ ] non triggering (hit too soft? event getting dropped?)
- (bug) adjusting BPM changes gold mode % accuracy
  - this relates to calibration offset (and may temporarily NOT be a problem at the moment due to changes in gold mode calc)
- Feature: Volume control
  - [ ] global
  - [ ] per voice (inl metronome)
- Loop editing
  - [ ] Allow easily re-assigning an instrument within a row (e.g. swap hihat to ride)
  - [ ] Allow click-and-drag UX to add/remove beats
  - [ ] File open UX -- open a loop from a file
  - [ ] easily import midi
    - e.g. from Groove Scribe
- try colored emojis via https://crates.io/crates/egui-twemoji for gold mode status
- Onboarding / Ease of Use
  - [ ] redo the "calibrate latency offset" UX. Look at other models like Rhythm Doctor
  - [..] press ? to show help (e.g. see all key bindings)
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
  - [ ] Add a custom app icon
  - [ ] sign code for easier local running without security override on Mac
  - [ ] Make it a DMG?
  - [ ] Mac Store
  - [ ] Auto updater
  - [ ] `include_bytes!` https://doc.rust-lang.org/std/macro.include_bytes.html
    - [..] include loops JSON data, or fetch them remotely (e.g. from public GH link)
    - [..] include audio data so we can play sounds?
      - ... would this also work for web, just makes a fat binary/wasm file?
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
    - could we rethink scheduling? Right now it happens "just in time" as the clock progresses, but perhaps it should also depend on when notes are added/removed?
      - relatedly, we might want to track handles for all scheduled sounds so we can stop them before/during play if they're removed
      ```rs
      let handle = manager.play(sound.with_settings(sound_settings))?;
      // track the handles...
      // if need to stop...
      handle.stop()
      ```
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
- MIDI Input improvements
  - [ ] virtual midi controller .. Can I download one and attach it to the app, for testing?
    - [ ] Scope creep / side project.. Could I build a virtual midi controller?
      - One could even trigger it with a computer keyboard :P
      - Maybe it could be on ipad / iOS app too
  - [x] support >1 midi value per voice
  - [ ] allow easy rebinding within the app
  - [ ] save calibrated offset (latency) config per connected midi device / system (TD17 = -0.01) .. i have multiple for testing
- [ ] quality
  - [ ] run build + tests in Github CI
- [ ] shipping artifacts
  - [ ] on git tag, ship a release in Github CI
- App Versioning
  - Use version in Cargo file to manage Github release version (instead of separate `VERSION` file)
  - allow printing version. use include str / include bytes from VERSION file
- (bug) Causing full crash `Time shouldn't move backwards`. Potentially issue in vendored egui

```
thread 'main' panicked at /Users/nathanleiby/.cargo/registry/src/index.crates.io-6f17d22bba15001f/emath-0.28.1/src/history.rs:129:13:
Time shouldn't move backwards
stack backtrace:
   0: rust_begin_unwind
             at /rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library/std/src/panicking.rs:652:5
   1: core::panicking::panic_fmt
             at /rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library/core/src/panicking.rs:72:14
   2: emath::history::History<T>::add
             at /Users/nathanleiby/.cargo/registry/src/index.crates.io-6f17d22bba15001f/emath-0.28.1/src/history.rs:129:13
   3: egui::input_state::PointerState::begin_frame
             at /Users/nathanleiby/.cargo/registry/src/index.crates.io-6f17d22bba15001f/egui-0.28.1/src/input_state.rs:938:13
   4: egui::input_state::InputState::begin_frame
             at /Users/nathanleiby/.cargo/registry/src/index.crates.io-6f17d22bba15001f/egui-0.28.1/src/input_state.rs:221:23
   5: egui::context::ContextImpl::begin_frame_mut
             at /Users/nathanleiby/.cargo/registry/src/index.crates.io-6f17d22bba15001f/egui-0.28.1/src/context.rs:449:26
   6: egui::context::Context::begin_frame::{{closure}}
             at /Users/nathanleiby/.cargo/registry/src/index.crates.io-6f17d22bba15001f/egui-0.28.1/src/context.rs:777:26
   7: egui::context::Context::write
             at /Users/nathanleiby/.cargo/registry/src/index.crates.io-6f17d22bba15001f/egui-0.28.1/src/context.rs:723:9
   8: egui::context::Context::begin_frame
             at /Users/nathanleiby/.cargo/registry/src/index.crates.io-6f17d22bba15001f/egui-0.28.1/src/context.rs:777:9
   9: egui::context::Context::run
             at /Users/nathanleiby/.cargo/registry/src/index.crates.io-6f17d22bba15001f/egui-0.28.1/src/context.rs:752:9
  10: egui_miniquad::EguiMq::run
             at ./egui_miniquad/src/lib.rs:185:27
  11: egui_macroquad::Egui::ui
             at ./egui_macroquad/src/lib.rs:87:9
  12: egui_macroquad::ui
             at ./egui_macroquad/src/lib.rs:100:5
  13: macroix::ui::UI::render
             at ./src/ui.rs:23:9
  14: macroix::amain::{{closure}}
             at ./src/main.rs:118:9
  15: macroix::main::{{closure}}
             at ./src/main.rs:49:1
  16: macroquad::exec::resume
             at /Users/nathanleiby/.cargo/registry/src/index.crates.io-6f17d22bba15001f/macroquad-0.4.12/src/exec.rs:72:11
  17: <macroquad::Stage as miniquad::event::EventHandler>::draw::{{closure}}
             at /Users/nathanleiby/.cargo/registry/src/index.crates.io-6f17d22bba15001f/macroquad-0.4.12/src/lib.rs:721:24
  18: <macroquad::Stage as miniquad::event::EventHandler>::draw::maybe_unwind
             at /Users/nathanleiby/.cargo/registry/src/index.crates.io-6f17d22bba15001f/macroquad-0.4.12/src/lib.rs:712:21
  19: <macroquad::Stage as miniquad::event::EventHandler>::draw
             at /Users/nathanleiby/.cargo/registry/src/index.crates.io-6f17d22bba15001f/macroquad-0.4.12/src/lib.rs:717:26
  20: miniquad::native::macos::define_opengl_view_class::draw_rect
             at /Users/nathanleiby/.cargo/registry/src/index.crates.io-6f17d22bba15001f/miniquad-0.4.5/src/native/macos.rs:663:13
  21: <unknown>
  22: <unknown>
  23: <unknown>
  24: <unknown>
  25: <unknown>
  26: <unknown>
  27: <unknown>
  28: <unknown>
  29: <unknown>
  30: <unknown>
  31: <unknown>
  32: <unknown>
  33: <unknown>
  34: <unknown>
  35: <unknown>
  36: <(A,B,C,D) as objc::message::MessageArguments>::invoke
             at /Users/nathanleiby/.cargo/registry/src/index.crates.io-6f17d22bba15001f/objc-0.2.7/src/message/mod.rs:128:17
  37: objc::message::platform::send_unverified
             at /Users/nathanleiby/.cargo/registry/src/index.crates.io-6f17d22bba15001f/objc-0.2.7/src/message/apple/mod.rs:27:9
  38: objc::message::send_message
             at /Users/nathanleiby/.cargo/registry/src/index.crates.io-6f17d22bba15001f/objc-0.2.7/src/message/mod.rs:178:5
  39: miniquad::native::macos::run
             at /Users/nathanleiby/.cargo/registry/src/index.crates.io-6f17d22bba15001f/objc-0.2.7/src/macros.rs:142:15
  40: miniquad::start
             at /Users/nathanleiby/.cargo/registry/src/index.crates.io-6f17d22bba15001f/miniquad-0.4.5/src/lib.rs:384:9
  41: macroquad::Window::from_config
             at /Users/nathanleiby/.cargo/registry/src/index.crates.io-6f17d22bba15001f/macroquad-0.4.12/src/lib.rs:840:9
  42: macroix::main
             at ./src/main.rs:49:1
  43: core::ops::function::FnOnce::call_once
             at /rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

- [..] "UI Stories" (UI Test States)
  - like in Storybook, it would be nice to quickly pop into the app with a given data-state
    - a specific loop is loaded
    - some user hits exist with various accuracies
    - there's historical data so gold mode graph is populated
  - Currently, I need to manually play drum parts and verify behavior
  - As with Storybook, the goal is def to move lots of testing to units.
  - Scope creep..
    - Possible I could load individual components of my app, too?
    - :thinking: "EGUI Storybook" might be compelling to others using EGUI too and building something composed of various components.
    - It would be esp neat if I could spin up in a browser

### Research / Learn

- building for android or ios
  - https://macroquad.rs/articles/android/
  - https://macroquad.rs/articles/ios/
- midi - how does it work?
  - [ ] https://computermusicresource.com/MIDI.Commands.html
- get better at using rust (+VSCode), e.g. debugger, cargo fix, etc https://code.visualstudio.com/docs/languages/rust
- [ ] Explore macroquad featureset, including [experimental](https://docs.rs/macroquad/latest/macroquad/experimental/index.html) like state machine and scenes
  - [ ] Also explore community extension https://github.com/ozkriff/awesome-quads
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
- UI: try out theme colors from rerun https://github.com/rerun-io/rerun/blob/4636188996038f4be913f813fb263a3751c1d469/crates/re_ui/src/design_tokens.rs#L207
  - see other inspiration: https://github.com/emilk/egui/issues/996
- UI: consider dark-mode switcher widget https://tau.garden/egui-theme-switch/

### NEEDS TRIAGE / CLARITY

- NEED UX DESIGN WORK
  - Capture progress over time (graph it, etc)
    - The full details of the gold mode chart.
    - Personal bests.
  - Quick start + gets you into flow.
    - how to reduce barrier to entry?
      - resume previous state?
      - daily challenge?
      - suggested activities?
      - warm up drill? (with option to skip)
      - remind user of last session's work. progress and
- More nuanced Accuracy scoring
  - Possible approach: Use more Accuracy levels instead of just CORRECT , EARLY/LATE , MISSED. For example, in DDR a note can be "marvelous", "perfect", "great, "good", ... these are finer gradations of early/late
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

- [x] (usability) make extra BeatGrid rows less distracting -- allow show/hide in UI for unused rows
  - get number of rows right
  - allow toggle via button on RHS
  - make sure click interactions select right instrument row
- [x] Load into the app with some mock game state.
  - basic pass working. controlled by `MOCK_INITIAL_STATE` in `main.rs`
- [x] (bug) it's possible to click-and-drag on the gold mode chart
- [x] give partial credit in "% acc" summary for close hits (e.g. 75% of the note)
- [x] keep gold mode graph centered from 0 to 100
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
