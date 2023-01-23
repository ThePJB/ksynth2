use crate::audio::*;
use crate::kapp::*;
use crate::kmath::*;
use crate::widgets::*;
use std::collections::HashMap;

pub struct VSliders {
        n: FloatSlider,
        troll: FloatSlider,
        ea: FloatSlider,
        ed: FloatSlider,
        es: FloatSlider,
        er: FloatSlider,
        detune: FloatSlider,
        voices: FloatSlider,
        amp: FloatSlider,
        cut: FloatSlider,
        cur: FloatSlider,
        cdt: FloatSlider,
        cdr: FloatSlider,
        aout: FloatSlider,
}

impl Default for VSliders {
    fn default() -> Self {
        VSliders {
            n: FloatSlider::new(1.0, 1.0, 30.0, "NHARM".to_owned()),
            troll: FloatSlider::new(1.0, 1.0, 4.0, "TROLL".to_owned()),

            ea: FloatSlider::new(0.1, 0.0, 3.0, "ATTACK".to_owned()),
            ed: FloatSlider::new(0.1, 0.0, 3.0, "DECAY".to_owned()),
            es: FloatSlider::new(0.5, 0.0, 1.0, "SUSTAIN".to_owned()),
            er: FloatSlider::new(0.1, 0.0, 3.0, "RELEASE".to_owned()),

            detune: FloatSlider::new(0.0, 0.0, 99.0, "DETUNE".to_owned()),
            voices: FloatSlider::new(1.0, 1.0, 8.0, "VOICES".to_owned()),

            amp: FloatSlider::new(-20.0, -30.0, 10.0, "AMP-PRE".to_owned()),
            
            cut: FloatSlider::new(-100.0, -100.0, 10.0, "C-UP-T".to_owned()),
            cur: FloatSlider::new(1.0, 1.0, 8.0, "C-UP-R".to_owned()),
            
            cdt: FloatSlider::new(0.0, -30.0, 20.0, "C-DOWN-T".to_owned()),
            cdr: FloatSlider::new(1.0, 1.0, 8.0, "C-DOWN-R".to_owned()),

            aout: FloatSlider::new(-20.0, -30.0, 10.0, "AMP-OUT".to_owned()),
        }
    }
}

impl VSliders {
    fn get_sd(&self, f: f32) -> SoundDesc {
        SoundDesc {
            f,
            n: self.n.curr(),
            troll: self.troll.curr(),
            ea: self.ea.curr(),
            ed: self.ed.curr(),
            es: self.es.curr(),
            er: self.er.curr(),
            detune: self.detune.curr(),
            voices: self.voices.curr(),
            amp: self.amp.curr(),
            cut: self.cut.curr(),
            cur: self.cur.curr(),
            cdt: self.cdt.curr(),
            cdr: self.cdr.curr(),
            aout: self.aout.curr(),
        }
    }
}
impl Knobs {
    fn get_sd(&self, f: f32) -> SoundDesc {
        SoundDesc {
            f,
            n: self.n.curr(),
            troll: self.troll.curr(),
            ea: self.a.curr(),
            ed: self.d.curr(),
            es: self.s.curr(),
            er: self.r.curr(),
            detune: self.detune.curr(),
            voices: self.voices.curr(),
            amp: self.amp.curr(),
            cut: self.cut.curr(),
            cur: self.cur.curr(),
            cdt: self.cdt.curr(),
            cdr: self.cdr.curr(),
            aout: self.aout.curr(),
        }
    }
}

pub struct Knobs {
    pub a: Knob,
    pub d: Knob,
    pub s: Knob,
    pub r: Knob,
    
    pub n: Knob,
    pub troll: Knob,
    pub detune: Knob,
    pub voices: Knob,
    
    pub amp: Knob,
    pub cut: Knob,
    pub cur: Knob,
    pub cdt: Knob,
    pub cdr: Knob,

    pub aout: Knob,
}

// compressor needs do upward, do downward

