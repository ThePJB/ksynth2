use crate::priority_queue::*;
use crate::kmath::*;

// mag <> db
pub fn db_to_vol(db: f32) -> f32 {
    10.0f32.powf(0.05 * db)
}

pub fn vol_to_db(vol: f32) -> f32 {
    20.0f32 * vol.log10()
}

pub struct Audio {

}

impl Audio {
    pub fn new() -> Audio {
        Audio {}
    }
}

pub enum Effect {
    Compressor{below_db: f32, below_ratio: f32, above_db: f32, above_ratio: f32},
}

// lets say id is 0..63 for now for effects
// otherwise we could do u32 and randomize
// easy enough to just use a pool in the game anyway
pub struct Mixer {
    sample_rate: f32,
    counter: u64,

    id: Vec<u32>,   // handle for deleting etc
    t1: Vec<u64>,   // time it was started
    t2: Vec<u64>,   // time lerp goes to
    f1: Vec<f32>,   // initial freq
    f2: Vec<f32>,   // lerp to freq
    a1: Vec<f32>,   // initial magnitude
    a2: Vec<f32>,   // lerp to magnitude
    phase: Vec<f32>,

    // how about a queue of up and coming sounds to so you can just put
    // several up like for an ADSR
    effects: Vec<(u32, Effect)>,    // id, effect

    pq: PriorityQueue<u64, MixerCommand>,
}

fn clerp(min: f32, max: f32, t: f32) -> f32 {
    let lerp_val = min + (max-min) * t;
    lerp_val.max(min).min(max)
}

impl Mixer {

    // so yeah we can apply a frame in the future, guess im just gonna have a queue of up and coomers
    // why not use my priority queue


    pub fn set_osc(&mut self, id: u32, dur: f32, f: f32, a: f32) {
        for i in 0..self.id.len() {
            if self.id[i] == id {
                let curr_t = (self.counter - self.t1[i]) as f32 / (self.t2[i] - self.t1[i]) as f32;
                self.t1[i] = self.counter;
                self.t2[i] = self.counter + (dur * self.sample_rate) as u64;
                self.f1[i] = clerp(self.f1[i], self.f2[i], curr_t);
                self.f2[i] = f;
                self.a1[i] = clerp(self.a1[i], self.a2[i], curr_t);
                self.a2[i] = a;

                return;
            }
        }
        // id not found
        self.id.push(id);
        self.t1.push(self.counter);
        self.t2.push(self.counter + (dur * self.sample_rate) as u64);
        self.f1.push(f);
        self.f2.push(f);
        self.a1.push(a);
        self.a2.push(a);
        self.phase.push(0.0);
    }

    pub fn handle_command(&mut self, c: MixerCommand) {
        let tnow = self.counter;
        match c {
            MixerCommand::SetOsc {
                id,
                from,
                dur,
                a,
                f,
            } => {
                if from == 0.0 {
                    self.set_osc(id, dur, f, a);
                } else {
                    self.pq.push(self.counter + (from * self.sample_rate) as u64, c);
                }
            },
            MixerCommand::SetEffect { id, effect } => {
                // blah blah blah
            }
        }
    }

    pub fn tick(&mut self) -> f32 {
        0.0
        // yeah 64 accumulators before applying effects
        // yeah getting close we need to peek the pq here
        // delete if it should be deleted: a of 0 reached
        // otherwise just insert that shit
        // dam should be able to delete from the pq as well
        // then its just a matter of boilerplate, id token mgmt
    }
}

#[derive(Clone)]
pub enum EffectType {
    // compression etc
}

#[derive(Clone)]
pub enum MixerCommand {
    // delete: duration and a2 0
    SetOsc {
        id: u32,
        from: f32,
        dur: f32,
        a: f32,
        f: f32,
    },
    SetEffect {
        id: u32,
        effect: EffectType,
    },
}