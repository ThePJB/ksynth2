use crate::kmath::*;

pub fn db_to_vol(db: f32) -> f32 {
    10.0f32.powf(0.05 * db)
}

pub fn vol_to_db(vol: f32) -> f32 {
    20.0f32 * vol.log10()
}

pub struct SoundDesc {
    frames: [TimbreFrame; 3],
    t1: f32,
    t2: f32,
}

// what if it only is one lerp. adsr release can upload the new one anyway

impl SoundDesc {
    fn curr_tf(&self, t: f32, t_depress: bool) -> TimbreFrame {
        if held {
            if t > self.t1 {
                self.frames[1]
            } else {
                let t = t / self.t1;
                TimbreFrame {
                    f: lerp(self.frames[0].f, self.frames[1].f, t),
                    sub: lerp(self.frames[0].sub, self.frames[1].sub, t),
                    root: lerp(self.frames[0].root, self.frames[1].root, t),
                    mid: lerp(self.frames[0].mid, self.frames[1].mid, t),
                    high: lerp(self.frames[0].high, self.frames[1].high, t),
                    amp: lerp(self.frames[0].amp, self.frames[1].amp, t),
                    detune: lerp(self.frames[0].detune, self.frames[1].detune, t),
                    voices: lerp(self.frames[0].voices, self.frames[1].voices, t),
                }
            }
        } else {
            if t > self.t2 {
                self.frames[2]
            } else {
                let t = t / self.t2;
                TimbreFrame {
                    f: lerp(self.frames[0].f, self.frames[1].f, t),
                    sub: lerp(self.frames[0].sub, self.frames[1].sub, t),
                    root: lerp(self.frames[0].root, self.frames[1].root, t),
                    mid: lerp(self.frames[0].mid, self.frames[1].mid, t),
                    high: lerp(self.frames[0].high, self.frames[1].high, t),
                    amp: lerp(self.frames[0].amp, self.frames[1].amp, t),
                    detune: lerp(self.frames[0].detune, self.frames[1].detune, t),
                    voices: lerp(self.frames[0].voices, self.frames[1].voices, t),
                }
            }
        }
    }
}

pub enum SoundCommand {
    PlaySound(SoundDesc),
    PlaySoundID(u32, SoundDesc),
    SetID(u32, SoundDesc),
}

pub struct TimbreFrame {
    f: f32,
    sub: f32,
    root: f32,
    mid: f32,
    high: f32,
    amp: f32,
    detune: f32,
    voices: f32,
}

pub struct Channel {
    sd: SoundDesc,
    age: f32,
    phases: Vec<f32>,
    held: bool, // determines first or second lerp
}

impl Channel {
    pub fn tick(&mut self) -> f32 {
        let 

        let mut acc = 0.0;

        let mut n = 0;
        loop {
            n += 1;

            let f = self.f * n; 
            let an = 1.0 / f; // but * magnitude of harmonic

            // wow im retarded should do this later

            // fn harmonic of n
            // also sub, is what 1 oct lower?

        }
        for i in 0..self
        self.phases += self.f
    }
}

pub struct Mixer {
    channels: Vec<Channel>,
}

impl Mixer {
    pub fn handle_command(&mut self, sc: SoundCommand) {
        match sc {
            SoundCommand::PlaySound(sd) => {},
            SoundCommand::PlaySoundID(id, sd) => {},
            SoundCommand::SetID(id, sd) => {},
        }
    }

    pub fn tick(&mut self) -> f32 {
        self.channels.iter().map(|x| x.tick()).sum()
    }
}