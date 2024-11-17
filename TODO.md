# TODO

Working on:

- [x] Get logging working in browser
- getting midi input working in browser
  - verified it does wwork you just need to wait
- show attached midi device in UI
- allow refreshing attached midi device via button press
- variable number of beats per loop

---

_Moved todos to https://github.com/nathanleiby/drum-break/issues._

- Add zebra stripes to rows of table, for better legibility
- Metronome still causes screen to flash.
  - Can likely fix same way.. "load once at start, then clone"
- improve usability of updating Latency offset.. clicking `]` over and over is slow.. and `shift + ]` jumps a bit too far per step (half it?).
- re-think audio scheduling. pretty sure i have a task for this but just a reminder ("just in time" is imperfect)
  - e.g. can stop sounds if they're removed, by calling handle.stop() ([docs](https://tesselode.github.io/kira/playing-sounds.html))
- toolchain
  - sort out vendoring of EGUI. hopefully don't need it copied in my repo
  - understanding WASM... could I get Macroquad to build in WASM the "normal" way, so it plays nice with the ecosystem
    - if solved, could try `trunk` with drumbreak for nicer local dev UX
