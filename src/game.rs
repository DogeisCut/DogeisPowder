use crate::{
    color::Color,
    world::{self, World},
};

pub struct Game {
    world: World,
}

impl Game {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            world: World::new(width, height),
        }
    }
    pub fn tick(&mut self) {
        self.world.populate_grid();
        self.world.update_physics();
    }
    pub fn render(&self, buffer: &mut Vec<u32>) {
        // May be swapped out later, I want glowing particles and UI so the rendering methods will likely change...

        const BG_COLOR: Color = Color { value: 0xFF1A1A1A };

        for pixel in buffer.iter_mut() {
            *pixel = BG_COLOR.value
        }

        for y in 0..self.world.particles_grid.height {
            for x in 0..self.world.particles_grid.width {
                if let world::CellState::Occupied(index) =
                    self.world.particles_grid.get(x as isize, y as isize)
                {
                    let particle = self.world.particles_flat[index];
                    let color = particle.element.color().overlay(particle.decoration);

                    buffer[x + (y * self.world.particles_grid.width)] =
                        BG_COLOR.overlay(color).value;
                }
            }
        }
    }
}
