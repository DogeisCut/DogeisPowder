use std::usize;

use crate::{
    element::Element,
    input::InputState,
    tool::{self, Tool},
    world::{self, World},
};

pub struct Game {
    pub tool: Tool,
    pub world: World,
}

impl Game {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            tool: Tool::new(
                tool::Shape::Rect(15, 15, 0.0),
                tool::Kind::Smear(Some(Element::Sand), false),
            ),
            world: World::new(width, height),
        }
    }
    pub fn tick(&mut self, input: &InputState) {
        self.world.populate_grid();
        self.world.update_physics();
        self.tool.update(&mut self.world, input);
    }
    pub fn render(&self, buffer: &mut Vec<u32>, input: &InputState) {
        // May be swapped out later, I want glowing particles and UI so the rendering methods will likely change...

        for pixel in buffer.iter_mut() {
            *pixel = self.world.background_color.value;
        }

        self.render_particles(buffer);
        self.tool
            .render_preview(buffer, self.width(), self.height(), input.mouse_pos);
    }
    fn render_particles(&self, buffer: &mut Vec<u32>) {
        for y in 0..self.height() {
            for x in 0..self.width() {
                if let world::CellState::Occupied(index) =
                    self.world.particles_grid.get(x as isize, y as isize)
                {
                    let particle = self.world.particles_flat[index];
                    let color = particle.element.color().overlay(particle.decoration);

                    buffer[x + (y * self.width())] =
                        self.world.background_color.overlay(color).value;
                }
            }
        }
    }
    fn width(&self) -> usize {
        self.world.particles_grid.width
    }
    fn height(&self) -> usize {
        self.world.particles_grid.height
    }
}