impl Default for Knobs {
    fn default() -> Self {
        Knobs {
            a: Knob::new(0.1, 0.0, 2.0, 0.001, "Attack"),
            d: Knob::new(0.1, 0.0, 2.0, 0.001, "Decay"),
            s: Knob::new(0.5, 0.0, 1.0, 0.001, "Sustain"),
            r: Knob::new(0.1, 0.0, 2.0, 0.001, "Release"),

            n: Knob::new(3.0, 1.0, 20.0, 0.001, "Num Harmonics"),
            troll: Knob::new(2.0, 1.0, 5.0, 0.001, "Exponent"),
            voices: Knob::new(2.0, 2.0, 9.0, 0.001, "Voices"),
            detune: Knob::new(0.0, 0.0, 99.0, 0.001, "Detune"),
            aout: Knob::new(0.05, 0.0, 0.3, 0.001, "volume"),
            
            amp: Knob::new(0.1, 0.0, 30.0, 0.001, "Amplitude"),
            cut: Knob::new(0.0, 0.0, 1.0, 0.001, "up threshold"),
            cur: Knob::new(1.0, 1.0, 4.0, 0.001, "up ratio"),
            cdt: Knob::new(1.0, 0.0, 1.0, 0.001, "down threshold"),
            cdr: Knob::new(1.0, 1.0, 4.0, 0.001, "down ratio"),
            
        }
    }
}

pub struct SynthGUI {
    sliders: VSliders,
    knobs: Knobs,

    history: Vec<(usize, f32, f32)>,

    held_keys: HashMap<u32, (usize, f32, SoundDesc)>,
    times_pressed: [u32; 28],
}

impl Default for SynthGUI {
    fn default() -> Self {
        SynthGUI {
            knobs: Knobs::default(),
            sliders: VSliders::default(),
            history: Vec::new(),
            held_keys: HashMap::new(),
            times_pressed: [0; 28],
        }
    }
}

fn kc_to_note(kc: VirtualKeyCode) -> Option<usize> {
    match kc {
        VirtualKeyCode::Z => Some(0),
        VirtualKeyCode::X => Some(1),
        VirtualKeyCode::C => Some(2),
        VirtualKeyCode::V => Some(3),
        VirtualKeyCode::B => Some(4),
        VirtualKeyCode::N => Some(5),
        VirtualKeyCode::M => Some(6),

        VirtualKeyCode::A => Some(7),
        VirtualKeyCode::S => Some(8),
        VirtualKeyCode::D => Some(9),
        VirtualKeyCode::F => Some(10),
        VirtualKeyCode::G => Some(11),
        VirtualKeyCode::H => Some(12),
        VirtualKeyCode::J => Some(13),

        VirtualKeyCode::Q => Some(14),
        VirtualKeyCode::W => Some(15),
        VirtualKeyCode::E => Some(16),
        VirtualKeyCode::R => Some(17),
        VirtualKeyCode::T => Some(18),
        VirtualKeyCode::Y => Some(19),
        VirtualKeyCode::U => Some(20),

        VirtualKeyCode::Key1 => Some(21),
        VirtualKeyCode::Key2 => Some(22),
        VirtualKeyCode::Key3 => Some(23),
        VirtualKeyCode::Key4 => Some(24),
        VirtualKeyCode::Key5 => Some(25),
        VirtualKeyCode::Key6 => Some(26),
        VirtualKeyCode::Key7 => Some(27),

        _ => None,
    }
}

