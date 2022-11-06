use crate::kmath::*;

pub fn db_to_vol(db: f32) -> f32 {
    10.0f32.powf(0.05 * db)
}

pub fn vol_to_db(vol: f32) -> f32 {
    20.0f32 * vol.log10()
}

#[derive(Clone, Copy)]
pub struct SoundDesc {
    pub frames: [TimbreFrame; 2],
    pub t: f32,
    pub delete: bool,
}

// what if it only is one lerp. adsr release can upload the new one anyway

impl SoundDesc {
    fn curr_tf(&self, t: f32) -> TimbreFrame {
        if t > self.t {
            self.frames[1]
        } else {
            let t = t / self.t;
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

#[derive(Clone, Copy)]
pub struct SoundCommand {
    pub sd: SoundDesc,
    pub id: u32,
}

#[derive(Clone, Copy)]
pub struct TimbreFrame {
    pub f: f32,
    pub sub: f32,
    pub root: f32,
    pub mid: f32,
    pub high: f32,
    pub amp: f32,
    pub detune: f32,
    pub voices: f32,
}

pub struct Channel {
    pub sd: SoundDesc,
    pub age: f32,
    pub phases: Vec<[f32; 20]>,
    pub id: u32,
}

impl Channel {
    pub fn tick(&mut self) -> f32 {
        self.age += 1.0 / 44100.0;

        // return (self.sd.frames[0].f * self.age * 2.0 * PI).sin();

        let tf = self.sd.curr_tf(self.age);

        let voices_len = tf.voices.floor() as usize;
        self.phases.resize(voices_len, [0.0; 20]);  // or resize with random phases, is better

        let mut acc = 0.0;

        let a_vol = db_to_vol(tf.amp);

        for detune_voice_num in 0..voices_len {
            for n in 0..20 {
                let f = if n == 0 {
                    tf.f / 2.0
                } else {
                    n as f32 * tf.f
                };

                let local_a = if n == 0 {
                    tf.sub
                } else if n == 1 {
                    tf.root
                } else if n < 4 {
                    tf.mid
                } else {
                    tf.high
                };

                let a_roll = if n <= 1 {
                    1.0
                } else {
                    1.0 / n as f32
                };

                let detune_interval = 2.0f32.powf(tf.detune / 1200.0);
                let f = f * detune_interval.powf(detune_voice_num as f32);

                self.phases[detune_voice_num as usize][n as usize] = (self.phases[detune_voice_num as usize][n as usize] + f / 44100.0).fract();
                acc += a_roll * a_vol * local_a * (2.0 * PI * self.phases[detune_voice_num as usize][n as usize]).sin();
            }
        }
        acc
    }

    pub fn should_remove(&self) -> bool {
        self.sd.delete && self.age > self.sd.t
    }
}

#[derive(Default)]
pub struct Mixer {
    pub sample_count: u64,
    pub channels: Vec<Channel>,
}

impl Mixer {
    pub fn handle_command(&mut self, sc: SoundCommand) {
        println!("handle command id {} delete {}", sc.id, sc.sd.delete);
        for i in 0..self.channels.len() {
            // if its already playing we may want to blend
            if self.channels[i].id == sc.id {

                let curr = self.channels[i].sd.curr_tf(self.channels[i].age);

                self.channels[i].sd = sc.sd;

                self.channels[i].sd.frames[0] = curr; // blend

                self.channels[i].age = 0.0;
                return;
            }
        }
        self.channels.push(Channel {
            sd: sc.sd,
            id: sc.id,
            age: 0.0,
            phases: Vec::new(), // todo preallocate max phases for detune
        });
    }

    pub fn tick(&mut self) -> f32 {
        self.sample_count += 1;

        let mut i = self.channels.len();
        if i == 0 { return 0.0 }
        i -= 1;
        let mut acc = 0.0;
        loop {
            acc += self.channels[i].tick();
            
            if self.channels[i].should_remove() { 
                self.channels.swap_remove(i);
            }

            if i == 0 { break; }
            i -= 1;
        }
        acc
    }
}