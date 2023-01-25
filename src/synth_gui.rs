use crate::audio::*;
use crate::kapp::*;
use crate::kmath::*;
use crate::texture_buffer::TextureBuffer;
use crate::widgets::*;

use std::collections::HashMap;

use rustfft::num_complex::ComplexFloat;
use rustfft::{FftPlanner, num_complex::Complex};

const FFT_SIZE: usize = 8192;
const NUM_NOTES: usize = 34;

pub struct Knobs {
    pub a: Knob,
    pub d: Knob,
    pub s: Knob,
    pub r: Knob,
    
    pub n: Knob,
    pub troll: Knob,
    pub detune: Knob,
    pub voices: Knob,
    pub base_freq: Knob,
    
    pub amp: Knob,
    pub cut: Knob,
    pub cur: Knob,
    pub cdt: Knob,
    pub cdr: Knob,
    pub hard_clip: Knob,

    pub aout: Knob,
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
            hard_clip: self.hard_clip.curr(),
        }
    }
}

impl Default for Knobs {
    fn default() -> Self {
        Knobs {
            a: Knob::new(0.1, 0.0, 2.0, 0.001, "Attack"),
            d: Knob::new(0.1, 0.0, 2.0, 0.001, "Decay"),
            s: Knob::new(0.5, 0.0, 1.0, 0.001, "Sustain"),
            r: Knob::new(0.1, 0.0, 2.0, 0.001, "Release"),

            n: Knob::new(3.0, 1.0, 20.0, 0.001, "Num Harmonics"),
            troll: Knob::new(2.0, 1.0, 5.0, 0.001, "Exponent"),
            voices: Knob::new(1.0, 1.0, 9.0, 0.001, "Voices"),
            detune: Knob::new(0.0, 0.0, 99.0, 0.001, "Detune"),
            aout: Knob::new(-10.0, -80.0, 20.0, 0.001, "volume"),
            base_freq: Knob::new(110.0, 20.0, 880.0, 0.001, "Base Frequency"),

            amp: Knob::new(-30.0, -60.0, 30.0, 0.001, "Amplitude"),
            cut: Knob::new(-100.0, -100.0, 0.0, 0.001, "up threshold"),
            cur: Knob::new(1.0, 1.0, 16.0, 0.001, "up ratio"),
            cdt: Knob::new(0.0, -100.0, 0.0, 0.001, "down threshold"),
            cdr: Knob::new(1.0, 1.0, 16.0, 0.001, "down ratio"),
            hard_clip: Knob::new(0.0, -100.0, 0.0, 0.001, "hard clip db"),
            
        }
    }
}

pub struct SynthGUI {
    knobs: Knobs,

    history: Vec<(usize, f32, f32)>,

    held_keys: HashMap<u32, (usize, f32, SoundDesc)>,
    times_pressed: HashMap<VirtualKeyCode, u32>,

    local_mixer: Mixer,
    sample_ringbuf: [f32; FFT_SIZE],
    rb_head: usize,
}

