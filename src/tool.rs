use std::mem;

use crate::{
    color::Color,
    element::Element,
    input::{Action, InputState},
    shape_utils,
    world::{ParticleSpawnMode, World},
};

const TOOL_SIZE_MIN: usize = 1;
const TOOL_SIZE_MAX: usize = 500;

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
                let new_w = (*w as isize) + amount;
                let new_h = (*h as isize) + amount;
                *w = new_w.clamp(TOOL_SIZE_MIN as isize, TOOL_SIZE_MAX as isize) as usize;
                *h = new_h.clamp(TOOL_SIZE_MIN as isize, TOOL_SIZE_MAX as isize) as usize;
            }
        }
    }

    pub fn cycle_forward(&mut self) {
        *self = match std::mem::take(self) {
            Self::Oval(w, h, r) => Self::Rect(w, h, r),
            Self::Rect(w, h, r) => Self::Tri(w, h, r),
            Self::Tri(w, h, r) => Self::Oval(w, h, r),
        }
    }

    pub fn cycle_backwards(&mut self) {
        *self = match std::mem::take(self) {
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
                let offset_x = *width as f32 / 2.0;
                let offset_y = *height as f32 / 2.0;

                shape_utils::rect(*width, *height, border_only, |x, y| {
                    f(x as f32 - offset_x, y as f32 - offset_y);
                });
            }
            Shape::Oval(width, height, _rotation) => {
                let rx = *width / 2;
                let ry = *height / 2;

                shape_utils::oval(rx, ry, rx, ry, border_only, |x, y| {
                    f(x as f32 - rx as f32, y as f32 - ry as f32);
                });
            }
            Shape::Tri(width, height, _rotation) => {} // TODO
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

pub enum Kind {
    None,
    Smear(Element, ParticleSpawnMode),
    Erase(),
    Replace(Option<Element>, Option<Element>),
    Grab(),
    Thermalize(f32),
    Color(Option<Color>),
    Picker(),
    ColorPicker(),
}

pub struct Tool {
    pub kind: Kind,
    pub use_action: Action,
    pub last_mouse_pos: Option<(f32, f32)>,
}
impl Tool {
    pub fn new(kind: Kind, use_action: Action) -> Self {
        Self {
            kind,
            use_action,
            last_mouse_pos: None,
        }
    }
    pub fn update(&mut self, shape: &mut Shape, world: &mut World, input: &InputState) {
        if input.is_action_pressed(Action::EnlargeToolShape) {
            shape.scale_by(1);
        }
        if input.is_action_pressed(Action::ShrinkToolShape) {
            shape.scale_by(-1);
        }
        if input.is_action_just_pressed(Action::SwitchToolShape) {
            shape.cycle_forward();
        }
        if input.is_action_pressed(self.use_action) {
            match self.kind {
                Kind::None => {}
                Kind::Smear(element, spawn_mode) => {
                    let current_pos = self.last_mouse_pos.unwrap_or(input.mouse_pos);

                    shape_utils::line(
                        current_pos.0 as usize,
                        current_pos.1 as usize,
                        input.mouse_pos.0 as usize,
                        input.mouse_pos.1 as usize,
                        |start_x, start_y| {
                            shape.for_each_point(false, |offset_x, offset_y| {
                                world.spawn_particle(
                                    element,
                                    start_x as f32 + offset_x,
                                    start_y as f32 + offset_y,
                                    spawn_mode,
                                );
                            });
                        },
                    );
                }
                Kind::Replace(from, to) => todo!(),
                Kind::Grab() => todo!(),
                Kind::Thermalize(by_temp) => todo!(),
                Kind::Color(color) => todo!(),
                Kind::Picker() => todo!(),
                Kind::ColorPicker() => todo!(),
                Kind::Erase() => {
                    let current_pos = self.last_mouse_pos.unwrap_or(input.mouse_pos);

                    shape_utils::line(
                        current_pos.0 as usize,
                        current_pos.1 as usize,
                        input.mouse_pos.0 as usize,
                        input.mouse_pos.1 as usize,
                        |start_x, start_y| {
                            shape.for_each_point(false, |offset_x, offset_y| {
                                let tx = (start_x as f32 + offset_x + 0.5) as isize;
                                let ty = (start_y as f32 + offset_y + 0.5) as isize;
                                world.kill_particle_at(tx, ty);
                            });
                        },
                    );
                },
            }
        }
        self.last_mouse_pos = Some(input.mouse_pos)
    }
}
