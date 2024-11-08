# TODO

_Moved todos to https://github.com/nathanleiby/drum-break/issues._

Screen is blinking is WASM build.
Maybe I'm loading a resource during the loop in a way that's expensive (e.g. audio samples?)
https://github.com/not-fl3/macroquad/issues/471#issuecomment-1346563191

confirmed it seems to flash right when audio is being scheduled.
if two beats in close succession maybe the same is in ram?
revisit idea of loading once and then cloning, for performance...

This is also happening on another thread locally, but NOT in wasm land.

I can chime in with my own repro + fix and context. Helping w `macroquad` lib is one of my goals!

---

Auto publish my Rust wasm build to gh pages and/or itch.

Could do deploy of static page here
https://github.com/nathanleiby/drum-break/settings/pages

put in `web/` folder and make sure CI above uses that
