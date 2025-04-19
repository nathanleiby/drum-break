# TODO

(_Moved todos to https://github.com/nathanleiby/drum-break/issues._)

---

Working on:

---

- Fix midi mapping for TD27. Closed HiHat hit is triggering "open hihat" note in app.
- Nicer icon and helper text in top right (see [rerun code](https://github.com/rerun-io/rerun/blob/1ad3042a85622804a6923a5d0c65f82ba1e601d3/crates/viewer/re_viewer/src/ui/top_panel.rs#L267-L281))
- improve usability of updating Latency offset.. clicking `]` over and over is slow.. and `shift + ]` jumps a bit too far per step (half it?).
- re-think audio scheduling. pretty sure i have a task for this but just a reminder ("just in time" is imperfect)
  - e.g. can stop sounds if they're removed, by calling handle.stop() ([docs](https://tesselode.github.io/kira/playing-sounds.html))
- toolchain
  - sort out vendoring of EGUI. hopefully don't need it copied in my repo
  - understanding WASM... could I get Macroquad to build in WASM the "normal" way, so it plays nice with the ecosystem
    - if solved, could try `trunk` with drumbreak for nicer local dev UX
