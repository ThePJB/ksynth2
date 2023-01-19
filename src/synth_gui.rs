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

pub struct SynthGUI {
    sliders: VSliders,

    history: Vec<(usize, f32, f32)>,

    held_keys: HashMap<u32, (f32, SoundDesc)>,
    times_pressed: [u32; 28],
}

impl Default for SynthGUI {
    fn default() -> Self {
        SynthGUI {
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
                let sd = self.sliders.get_sd(f);
                self.held_keys.insert(uid, (inputs.t, sd));
                outputs.sounds.push(
                    AudioCommand::PlayHold(uid as u64, sd)
                )
            }
        }
        let released_keys = inputs.prev_keys.difference(&inputs.curr_keys);
        for k in released_keys {
            if let Some(note) = kc_to_note(*k) {
                let uid = (31249577 + self.times_pressed[note]) * khash(12312577 * note as u32);
                if let Some((t_start, sd)) = self.held_keys.remove(&uid) {
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
            // top
            let r = r.grid_child(0, 0, 1, 3);
            let r = r.dilate_pc(-0.01);
            outputs.canvas.put_rect(r, 1.01, Vec4::new(0.9, 0.2, 0.2, 1.0));


            self.sliders.n.frame(inputs, outputs,    r.grid_child(0, 0, w, 1));
            self.sliders.troll.frame(inputs, outputs,   r.grid_child(1, 0, w, 1));
            self.sliders.ea.frame(inputs, outputs,   r.grid_child(2, 0, w, 1));
            self.sliders.ed.frame(inputs, outputs,   r.grid_child(3, 0, w, 1));
            self.sliders.es.frame(inputs, outputs,   r.grid_child(4, 0, w, 1));
            self.sliders.er.frame(inputs, outputs,   r.grid_child(5, 0, w, 1));
            self.sliders.detune.frame(inputs, outputs,   r.grid_child(6, 0, w, 1));
            self.sliders.voices.frame(inputs, outputs,   r.grid_child(7, 0, w, 1));
            self.sliders.amp.frame(inputs, outputs,   r.grid_child(8, 0, w, 1));
            self.sliders.cur.frame(inputs, outputs,   r.grid_child(10, 0, w, 1));
            self.sliders.cut.frame(inputs, outputs,   r.grid_child(11, 0, w, 1));
            self.sliders.cdr.frame(inputs, outputs,   r.grid_child(12, 0, w, 1));
            self.sliders.cdt.frame(inputs, outputs,   r.grid_child(13, 0, w, 1));
            self.sliders.aout.frame(inputs, outputs,   r.grid_child(14, 0, w, 1));
        }
        {
            // mid
            let r = r.grid_child(0, 1, 1, 3);
            let r = r.dilate_pc(-0.01);
            outputs.canvas.put_rect(r, 1.01, Vec4::new(0.0, 0.8, 0.4, 1.0));
        }
        {
            // bot
            let r = r.grid_child(0, 2, 1, 3);
            let r = r.dilate_pc(-0.01);
            outputs.canvas.put_rect(r, 1.01, Vec4::new(0.0, 0.4, 0.8, 1.0));


            
            for row in 0..28 {
                let r = r.grid_child(0, 27 - row, 1, 28);
                for &(note, start, end) in self.history.iter() {
                    if note == row as usize && end > inputs.t as f32 - 10.0 {
                        let r = r.child(1.0 - (inputs.t as f32 - start) / 10.0, 0.0, (end - start) / 10.0, 1.0);

                        let h = 90.0 * (row / 7) as f32;
                        let s = 1.0 - (row % 7) as f32 / 12.0;
                        let v = 1.0;

                        let c = Vec4::new(h, s, v, 1.0).hsv_to_rgb();
                        outputs.canvas.put_rect(r, 1.2, c);
                    }
                }

                // if let Some(start) = self.t_press[row as usize] {
                //     let end = inputs.t;
                //     let r = r.child(1.0 - (inputs.t as f32 - start) / 10.0, 0.0, (end - start) / 10.0, 1.0);

                //         let h = 90.0 * (row / 7) as f32;
                //         let s = 1.0 - (row % 7) as f32 / 12.0;
                //         let v = 1.0;

                //         let c = Vec4::new(h, s, v, 1.0).hsv_to_rgb();
                //         outputs.canvas.put_rect(r, 1.2, c);
                // }

            }
        }
    }
}

// yes plot with adsr
// uh how to plot currently held, guess by them instead of by row
// yea cause thats dum