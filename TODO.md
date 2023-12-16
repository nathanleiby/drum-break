# TODO

- [x] show looping bar for grid position
- [..] Play sounds based on grid data

- [..] ensure sound start aligns with moving over grid icon
  - [..] get it to play those sounds aligned correctly
    - [x] try kira 's clock to solve this https://github.com/tesselode/kira
    - [x] adjust BPM while playing
    - [x] play two sounds aligned
    - [ ] explore improving Kira's interface around the clock and looping to support my sequencer like use-case
      - [x] idea: looping behavior.. https://github.com/nathanleiby/kira/blob/main/crates/kira/src/clock.rs#L190-L197)
      - [x] check existing behavior: scheduling on 0 works? -> yes
      - [ ] idea: editing clock while it's playing
      - [x] idea: schedule just in time
- [ ] read loop data from file (e.g. TOML)
- [ ] loop editing / saving via UI
- [ ] loop editing while it's playing
- [ ] use a midi library for input
- [ ] akai mini working as input
