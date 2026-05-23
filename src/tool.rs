use crate::{
    color::Color,
    element::Element,
    input::{Action, InputState},
    raster,
    shape::Shape,
    world::{ParticleSpawnMode, World},
};

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
        if input.is_action_pressed(self.use_action) {
            match self.kind {
                Kind::None => {}
                Kind::Smear(element, spawn_mode) => {
                    let current_pos = self.last_mouse_pos.unwrap_or(input.mouse_pos);

                    raster::line(
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

                    raster::line(
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
                }
            }
        }
        self.last_mouse_pos = Some(input.mouse_pos)
    }
}
