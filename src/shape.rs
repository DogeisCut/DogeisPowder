use crate::{color::Color, raster};

const SIZE_MIN: usize = 1;
const SIZE_MAX: usize = 500;

#[derive(Clone, Copy)]
pub enum Shape {
    Oval(usize, usize, f32),
    Rect(usize, usize, f32),
    Tri(usize, usize, f32),
}

impl Default for Shape {
    fn default() -> Self {
        Self::Oval(0, 0, 0.0)
    }
}

impl Shape {
    pub fn scale_by(&mut self, amount: isize) {
        match self {
            Self::Oval(w, h, _) | Self::Rect(w, h, _) | Self::Tri(w, h, _) => {
                let new_w = (*w as isize) + amount * 2;
                let new_h = (*h as isize) + amount * 2;

                *w = (new_w.max(SIZE_MIN as isize).min(SIZE_MAX as isize) | 1) as usize;
                *h = (new_h.max(SIZE_MIN as isize).min(SIZE_MAX as isize) | 1) as usize;
            }
        }
    }

    pub fn cycle_forward(&mut self) {
        *self = match *self {
            Self::Oval(w, h, r) => Self::Rect(w, h, r),
            Self::Rect(w, h, r) => Self::Tri(w, h, r),
            Self::Tri(w, h, r) => Self::Oval(w, h, r),
        }
    }

    pub fn cycle_backwards(&mut self) {
        *self = match *self {
            Self::Oval(w, h, r) => Self::Tri(w, h, r),
            Self::Tri(w, h, r) => Self::Rect(w, h, r),
            Self::Rect(w, h, r) => Self::Oval(w, h, r),
        }
    }

    pub fn for_each_point<F>(&self, border_only: bool, mut f: F)
    where
        F: FnMut(f32, f32),
    {
        match self {
            Shape::Rect(width, height, _rotation) => {
                let rx = *width / 2;
                let ry = *height / 2;

                raster::rect(*width, *height, border_only, |x, y| {
                    f(x as f32 - rx as f32, y as f32 - ry as f32);
                });
            }
            Shape::Oval(width, height, _rotation) => {
                let rx = *width / 2;
                let ry = *height / 2;

                raster::oval(rx, ry, rx, ry, border_only, |x, y| {
                    f(x as f32 - rx as f32, y as f32 - ry as f32);
                });
            }
            Shape::Tri(width, height, _rotation) => {
                let rx = *width / 2;
                let ry = *height / 2;

                let p0 = (*width / 2, 0);
                let p1 = (0, *height - 1);
                let p2 = (*width - 1, *height - 1);

                raster::triangle(p0, p1, p2, border_only, |x, y| {
                    f(x as f32 - rx as f32, y as f32 - ry as f32);
                });
            }
        }
    }

    pub fn render_preview(
        &self,
        screen_buffer: &mut [u32],
        screen_width: usize,
        screen_height: usize,
        mouse_pos: (f32, f32),
    ) {
        self.for_each_point(true, |offset_x, offset_y| {
            let render_x = (mouse_pos.0 + 0.5 + offset_x) as isize;
            let render_y = (mouse_pos.1 + 0.5 + offset_y) as isize;

            if render_x >= 0
                && render_x < screen_width as isize
                && render_y >= 0
                && render_y < screen_height as isize
            {
                let idx = render_x as usize + (render_y as usize * screen_width);
                screen_buffer[idx] = Color {
                    value: screen_buffer[idx],
                }
                .invert(true)
                .value;
            }
        });
    }
}
