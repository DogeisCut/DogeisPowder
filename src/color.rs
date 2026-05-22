#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub value: u32,
}
impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: Option<u8>) -> Self {
        let alpha = a.unwrap_or(255) as u32;

        Self {
            value: (alpha << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32),
        }
    }

    pub fn a(&self) -> u8 {
        ((self.value >> 24) & 0xFF) as u8
    }

    pub fn r(&self) -> u8 {
        ((self.value >> 16) & 0xFF) as u8
    }

    pub fn g(&self) -> u8 {
        ((self.value >> 8) & 0xFF) as u8
    }

    pub fn b(&self) -> u8 {
        (self.value & 0xFF) as u8
    }

    pub fn set_a(&mut self, a: u8) {
        self.value = (self.value & 0x00FFFFFF) | ((a as u32) << 24);
    }

    pub fn set_r(&mut self, r: u8) {
        self.value = (self.value & 0xFF00FFFF) | ((r as u32) << 16);
    }

    pub fn set_g(&mut self, g: u8) {
        self.value = (self.value & 0xFFFF00FF) | ((g as u32) << 8);
    }

    pub fn set_b(&mut self, b: u8) {
        self.value = (self.value & 0xFFFFFF00) | (b as u32);
    }

    pub fn mix(&self, other: Color, factor: f32) -> Self {
        let f = factor.clamp(0.0, 1.0);
        let inv_f = 1.0 - f;

        // Mix each channel linearly (LERP)
        let r = ((self.r() as f32 * inv_f) + (other.r() as f32 * f)) as u8;
        let g = ((self.g() as f32 * inv_f) + (other.g() as f32 * f)) as u8;
        let b = ((self.b() as f32 * inv_f) + (other.b() as f32 * f)) as u8;
        let a = ((self.a() as f32 * inv_f) + (other.a() as f32 * f)) as u8;

        Color::new(r, g, b, Some(a))
    }

    pub fn overlay(&self, other: Color) -> Self {
        let r_bg = self.r() as f32 / 255.0;
        let g_bg = self.g() as f32 / 255.0;
        let b_bg = self.b() as f32 / 255.0;
        let a_bg = self.a() as f32 / 255.0;

        let r_fg = other.r() as f32 / 255.0;
        let g_fg = other.g() as f32 / 255.0;
        let b_fg = other.b() as f32 / 255.0;
        let a_fg = other.a() as f32 / 255.0;

        let a_out = a_fg + a_bg * (1.0 - a_fg);

        if a_out == 0.0 {
            return Color::new(0, 0, 0, Some(0));
        }

        let r_out = (r_fg * a_fg + r_bg * a_bg * (1.0 - a_fg)) / a_out;
        let g_out = (g_fg * a_fg + g_bg * a_bg * (1.0 - a_fg)) / a_out;
        let b_out = (b_fg * a_fg + b_bg * a_bg * (1.0 - a_fg)) / a_out;

        Color::new(
            (r_out * 255.0) as u8,
            (g_out * 255.0) as u8,
            (b_out * 255.0) as u8,
            Some((a_out * 255.0) as u8),
        )
    }
}