impl Default for SynthGUI {
    fn default() -> Self {
        SynthGUI {
            knobs: Knobs::default(),
            history: Vec::new(),
            held_keys: HashMap::new(),
            times_pressed: HashMap::new(),
            local_mixer: Mixer::default(),
            sample_ringbuf: [0.0; FFT_SIZE],
            rb_head: 0,
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
        VirtualKeyCode::Comma => Some(7),
        VirtualKeyCode::Period => Some(8),
        VirtualKeyCode::Slash => Some(9),
        VirtualKeyCode::RShift => Some(10),

        VirtualKeyCode::Capital => Some(6),
        VirtualKeyCode::A => Some(7),
        VirtualKeyCode::S => Some(8),
        VirtualKeyCode::D => Some(9),
        VirtualKeyCode::F => Some(10),
        VirtualKeyCode::G => Some(11),
        VirtualKeyCode::H => Some(12),
        VirtualKeyCode::J => Some(13),
        VirtualKeyCode::K => Some(14),
        VirtualKeyCode::L => Some(15),
        VirtualKeyCode::Semicolon => Some(16),
        VirtualKeyCode::Apostrophe => Some(17),
        VirtualKeyCode::Return => Some(18),

        VirtualKeyCode::Tab => Some(13),
        VirtualKeyCode::Q => Some(14),
        VirtualKeyCode::W => Some(15),
        VirtualKeyCode::E => Some(16),
        VirtualKeyCode::R => Some(17),
        VirtualKeyCode::T => Some(18),
        VirtualKeyCode::Y => Some(19),
        VirtualKeyCode::U => Some(20),
        VirtualKeyCode::I => Some(21),
        VirtualKeyCode::O => Some(22),
        VirtualKeyCode::P => Some(23),
        VirtualKeyCode::LBracket => Some(24),
        VirtualKeyCode::RBracket => Some(25),
        VirtualKeyCode::Backslash => Some(26),

        VirtualKeyCode::Escape => Some(20),
        VirtualKeyCode::Key1 => Some(21),
        VirtualKeyCode::Key2 => Some(22),
        VirtualKeyCode::Key3 => Some(23),
        VirtualKeyCode::Key4 => Some(24),
        VirtualKeyCode::Key5 => Some(25),
        VirtualKeyCode::Key6 => Some(26),
        VirtualKeyCode::Key7 => Some(27),

        VirtualKeyCode::Key8 => Some(28),
        VirtualKeyCode::Key9 => Some(29),
        VirtualKeyCode::Key0 => Some(30),
        VirtualKeyCode::Minus => Some(31),
        VirtualKeyCode::Equals => Some(32),
        VirtualKeyCode::Back => Some(33),

        _ => None,
    }
}

impl SynthGUI {
    pub fn frame(&mut self, inputs: &FrameInputState, outputs: &mut FrameOutputs) {
        // key presses
        let pressed_keys = inputs.curr_keys.difference(&inputs.prev_keys);
        for k in pressed_keys {
            if let Some(note) = kc_to_note(*k) {
                let uid = (31249577 + self.times_pressed.get(k).unwrap_or(&0)) * khash(12312577 * note as u32);
                let f = self.knobs.base_freq.curr() * 2.0f32.powf(note as f32/12.0);
                let sd = self.knobs.get_sd(f);
                self.held_keys.insert(uid, (note, inputs.t, sd));

                let com = AudioCommand::PlayHold(uid as u64, sd);
                self.local_mixer.handle_command(com);
                outputs.sounds.push(
                    com,
                )
            }
        }
        let released_keys = inputs.prev_keys.difference(&inputs.curr_keys);
        for k in released_keys {
            if let Some(note) = kc_to_note(*k) {
                let uid = (31249577 + self.times_pressed.get(k).unwrap_or(&0)) * khash(12312577 * note as u32);
                if let Some((_note, t_start, _sd)) = self.held_keys.remove(&uid) {
                    let t_end = inputs.t as f32;
                    self.history.push((note, t_start, t_end));
                    let com = AudioCommand::Release(uid as u64);
                    self.local_mixer.handle_command(com);
                    outputs.sounds.push(
                        com,
                    )
                }
                self.times_pressed.insert(*k, *self.times_pressed.get(k).unwrap_or(&0) + 1);

            }
        }

        outputs.canvas.put_rect(inputs.screen_rect, 1.0, Vec4::grey(0.2));
        let r = inputs.screen_rect.dilate_pc(-0.003);

        {
            let w_envelope = 0.3333;

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
            {
                // adsr visualizer
                let r = r.dilate_pc(-0.01);
                let r = r.child(0.0, 0.1, 1.0, 0.9);
                let r = r.child(0.2, 0.0, 0.8, 1.0);
                let r = r.dilate_pc(-0.03);
                outputs.canvas.put_rect(r, 1.02, v4(0., 0., 0., 1.));
                
                let a = self.knobs.a.curr();
                let d = self.knobs.d.curr();
                let s = 1.0 - self.knobs.s.curr();
                let sustime = 0.7;
                let rel = self.knobs.r.curr();

                let tot = a+d+sustime+rel;

                let tot = tot.max(4.0);
                
                let a_start = r.bl();
                let a_top = v2(r.x + r.w * a/tot, r.y);
                let a_bot = v2(a_top.x, a_start.y);
                
                let d_start = a_top;
                let d_bot = a_bot;
                let d_end = v2(d_start.x + d/tot * r.w, r.y + s*r.h);
                let d_bot2 = v2(d_end.x, d_bot.y);
                
                let sr = Rect::new(d_end.x, d_end.y, (sustime/tot) * r.w, r.h - s * r.h);
                
                let r_start = sr.tr();
                let r_bot = sr.br();
                let r_end = v2(r_bot.x + rel/tot * r.w, r_bot.y);
                
                let c = v4(1., 1., 1., 1.);
                outputs.canvas.put_triangle(a_start, a_top, a_bot, 1.03, c);
                outputs.canvas.put_triangle(d_start, d_bot, d_end, 1.03, c);
                outputs.canvas.put_triangle(d_bot, d_end, d_bot2, 1.03, c);
                outputs.canvas.put_rect(sr, 1.03, c);
                outputs.canvas.put_triangle(r_start, r_end, r_bot, 1.03, c);
            }

            // Oscillator
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
                    self.knobs.base_freq.frame(inputs, outputs, r.grid_child(1, 1, 2, 4));
                    if self.knobs.aout.frame(inputs, outputs, r.grid_child(1, 0, 2, 4)) {
                        let com = AudioCommand::SetVol(db_to_vol(self.knobs.aout.curr()));
                        outputs.sounds.push(com);
                        self.local_mixer.handle_command(com);
                    };
                }
            }

