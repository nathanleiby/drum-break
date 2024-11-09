# Development

## Running

- install rust: https://www.rust-lang.org/tools/install
- clone this repo
- run it: `cargo run`

There are some helper scripts for local runs, too. See:

- `./dev.sh`
- `./dev-wasm.sh`

## Creating a release

- update the `VERSION` file
  - versioning scheme is semver with leading `v`, e.g. `v0.0.1`
- run `./release.sh`

To release the web version:

- run `./release-wasm.sh`
- commit the generated code (in `github_pages/`) and push

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
