use minifb::KeyRepeat::No;

use crate::{
    color::Color,
    element::{Element, StateOfMatter},
    particle::Particle,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CellState {
    Empty,
    Occupied(usize),
    OutOfBounds,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParticleSpawnMode {
    Overlap,
    Override,
    EmptyOnly,
}

pub struct IndexGrid {
    pub width: usize,
    pub height: usize,
    cells: Vec<Option<usize>>,
}
impl IndexGrid {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![None; width * height],
        }
    }

    pub fn get(&self, x: isize, y: isize) -> CellState {
        if x < 0 || y < 0 || x >= self.width as isize || y >= self.height as isize {
            return CellState::OutOfBounds;
        }

        match self.cells[x as usize + (y as usize * self.width)] {
            Some(index) => CellState::Occupied(index),
            None => CellState::Empty,
        }
    }

    pub fn set(&mut self, x: usize, y: usize, value: Option<usize>) {
        self.cells[x + (y * self.width)] = value;
    }

    pub fn empty(&mut self) {
        self.cells = vec![None; self.width * self.height];
    }
}

pub enum Edge {
    Stop,
    Loop,
    Destroy,
}

pub enum Gravity {
    Linear(f32, f32),
    Radial(f32, f32, f32),
}

pub struct World {
    pub particles_flat: Vec<Particle>,
    pub particles_grid: IndexGrid,
    pub edge: Edge,
    pub gravity: Gravity,
    pub background_color: Color,
}
impl World {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            particles_flat: Vec::new(),
            particles_grid: IndexGrid::new(width, height),
            edge: Edge::Stop,
            gravity: Gravity::Linear(0.0, 1.0),
            background_color: Color { value: 0xFF1A1A1A },
        }
    }

    pub fn populate_grid(&mut self) {
        self.particles_grid.empty();

        for index in 0..self.particles_flat.len() {
            let particle = self.particles_flat[index];

            let grid_x = particle.get_grid_x();
            let grid_y = particle.get_grid_y();

            self.particles_grid.set(grid_x, grid_y, Some(index));
        }
    }

    pub fn try_move_particle(&mut self, index: usize, target_x: f32, target_y: f32) -> bool {
        let target_grid_x = (target_x + 0.5) as isize;
        let target_grid_y = (target_y + 0.5) as isize;

        match self.particles_grid.get(target_grid_x, target_grid_y) {
            CellState::Empty => {
                let mut particle = self.particles_flat[index];

                let old_grid_x = particle.get_grid_x();
                let old_grid_y = particle.get_grid_y();

                particle.x = target_x;
                particle.y = target_y;

                self.particles_flat[index] = particle;

                self.particles_grid.set(old_grid_x, old_grid_y, None);
                self.particles_grid.set(
                    target_grid_x as usize,
                    target_grid_y as usize,
                    Some(index),
                );

                true
            }
            CellState::Occupied(other_index) => {
                let mut particle = self.particles_flat[index];
                let mut other_particle = self.particles_flat[other_index];

                if particle.element.density() <= other_particle.element.density() {
                    return false;
                }

                let old_grid_x = particle.get_grid_x();
                let old_grid_y = particle.get_grid_y();

                let old_x = particle.x;
                let old_y = particle.y;

                particle.x = target_x;
                particle.y = target_y;

                other_particle.x = old_x;
                other_particle.y = old_y;

                self.particles_flat[index] = particle;
                self.particles_flat[other_index] = other_particle;

                self.particles_grid.set(
                    target_grid_x as usize,
                    target_grid_y as usize,
                    Some(index),
                );
                self.particles_grid
                    .set(old_grid_x, old_grid_y, Some(other_index));

                true
            }
            CellState::OutOfBounds => false,
        }
    }

    pub fn try_move_particle_by(&mut self, index: usize, by_x: f32, by_y: f32) -> bool {
        let particle = self.particles_flat[index];
        self.try_move_particle(index, particle.x + by_x, particle.y + by_y)
    }

    pub fn spawn_particle(
        &mut self,
        element: Element,
        x: f32,
        y: f32,
        particle_spawn_mode: ParticleSpawnMode,
    ) {
        let target_grid_x = (x + 0.5) as isize;
        let target_grid_y = (y + 0.5) as isize;
        match particle_spawn_mode {
            ParticleSpawnMode::Overlap => {
                self.particles_flat
                    .push(Particle::new(element, x, y, 0.0, 0.0));
                self.particles_grid.set(
                    target_grid_x as usize,
                    target_grid_y as usize,
                    Some(self.particles_flat.len() - 1),
                );
            }
            ParticleSpawnMode::Override => {
                self.particles_flat
                    .remove(x as usize + (y as usize * self.particles_grid.width));

                self.particles_flat
                    .push(Particle::new(element, x, y, 0.0, 0.0));
                self.particles_grid.set(
                    target_grid_x as usize,
                    target_grid_y as usize,
                    Some(self.particles_flat.len() - 1),
                );
            }
            ParticleSpawnMode::EmptyOnly => {
                if self.particles_grid.get(target_grid_x, target_grid_y) == CellState::Empty {
                    self.particles_flat
                        .push(Particle::new(element, x, y, 0.0, 0.0));
                    self.particles_grid.set(
                        target_grid_x as usize,
                        target_grid_y as usize,
                        Some(self.particles_flat.len() - 1),
                    );
                }
            }
        }
    }

    pub fn update_physics(&mut self) {
        for index in 0..self.particles_flat.len() {
            let particle = self.particles_flat[index];

            match particle.element.kind() {
                StateOfMatter::Powder => {
                    let bias: f32 = if rand::random::<bool>() { 1.0 } else { -1.0 };

                    if !self.try_move_particle_by(index, 0.0, 1.0) {
                        if !self.try_move_particle_by(index, bias, 1.0) {
                            self.try_move_particle_by(index, -bias, 1.0);
                        };
                    };
                }
                StateOfMatter::Liquid => {
                    let bias: f32 = if rand::random::<bool>() { 1.0 } else { -1.0 };

                    if !self.try_move_particle_by(index, 0.0, 1.0) {
                        if !self.try_move_particle_by(index, bias, 1.0) {
                            if !self.try_move_particle_by(index, -bias, 1.0) {
                                if !self.try_move_particle_by(index, bias, 0.0) {
                                    self.try_move_particle_by(index, -bias, 0.0);
                                };
                            };
                        };
                    };
                }
                StateOfMatter::Gas => {
                    let bias: f32 = if rand::random::<bool>() { 1.0 } else { -1.0 };

                    if !self.try_move_particle_by(index, bias, -1.0) {
                        if !self.try_move_particle_by(index, 0.0, -1.0) {
                            if !self.try_move_particle_by(index, -bias, -1.0) {
                                if !self.try_move_particle_by(index, bias, 0.0) {
                                    self.try_move_particle_by(index, -bias, 0.0);
                                };
                            };
                        };
                    };
                }
                StateOfMatter::Solid => {
                    // Solids sit still, they don't even move with velocity.
                }
                StateOfMatter::Energy => {
                    // Normally energy particles would move with velocity but we don't have that implemented.
                }
            }
        }
    }
}
