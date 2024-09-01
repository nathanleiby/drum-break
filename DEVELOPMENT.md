# Development

## Running

- install rust: https://www.rust-lang.org/tools/install
- clone this repo
- run it: `cargo run`

## Creating a release

- update the `VERSION` file
  - versioning scheme is semver with leading `v`, e.g. `v0.0.1`
- run `./release.sh`

## System Design (TODO: Fix outdated content)

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

````
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
````

```mermaid
graph TD;
  ui
  audio
  input
  voices

  input --> audio
  audio --> ui


```
