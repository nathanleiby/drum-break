# macroix

Macroix is an experimental drum training tool.

![work in progress screenshot](./screenshot.png)

![scoring hits](./scores.png)

It is called Macroix based on "Macroquad" (rust game engine) + "LaCroix" (sparkling water).

The tool is in a pre-alpha state.

## Download

1. Download the latest [release-<version>.zip](https://github.com/nathanleiby/macroix/releases) from Github.
2. Unzip it
3. `cd release/`
4. Run `./macroix`.
   - On Mac, ou will likely need to open your security settings (roughly: https://www.macworld.com/article/672947/how-to-open-a-mac-app-from-an-unidentified-developer.html) and allow it to run.
     ![mac security settings](./mac-security-settings.png)

### Testing goals

- does it run?
- can you calibrate the audio latency?
- does it work with your midi input device (e.g. Akai MPK Mini, Alesis Nitro, TD17)?

## Development

### Running

- install rust: https://www.rust-lang.org/tools/install
- clone this repo
- run it: `cargo run`

### Creating a release

- update the `VERSION` file
  - versioning scheme is semver with leading `v`, e.g. `v0.0.1`
- run `./release.sh`

### System Design

What happens in a frame

```
- process
  - timing info (audio clock, actual)
  - user input (TODO: translate into events)
- update state
  - save user input
  - propagate corresponding game state
  - schedule upcoming audio events, if needed
- draw
```

What threads are running and need to be coordinated:

```
game
audio
input
```

How is the project organized

```
- Input -
```mermaid
graph TD;
  ui
  audio
  input

  events --> ui
  events --> audio

  input --> events
  audio --> events
  ui --> events
```

```mermaid
graph TD;
  ui
  audio
  input
  voices

  input --> audio
  audio --> ui


```
