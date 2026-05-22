use crate::{
    color::Color,
    element::Element,
    shape_utils::{oval, rect},
    tool::{self, Tool},
    world::{self, World},
};

pub struct Game {
    pub tool: Tool,
    pub world: World,
    pub mouse_x: usize,
    pub mouse_y: usize,
}

impl Game {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            tool: Tool::new(
                tool::Shape::Oval(15, 15, 0.0),
                tool::Kind::Smear(Some(Element::Sand)),
            ),
            world: World::new(width, height),
            mouse_x: 0,
            mouse_y: 0,
        }
    }
    pub fn tick(&mut self) {
        self.world.populate_grid();
        self.world.update_physics();
    }
    pub fn render(&self, buffer: &mut Vec<u32>) {
        // May be swapped out later, I want glowing particles and UI so the rendering methods will likely change...

        for pixel in buffer.iter_mut() {
            *pixel = self.world.background_color.value;
        }

        self.render_particles(buffer);
        self.render_tool(buffer);
    }
    fn render_particles(&self, buffer: &mut Vec<u32>) {
        for y in 0..self.world.particles_grid.height {
            for x in 0..self.world.particles_grid.width {
                if let world::CellState::Occupied(index) =
                    self.world.particles_grid.get(x as isize, y as isize)
                {
                    let particle = self.world.particles_flat[index];
                    let color = particle.element.color().overlay(particle.decoration);

                    buffer[x + (y * self.world.particles_grid.width)] =
                        self.world.background_color.overlay(color).value;
                }
            }
        }
    }
    fn render_tool(&self, buffer: &mut Vec<u32>) {
        match self.tool.shape {
            tool::Shape::Oval(width, height, rotation) => {
                oval(self.mouse_x, self.mouse_y, width, height, true, |x, y| {
                    if (x < self.world.particles_grid.width)
                        && (y < self.world.particles_grid.height)
                    {
                        let index = x + (y * self.world.particles_grid.width);
                        buffer[index] = Color {
                            value: buffer[index],
                        }
                        .invert(true)
                        .value;
                    }
                });
            }
            tool::Shape::Rect(width, height, rotation) => {
                let origin_x = self.mouse_x - ((width as f32) / 2.0).round() as usize;
                let origin_y: usize = self.mouse_y - ((height as f32) / 2.0).round() as usize;
                rect(width, height, true, |x, y| {
                    let offset_x = x + origin_x;
                    let offset_y = y + origin_y;
                    if (offset_x < self.world.particles_grid.width)
                        && (offset_y < self.world.particles_grid.height)
                    {
                        let index = (offset_x) + (offset_y * self.world.particles_grid.width);
                        buffer[index] = Color {
                            value: buffer[index],
                        }
                        .invert(true)
                        .value;
                    }
                });
            }
            tool::Shape::Tri(width, height, rotation) => {}
        }
    }
}
