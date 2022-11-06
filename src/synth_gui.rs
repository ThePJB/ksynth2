use crate::audio::*;
use crate::kapp::*;
use crate::kmath::*;
use crate::widgets::*;

pub struct VSliders {
    sub: FloatSlider,
    root: FloatSlider,
    mid: FloatSlider,
    high: FloatSlider,

    amp: FloatSlider,
    detune: FloatSlider,
    voices: FloatSlider,
}

impl Default for VSliders {
    fn default() -> Self {
        VSliders {
            sub: FloatSlider::new(0.0, 0.0, 1.0, "Sub".to_owned()),
            root: FloatSlider::new(1.0, 0.0, 1.0, "Root".to_owned()),
            mid: FloatSlider::new(1.0, 0.0, 1.0, "Mid".to_owned()),
            high: FloatSlider::new(1.0, 0.0, 1.0, "High".to_owned()),
            amp: FloatSlider::new(-10.0, -30.0, 0.0, "Amp".to_owned()),
            detune: FloatSlider::new(0.0, 0.0, 99.0, "Detune".to_owned()),
            voices: FloatSlider::new(1.0, 1.0, 8.0, "Voices".to_owned()),
        }
    }
}

impl VSliders {
    fn get_tf(&self, f: f32) -> TimbreFrame {
        TimbreFrame {
            f,
            sub: self.sub.curr(),
            root: self.root.curr(),
            mid: self.mid.curr(),
            high: self.high.curr(),
            amp: self.amp.curr(),
            detune: self.detune.curr(),
            voices: self.voices.curr(),
        }
    }
}

static S_PRESS: usize = 0;
static S_HELD: usize = 1;
static S_DONE: usize = 2;

pub struct SynthGUI {
    sliders: [VSliders; 3],
    curr: usize,
    curr_copy: Option<usize>,
    s_t1: FloatSlider,
    s_t2: FloatSlider,

    history: Vec<(usize, f32, f32)>,

    t_press: [Option<f32>; 28],
}

impl Default for SynthGUI {
    fn default() -> Self {
        SynthGUI {
            sliders: [VSliders::default(), VSliders::default(), VSliders::default()],
            curr: 0,
            curr_copy: None,
            s_t1: FloatSlider::new(0.5, 0.0, 3.0, "attack".to_owned()),
            s_t2: FloatSlider::new(0.5, 0.0, 3.0, "release".to_owned()),
            history: Vec::new(),
            t_press: [None; 28],
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
        let colours = [
            Vec4::new(0.9, 0.2, 0.2, 1.0),
            Vec4::new(0.2, 0.9, 0.2, 1.0),
            Vec4::new(0.2, 0.2, 0.9, 1.0),
        ];



        // key presses
        let pressed_keys = inputs.curr_keys.difference(&inputs.prev_keys);
        for k in pressed_keys {
            if let Some(note) = kc_to_note(*k) {
                self.t_press[note] = Some(inputs.t as f32);
                
                let f = 110.0 * 2.0f32.powf(note as f32/12.0);
                outputs.sounds.push(
                    SoundCommand {
                        id: note as u32,
                        sd: SoundDesc {
                            frames: [
                                self.sliders[0].get_tf(f),
                                self.sliders[1].get_tf(f),
                            ],
                            t: self.s_t1.curr(),
                            delete: false,
                        }
                    }
                )
            }
        }
        let released_keys = inputs.prev_keys.difference(&inputs.curr_keys);
        for k in released_keys {
            if let Some(note) = kc_to_note(*k) {
                let t_start = self.t_press[note].unwrap();
                let t_end = inputs.t as f32;
                self.t_press[note] = None;

                self.history.push((note, t_start, t_end));

                let f = 110.0 * 2.0f32.powf(note as f32/12.0);
                outputs.sounds.push(
                    SoundCommand {
                        id: note as u32,
                        sd: SoundDesc {
                            frames: [
                                self.sliders[1].get_tf(f),
                                self.sliders[2].get_tf(f),
                            ],
                            t: self.s_t2.curr(),
                            delete: true,
                        }
                    }
                )
            }
        }

        outputs.canvas.put_rect(inputs.screen_rect, 1.0, Vec4::grey(0.2));
        let r = inputs.screen_rect.dilate_pc(-0.003);
        {
            // top
            let r = r.grid_child(0, 0, 1, 3);
            let r = r.dilate_pc(-0.01);
            outputs.canvas.put_rect(r, 1.01, colours[self.curr]);

            {
                // selection pane
                let r = r.child(0.0, 0.0, 0.1, 1.0);
                let r = r.dilate_pc(-0.01);

                for i in 0..3 {
                    let r = r.grid_child(0, i, 1, 3);
                    let r = r.dilate_pc(-0.05);
                    if r.contains(inputs.mouse_pos) && inputs.lmb == KeyStatus::JustPressed {
                        self.curr = i as usize;
                    }
                    outputs.canvas.put_rect(r, 1.04, colours[i as usize]);
                    
                }
            }

            let r = r.child(0.1, 0.0, 0.9, 1.0);

            self.sliders[self.curr].sub.frame(inputs, outputs,    r.grid_child(0, 0, 7, 1));
            self.sliders[self.curr].root.frame(inputs, outputs,   r.grid_child(1, 0, 7, 1));
            self.sliders[self.curr].mid.frame(inputs, outputs,    r.grid_child(2, 0, 7, 1));
            self.sliders[self.curr].high.frame(inputs, outputs,   r.grid_child(3, 0, 7, 1));
            self.sliders[self.curr].amp.frame(inputs, outputs,    r.grid_child(4, 0, 7, 1));
            self.sliders[self.curr].detune.frame(inputs, outputs, r.grid_child(5, 0, 7, 1));
            self.sliders[self.curr].voices.frame(inputs, outputs, r.grid_child(6, 0, 7, 1));
        }
        {
            // mid
            let r = r.grid_child(0, 1, 1, 3);
            let r = r.dilate_pc(-0.01);
            outputs.canvas.put_rect(r, 1.01, Vec4::new(0.0, 0.8, 0.4, 1.0));
            
            {
                let r = r.grid_child(0, 0, 7, 1);
                self.s_t1.frame(inputs, outputs, r);
            }
            {
                let r = r.grid_child(1, 0, 7, 1);
                self.s_t2.frame(inputs, outputs, r);
            }
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

                if let Some(start) = self.t_press[row as usize] {
                    let end = inputs.t;
                    let r = r.child(1.0 - (inputs.t as f32 - start) / 10.0, 0.0, (end - start) / 10.0, 1.0);

                        let h = 90.0 * (row / 7) as f32;
                        let s = 1.0 - (row % 7) as f32 / 12.0;
                        let v = 1.0;

                        let c = Vec4::new(h, s, v, 1.0).hsv_to_rgb();
                        outputs.canvas.put_rect(r, 1.2, c);
                }

            }
        }
    }
}