impl SynthGUI {
    pub fn frame(&mut self, inputs: &FrameInputState, outputs: &mut FrameOutputs) {
        let w = 15;

        // key presses
        let pressed_keys = inputs.curr_keys.difference(&inputs.prev_keys);
        for k in pressed_keys {
            if let Some(note) = kc_to_note(*k) {
                self.times_pressed[note] += 1;
                let uid = (31249577 + self.times_pressed[note]) * khash(12312577 * note as u32);
                let f = 110.0 * 2.0f32.powf(note as f32/12.0);
                let sd = self.knobs.get_sd(f);
                self.held_keys.insert(uid, (note, inputs.t, sd));
                outputs.sounds.push(
                    AudioCommand::PlayHold(uid as u64, sd)
                )
            }
        }
        let released_keys = inputs.prev_keys.difference(&inputs.curr_keys);
        for k in released_keys {
            if let Some(note) = kc_to_note(*k) {
                let uid = (31249577 + self.times_pressed[note]) * khash(12312577 * note as u32);
                if let Some((_note, t_start, _sd)) = self.held_keys.remove(&uid) {
                    let t_end = inputs.t as f32;
                    self.history.push((note, t_start, t_end));
                    outputs.sounds.push(
                        AudioCommand::Release(uid as u64)
                    )
                }
            }
        }

        outputs.canvas.put_rect(inputs.screen_rect, 1.0, Vec4::grey(0.2));
        let r = inputs.screen_rect.dilate_pc(-0.003);

        {
            let w_envelope = 0.3333;
            let w_osc = 0.3333;

            // top
            let r = r.grid_child(0, 0, 1, 3);

            // envelope section
            let r = r.child(0.0, 0.0, w_envelope, 1.0);
            {
                let r = r.dilate_pc(-0.01);
                outputs.canvas.put_rect(r, 1.01, Vec4::new(0.9, 0.2, 0.2, 1.0));
                outputs.glyphs.push_center_str("envelope", r.x + r.w/2.0, r.y + 0.1*r.h/2.0, 0.1*r.h/2.5, 0.1*r.h/2.5, 1.2, v4(1.0, 1.0, 1.0, 1.0));
                let r = r.child(0.0, 0.1, 1.0, 0.9);
                {
                    let r = r.dilate_pc(-0.01);
                    let r = r.grid_child(0, 0, 5, 1);
                    self.knobs.a.frame(inputs, outputs, r.grid_child(0, 0, 1, 4));
                    self.knobs.d.frame(inputs, outputs, r.grid_child(0, 1, 1, 4));
                    self.knobs.s.frame(inputs, outputs, r.grid_child(0, 2, 1, 4));
                    self.knobs.r.frame(inputs, outputs, r.grid_child(0, 3, 1, 4));
                }
            }
            
            let r = r.child(1.0, 0.0, 1.0, 1.0);
            {
                let r = r.dilate_pc(-0.01);
                outputs.canvas.put_rect(r, 1.01, Vec4::new(0.9, 0.2, 0.2, 1.0));
                outputs.glyphs.push_center_str("oscillator", r.x + r.w/2.0, r.y + 0.1*r.h/2.0, 0.1*r.h/2.5, 0.1*r.h/2.5, 1.2, v4(1.0, 1.0, 1.0, 1.0));
                let r = r.child(0.0, 0.1, 1.0, 0.9);
                {
                    let r = r.dilate_pc(-0.01);
                    self.knobs.n.frame(inputs, outputs, r.grid_child(0, 0, 2, 4));
                    self.knobs.troll.frame(inputs, outputs, r.grid_child(0, 1, 2, 4));
                    self.knobs.detune.frame(inputs, outputs, r.grid_child(0, 2, 2, 4));
                    self.knobs.voices.frame(inputs, outputs, r.grid_child(0, 3, 2, 4));
                    self.knobs.aout.frame(inputs, outputs, r.grid_child(1, 0, 2, 4));
                }
            }
            let r = r.child(1.0, 0.0, 1.0, 1.0);
            {
                let r = r.dilate_pc(-0.01);
                outputs.canvas.put_rect(r, 1.01, Vec4::new(0.9, 0.2, 0.2, 1.0));
                outputs.glyphs.push_center_str("compressor", r.x + r.w/2.0, r.y + 0.1*r.h/2.0, 0.1*r.h/2.5, 0.1*r.h/2.5, 1.2, v4(1.0, 1.0, 1.0, 1.0));
                let r = r.child(0.0, 0.1, 1.0, 0.9);
                {
                    let r = r.dilate_pc(-0.01);
                    self.knobs.amp.frame(inputs, outputs, r.grid_child(0, 0, 3, 4));
                    self.knobs.cut.frame(inputs, outputs, r.grid_child(1, 0, 3, 4));
                    self.knobs.cur.frame(inputs, outputs, r.grid_child(1, 1, 3, 4));
                    self.knobs.cdt.frame(inputs, outputs, r.grid_child(2, 0, 3, 4));
                    self.knobs.cdr.frame(inputs, outputs, r.grid_child(2, 1, 3, 4));
                }
            }

            // self.sliders.n.frame(inputs, outputs,    r.grid_child(0, 0, w, 1));
            // self.sliders.troll.frame(inputs, outputs,   r.grid_child(1, 0, w, 1));
            // self.sliders.ea.frame(inputs, outputs,   r.grid_child(2, 0, w, 1));
            // self.sliders.ed.frame(inputs, outputs,   r.grid_child(3, 0, w, 1));
            // self.sliders.es.frame(inputs, outputs,   r.grid_child(4, 0, w, 1));
            // self.sliders.er.frame(inputs, outputs,   r.grid_child(5, 0, w, 1));
            // self.sliders.detune.frame(inputs, outputs,   r.grid_child(6, 0, w, 1));
            // self.sliders.voices.frame(inputs, outputs,   r.grid_child(7, 0, w, 1));
            // self.sliders.amp.frame(inputs, outputs,   r.grid_child(8, 0, w, 1));
            // self.sliders.cur.frame(inputs, outputs,   r.grid_child(10, 0, w, 1));
            // self.sliders.cut.frame(inputs, outputs,   r.grid_child(11, 0, w, 1));
            // self.sliders.cdr.frame(inputs, outputs,   r.grid_child(12, 0, w, 1));
            // self.sliders.cdt.frame(inputs, outputs,   r.grid_child(13, 0, w, 1));
            // self.sliders.aout.frame(inputs, outputs,   r.grid_child(14, 0, w, 1));
        }

        // {
        //     // mid
        //     let r = r.grid_child(0, 1, 1, 3);
        //     let r = r.dilate_pc(-0.01);
        //     outputs.canvas.put_rect(r, 1.01, Vec4::new(0.0, 0.8, 0.4, 1.0));
        // }

        {
            // bot
            let r = r.grid_child(0, 2, 1, 3);
            let r = r.dilate_pc(-0.01);
            outputs.canvas.put_rect(r, 1.01, Vec4::new(0.0, 0.4, 0.8, 1.0));

            // draw history notes
            for &(note, start, end) in self.history.iter() {
                if end > inputs.t as f32 - 10.0 {
                    let r = r.grid_child(0, 27 - note as i32, 1, 28);
                    let r = r.child(1.0 - (inputs.t as f32 - start) / 10.0, 0.0, (end - start) / 10.0, 1.0);

                    let h = 90.0 * (note / 7) as f32;
                    let s = 1.0 - (note % 7) as f32 / 12.0;
                    let v = 1.0;

                    let c = Vec4::new(h, s, v, 1.0).hsv_to_rgb();
                    outputs.canvas.put_rect(r, 1.2, c);
                }
            }

            // draw held notes
            for &(note, start, _sd) in self.held_keys.values() {
                let r = r.grid_child(0, 27 - note as i32, 1, 28);
                let end = inputs.t;
                let r = r.child(1.0 - (inputs.t as f32 - start) / 10.0, 0.0, (end-start) / 10.0, 1.0);

                let h = 90.0 * (note / 7) as f32;
                let s = 1.0 - (note % 7) as f32 / 12.0;
                let v = 1.0;

                let c = Vec4::new(h, s, v, 1.0).hsv_to_rgb();
                outputs.canvas.put_rect(r, 1.2, c);
            }
        }
    }
}