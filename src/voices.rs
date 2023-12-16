/// Voices represents the notes to be played on each instrument.
pub struct Voices {
    pub metronome: Vec<f64>,
    pub closed_hihat: Vec<f64>,
    pub snare: Vec<f64>,
    pub kick: Vec<f64>,
    pub open_hihat: Vec<f64>,
}

impl Voices {
    pub fn new_samba() -> Self {
        // let lambda = |x: f64| (x - 1.) / 2.; // 8 quarter note beats per loop
        let lambda = |x: f64| (x - 1.);
        let closed_hihat_notes = vec![1., 3., 4., 5., 7., 8., 9., 11., 12., 13., 15., 16.]
            .into_iter()
            .map(lambda)
            .collect();
        let snare_notes = vec![1., 3., 6., 8., 10., 13., 15.]
            .into_iter()
            .map(lambda)
            .collect();
        let kick_notes: Vec<f64> = vec![1., 4., 5., 8., 9., 12., 13., 16.]
            .into_iter()
            .map(lambda)
            .collect();
        let open_hihat_notes: Vec<f64> = vec![3., 7., 11., 15.].into_iter().map(lambda).collect();
        let metronome_notes: Vec<f64> = (0..16).into_iter().map(|x| x as f64).collect();
        Self {
            metronome: metronome_notes,
            closed_hihat: closed_hihat_notes,
            snare: snare_notes,
            kick: kick_notes,
            open_hihat: open_hihat_notes,
        }
    }

    // TODO: new from file

    pub fn toggle_beat(&mut self, row: f64, beat: f64) {
        if row == 0. {
            if let Some(pos) = self.closed_hihat.iter().position(|x| *x == beat) {
                self.closed_hihat.remove(pos);
            } else {
                self.closed_hihat.push(beat);
            }
        } else if row == 1. {
            if let Some(pos) = self.snare.iter().position(|x| *x == beat) {
                self.snare.remove(pos);
            } else {
                self.snare.push(beat);
            }
        } else if row == 2. {
            if let Some(pos) = self.kick.iter().position(|x| *x == beat) {
                self.kick.remove(pos);
            } else {
                self.kick.push(beat);
            }
        } else if row == 3. {
            if let Some(pos) = self.open_hihat.iter().position(|x| *x == beat) {
                self.open_hihat.remove(pos);
            } else {
                self.open_hihat.push(beat);
            }
        }
    }
}