            // Compressor
            let mut max = 0.0f32;
            let mut i = self.rb_head;
            for _ in 0..1024 {
                max = max.max(self.sample_ringbuf[i].abs());
                i = (i + self.sample_ringbuf.len() - 1) % self.sample_ringbuf.len();

            }

            let max_db = vol_to_db(max);


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
                    self.knobs.hard_clip.frame(inputs, outputs, r.grid_child(1, 2, 3, 4));

                    let r = r.grid_child(0, 1, 3, 4).child(0.0, 0.0, 1.0, 3.0);
                    outputs.canvas.put_rect(r, 1.02, v4(0., 0., 0., 1.));
                    let rh = (100.0 + max_db).max(0.0) / 100.0;
                    let hc_db = self.knobs.hard_clip.curr();
                    let hc_line = (100.0 + hc_db).max(0.0) / 100.0;
                    let cu_db = self.knobs.cut.curr();
                    let cu_line = (100.0 + cu_db).max(0.0) / 100.0;
                    let cd_db = self.knobs.cdt.curr();
                    let cd_line = (100.0 + cd_db).max(0.0) / 100.0;

                    let hc_vol = db_to_vol(hc_db);

                    outputs.canvas.put_rect(r.child(0.4, 1.0 - rh, 0.2, rh), 1.03, v4(1., 1., 1., 1.));

                    // hard clip line
                    outputs.canvas.put_rect(r.child(0.0, 1.0 - hc_line, 1.0, 0.01), 1.03, v4(1., 0., 0., 1.));
                    
                    // upward line
                    outputs.canvas.put_rect(r.child(0.0, 1.0 - cu_line, 1.0, 0.01), 1.03, v4(0., 0., 1., 1.));
                    
