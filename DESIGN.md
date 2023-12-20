# Design

My thinking around core problems in this app.

## Audio Latency / Calibration


We probably *do not* want to play sounds too.
For example, if you're jamming on an edrum kit, you can use its sound to get instant feedback.
If we do play sounds on hit, we'll want to fire instantaneously as best we can, vs using the clock setup.
This might require us to handle input events that occur between frames, if that's possible

Getting the exact timing for input events, vs locking them to the next frame's timestamp, could also be useful
for better timing data collection.

- inspiration: https://ddrkirbyisq.medium.com/rhythm-quest-devlog-10-latency-calibration-fb6f1a56395c
https://rhythmquestgame.com/devlog/devlog.html
- https://exceed7.com/native-audio/rhythm-game-crash-course/index.html
- watch: https://www.youtube.com/watch?v=JTuZvRF-OgE&t=41s
