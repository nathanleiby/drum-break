# macroix

Macroix is an experimental drum training tool.

It is called Macroix based on "Macroquad" (rust game engine) + "LaCroix" (sparkling water).

The tool is in a pre-alpha state.

To run it, you can either:

- build it and run it yourself
  - install rust: https://www.rust-lang.org/tools/install
  - clone this repo
  - run it: `cargo run`
- download a [release](https://github.com/nathanleiby/macroix/releases) from Github
    - make it executable: `chmod +x macroix`
    - run it: `./macroix`
      - you will likely need to open your security settings in Mac (roughly: https://www.macworld.com/article/672947/how-to-open-a-mac-app-from-an-unidentified-developer.html) and allow it to run

Testing goals:
- does it run?
- can you calibrate the audio latency?
- does it work with you midi input device (e.g. Akai MPK Mini, Alesis Nitro, TD17)?