                    // downward line
                    outputs.canvas.put_rect(r.child(0.0, 1.0 - cd_line, 1.0, 0.01), 1.03, v4(0., 1., 0., 1.));
                }
            }
        }
        // FFT
        // how many times to pump the mixer, 44100/60 lol?
        // ive got t, is it accurate enough
        // definitely pump it better please
        while self.local_mixer.sample_count < (inputs.t * 44100.) as u64 {
            self.sample_ringbuf[self.rb_head] = self.local_mixer.tick() / self.local_mixer.out_vol;
            self.rb_head = (self.rb_head + 1) % FFT_SIZE;
        }
        
        let mut planner = FftPlanner::new();
        let mut buf = [Complex{re: 0.0, im: 0.0}; FFT_SIZE];
        for i in 0..FFT_SIZE {
            let x = self.sample_ringbuf[(self.rb_head + i) % FFT_SIZE] * blackman(i, FFT_SIZE);
            buf[i] = Complex{re: x, im: 0.0};
        }
        let fft = planner.plan_fft_forward(FFT_SIZE);
        fft.process(&mut buf);

        // now display it
        let fft_display_w = FFT_SIZE/8;
        let fft_height = 64;
        let mut tb = TextureBuffer::new(fft_display_w, fft_height);
        for i in 0..fft_display_w {
            let h = -vol_to_db(buf[i].re / FFT_SIZE as f32) / 100.0;
            // let h = -2.0-vol_to_db(buf[i].abs()/buf.len() as f32) / 100.0;

            for j in 0..fft_height {
                if (h * fft_height as f32) < j as f32 {
                    tb.set(i as i32, (fft_height - j - 1) as i32, v4(1., 1., 1., 1.))
                } else {
                    tb.set(i as i32, (fft_height - j - 1) as i32, v4(0., 0., 0., 1.))
                }
            }
        }
        


        {
            // mid
            let r = r.grid_child(0, 1, 1, 3);
            let r = r.dilate_pc(-0.01);
            outputs.set_texture.push((tb, 0));
            outputs.draw_texture.push((r, 0));
            // outputs.canvas.put_rect(r, 1.01, Vec4::new(0.0, 0.8, 0.4, 1.0));
        }

        {
            // bot
            let r = r.grid_child(0, 2, 1, 3);
            let r = r.dilate_pc(-0.01);
            outputs.canvas.put_rect(r, 1.01, v4(0., 0., 0., 1.));

            // draw history notes
            for &(note, start, end) in self.history.iter() {
                if end > inputs.t as f32 - 10.0 {
                    let r = r.grid_child(0, (NUM_NOTES - 1 - note) as i32, 1, NUM_NOTES as i32);
                    let r = r.child(1.0 - (inputs.t as f32 - start) / 10.0, 0.0, (end - start) / 10.0, 1.0);

                    let h = 72.0 * (note / 7) as f32;
                    let s = 1.0 - (note % 7) as f32 / 8.0;
                    let v = 1.0;

                    let c = Vec4::new(h, s, v, 1.0).hsv_to_rgb();
                    outputs.canvas.put_rect(r, 1.2, c);
                }
            }

            // draw held notes
            for &(note, start, _sd) in self.held_keys.values() {
                let r = r.grid_child(0, (NUM_NOTES - 1 - note) as i32, 1, NUM_NOTES as i32);
                let end = inputs.t;
                let r = r.child(1.0 - (inputs.t as f32 - start) / 10.0, 0.0, (end-start) / 10.0, 1.0);

                let h = 72.0 * (note / 7) as f32;
                let s = 1.0 - (note % 7) as f32 / 8.0;
                let v = 1.0;

                let c = Vec4::new(h, s, v, 1.0).hsv_to_rgb();
                outputs.canvas.put_rect(r, 1.2, c);
            }
        }
    }
}

pub fn blackman(n: usize, N: usize) -> f32 {
    let a0 = 0.42;
    let a1 = 0.5;
    let a2 = 0.08;

    a0 - a1 * (2.0*PI*n as f32/N as f32).cos() + a2 * (4.0*PI*n as f32/N as f32).cos()
}