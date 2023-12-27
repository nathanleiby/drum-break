# Design

My thinking around core problems in this app.

## Audio Latency / Calibration

We probably _do not_ want to play sounds too.
For example, if you're jamming on an edrum kit, you can use its sound to get instant feedback.
If we do play sounds on hit, we'll want to fire instantaneously as best we can, vs using the clock setup.
This might require us to handle input events that occur between frames, if that's possible

Getting the exact timing for input events, vs locking them to the next frame's timestamp, could also be useful
for better timing data collection.

- inspiration: https://ddrkirbyisq.medium.com/rhythm-quest-devlog-10-latency-calibration-fb6f1a56395c
  https://rhythmquestgame.com/devlog/devlog.html
- https://exceed7.com/native-audio/rhythm-game-crash-course/index.html
- watch: https://www.youtube.com/watch?v=JTuZvRF-OgE&t=41s

## Accuracy

Possible approach:

- for each voice
  - maintain a reference to the next target_beat
    - if the user provides input, compute the accuracy
  - if it's a hit (non-Miss), register it and update the target_beat
  - it it's a miss, register it but DO NOT update the target_beat

## Drill types

- Exact drills
  - learn the verse, chorus, etc parts from X song
  - learn the Amen break
  - etc
- orchestration (play the same rhythm on different drums)
  - for example, keep a samba groove in LF, RF, RH, but then orchestrate the LH around the kit
  - tests that you can "keep the groove going" while exploring around
- Generative drills - e.g. mix and match to amp up the difficulty curve to meet the user
  - beat variations like:
    - various hh/ride ostinatos
    - various kick ostinatos
    - various snare rhythms
      - ghost notes(?)
    - pedal hi hat(?)
  - static or evolving?
    - could be a drill that doesn't change
    - could be a sightreading thing (it changes each time you ace a loop 5x)
- polyrhythm exerices
  - polyrhythm songs could be a exact drill
  - evolving rhythms could be a generative drill
- skill based
  - double kick
  - blast beats
  - flams
  - paradiddles (how to deal with LH / RH? one option could be diff drum for each)
- fills
- accents / dynamics

## Displaying accuracy

"Did you _ace_ the loop?"

- `100%` correct
- `0` early late
- `0` missed (incl: no extra hits)

"Did you _great_ the loop?"

- `>= 90%` correct
- `< 10%` early or late
- `0` missed (incl: no extra hits)
