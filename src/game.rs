use std::usize;

use crate::{
    element::Element,
    input::{Action, InputState},
    tool::{self, Shape, Tool},
    world::{self, World},
};

pub struct Game {
    pub tool_shape: Shape,
    // TODO: sepertae color tools and element tools into seperate states
    pub tools: (Tool, Tool, Tool),
    pub world: World,
}

impl Game {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            tool_shape: Shape::Oval(15, 15, 0.0),
            tools: (
                Tool::new(
                    tool::Kind::Smear(Element::Sand, world::ParticleSpawnMode::EmptyOnly),
                    Action::UseTool,
                ),
                Tool::new(tool::Kind::Erase(), Action::UseSecondaryTool),
                Tool::new(tool::Kind::Picker(), Action::UseTrinaryTool),
            ),
            world: World::new(width, height),
        }
    }
    pub fn tick(&mut self, input: &InputState) {
        self.world.populate_grid();
        self.world.update_physics();
        self.tools
            .0
            .update(&mut self.tool_shape, &mut self.world, input);
        self.tools
            .1
            .update(&mut self.tool_shape, &mut self.world, input);
        self.tools
            .2
            .update(&mut self.tool_shape, &mut self.world, input);
    }
    pub fn render(&self, buffer: &mut Vec<u32>, input: &InputState) {
        // May be swapped out later, I want glowing particles and UI so the rendering methods will likely change...

        for pixel in buffer.iter_mut() {
            *pixel = self.world.background_color.value;
        }

        self.render_particles(buffer);
        self.tool_shape
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
