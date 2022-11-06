use crate::kmath::*;
use crate::kapp::*;

pub struct FloatSlider {
    pub t: f32, // 1.0 top 0.0 bot
    pub min: f32,
    pub max: f32,
    pub held: bool,
    pub label: String,
}

impl FloatSlider {
    pub fn new(default: f32, min: f32, max: f32, label: String) -> FloatSlider {
        FloatSlider {
            t: (default - min) / (max - min),
            min, 
            max,
            held: false,
            label
        }
    }

    pub fn set_val(&mut self, val: f32) {
        self.t = (val - self.min) / (self.max - self.min);
    }

    pub fn curr(&self) -> f32 {
        lerp(self.min, self.max, self.t)
    }

    pub fn frame(&mut self, inputs: &FrameInputState, outputs: &mut FrameOutputs, r: Rect) -> bool {
        let mut any_change = false;
        if inputs.lmb == KeyStatus::JustPressed && r.contains(inputs.mouse_pos) {
            self.held = true;
        }


        // fine
        if r.contains(inputs.mouse_pos) {
            if inputs.scroll_delta > 0.0 {
                self.t = (self.t + 0.0005).min(1.0);
                any_change = true;
            }
            if inputs.scroll_delta < 0.0 {
                self.t = (self.t - 0.0005).max(0.0);
                any_change = true;
            }
        }

        // text stuff
        let a = 12.0 / 14.0;

        // colours
        let line = Vec4::grey(0.8);
        let fg = Vec4::grey(0.6);
        let bg = Vec4::grey(0.2);
        let text = Vec4::grey(1.0);

        outputs.canvas.put_rect(r, 1.0, line);
        let r = r.dilate_pc(-0.02);
        outputs.canvas.put_rect(r, 1.1, bg);

        let top_h = 0.1;
        let mid_h = 0.85;
        let bot_h = 0.05;
        let slider_h = r.h * bot_h;

        // top
        {
            let r = r.child(0.0, 0.0, 1.0, top_h);
            // outputs.canvas.put_rect(r, 5.0, Vec4::new(1.0, 0.0, 0.0, 1.0));
            let (top, bot) = r.split_ud(0.5);
            let top = top.dilate_pc(-0.05);
            let bot = bot.dilate_pc(-0.05);
            outputs.glyphs.pushc(top, &self.label, a, text, 2.0);
            let s = &format!("{:.3}", self.max);
            outputs.glyphs.pushl(bot, s, a, fg, 2.0);
        }
        
        // mid
        {
            let r = r.child(0.0, top_h, 1.0, mid_h);
            let r = r.dilate(-0.01);
            // outputs.canvas.put_rect(r, 5.0, Vec4::new(0.0, 1.0, 0.0, 1.0));
            let midline = r.child(0.49, 0.0, 0.05, 1.0);
            outputs.canvas.put_rect(midline, 1.2, fg);

            if self.held {
                self.t = remap(inputs.mouse_pos.y, r.bot(), r.top(), 0.0, 1.0).max(0.0).min(1.0);
            }
            
            // slider
            {
                // let r = r.child(1.0 - self.t)
                let r = Rect::new_centered(r.x + r.w/2.0, r.y + (1.0 - self.t)*r.h, r.w, slider_h);
                outputs.canvas.put_rect(r, 1.6, fg);
                let r = r.dilate_pc(-0.05);
                let s = &format!("{:.3}", self.curr()); // if the rect draws properly (it does ish)
                outputs.glyphs.pushc(r, s, a, text, 2.0);

            }
        }
        
        // bot
        {
            let r = r.child(0.0, top_h + mid_h, 1.0, bot_h);
            // outputs.canvas.put_rect(r, 5.0, Vec4::new(0.0, 0.0, 1.0, 1.0));
            let r = r.dilate_pc(-0.05);
            let s = &format!("{:.3}", self.min);
            outputs.glyphs.pushl(r, s, a, fg, 2.0);
        }

        if self.held && inputs.lmb == KeyStatus::JustReleased {
            self.held = false;
            return true;
        }
        if self.held && inputs.lmb == KeyStatus::Pressed && inputs.mouse_delta != Vec2::new(0.0, 0.0) {
            return true;
        }
        return any_change;
    }
}
