use minifb::KeyRepeat::No;

use crate::{
    color::Color,
    element::{Element, StateOfMatter},
    particle::{self, Particle}, vector,
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Edge {
    Stop,
    Loop,
    Destroy,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Gravity {
    None,
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
            gravity: Gravity::Linear(0.0, 0.3),
            background_color: Color { value: 0xFF1A1A1A },
        }
    }

    pub fn populate_grid(&mut self) {
        self.particles_grid.empty();

        for index in 0..self.particles_flat.len() {
            let particle = self.particles_flat[index];

            if particle.is_dead {
                continue;
            }

            let grid_x = particle.get_grid_x();
            let grid_y = particle.get_grid_y();

            self.particles_grid.set(grid_x, grid_y, Some(index));
        }
    }

    pub fn try_move_particle(&mut self, index: usize, target_x: f32, target_y: f32) -> bool {
        let target_grid_x = f32_to_grid(target_x) as usize;
        let target_grid_y = f32_to_grid(target_y) as usize;

        match self
            .particles_grid
            .get(target_grid_x as isize, target_grid_y as isize)
        {
            CellState::Empty => {
                let mut particle = self.particles_flat[index];

                let old_grid_x = particle.get_grid_x();
                let old_grid_y = particle.get_grid_y();

                particle.x = target_x;
                particle.y = target_y;

                self.particles_flat[index] = particle;

                self.particles_grid.set(old_grid_x, old_grid_y, None);
                self.particles_grid
                    .set(target_grid_x, target_grid_y, Some(index));

                particle
                    .element
                    .on_physics_move_attempt(index, particle, self, target_x, target_y);

                true
            }
            CellState::Occupied(other_index) => {

                if index == other_index {
                    let mut particle = self.particles_flat[index];
                    particle.x = target_x;
                    particle.y = target_y;
                    self.particles_flat[index] = particle;
                    return true;
                }

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

                self.particles_grid
                    .set(target_grid_x, target_grid_y, Some(index));
                self.particles_grid
                    .set(old_grid_x, old_grid_y, Some(other_index));

                particle
                    .element
                    .on_physics_move_attempt(index, particle, self, target_x, target_y);

                true
            }
            CellState::OutOfBounds => {
                // TODO: handle different edge types

                let particle = self.particles_flat[index];
                particle
                    .element
                    .on_physics_move_attempt(index, particle, self, target_x, target_y);

                false
            }
        }
    }

    pub fn try_move_particle_by(&mut self, index: usize, by_x: f32, by_y: f32) -> bool {
        let particle = self.particles_flat[index];
        self.try_move_particle(index, particle.x + by_x, particle.y + by_y)
    }

    pub fn move_with_velocity(&mut self, index: usize) {
        let p = self.particles_flat[index];
        
        let mag = crate::vector::magnitude_of((p.vx, p.vy));
        let steps = mag.ceil() as i32;

        if steps == 0 {
            return;
        }

        let dx = p.vx / steps as f32;
        let dy = p.vy / steps as f32;
        
        let friction = p.element.friction();
        let restitution = p.element.restitution();

        let mut hit_x = false;
        let mut hit_y = false;

        for _ in 0..steps {
            if !self.try_move_particle_by(index, dx, dy) {
                let moved_x = self.try_move_particle_by(index, dx, 0.0);
                let moved_y = self.try_move_particle_by(index, 0.0, dy);

                if !moved_x { hit_x = true; }
                if !moved_y { hit_y = true; }
                
                if !hit_x && !hit_y {
                    hit_x = true;
                    hit_y = true;
                }
                
                if hit_x || hit_y {
                    break;
                }
            }
        }

        let mut updated_p = self.particles_flat[index];

        if hit_x {
            updated_p.vx *= -restitution;
            updated_p.vy *= 1.0 - friction;
        }
        if hit_y {
            updated_p.vy *= -restitution;
            updated_p.vx *= 1.0 - friction;
        }
        
        if updated_p.vx.abs() < 0.01 { updated_p.vx = 0.0; }
        if updated_p.vy.abs() < 0.01 { updated_p.vy = 0.0; }

        self.particles_flat[index] = updated_p;
    }

    pub fn kill_particle(&mut self, index: usize) {
        if self.particles_flat[index].is_dead {
            return;
        }

        self.particles_flat[index].is_dead = true;

        let grid_x = self.particles_flat[index].get_grid_x();
        let grid_y = self.particles_flat[index].get_grid_y();
        self.particles_grid.set(grid_x, grid_y, None);
    }

    pub fn kill_particle_at(&mut self, grid_x: isize, grid_y: isize) {
        if let CellState::Occupied(index) = self.particles_grid.get(grid_x, grid_y) {
            self.kill_particle(index);
        }
    }

    pub fn spawn_particle(
        &mut self,
        element: Element,
        x: f32,
        y: f32,
        particle_spawn_mode: ParticleSpawnMode,
    ) {
        let target_grid_x = f32_to_grid(x) as usize;
        let target_grid_y = f32_to_grid(y) as usize;
        match particle_spawn_mode {
            ParticleSpawnMode::Overlap => {
                self.particles_flat
                    .push(Particle::new(element, x, y, 0.0, 0.0));
                self.particles_grid.set(
                    target_grid_x,
                    target_grid_y,
                    Some(self.particles_flat.len() - 1),
                );
            }
            ParticleSpawnMode::Override => {
                self.kill_particle_at(target_grid_x as isize, target_grid_y as isize);
                self.particles_flat
                    .push(Particle::new(element, x, y, 0.0, 0.0));
                self.particles_grid.set(
                    target_grid_x,
                    target_grid_y,
                    Some(self.particles_flat.len() - 1),
                );
            }
            ParticleSpawnMode::EmptyOnly => {
                if self
                    .particles_grid
                    .get(target_grid_x as isize, target_grid_y as isize)
                    == CellState::Empty
                {
                    self.particles_flat
                        .push(Particle::new(element, x, y, 0.0, 0.0));
                    self.particles_grid.set(
                        target_grid_x,
                        target_grid_y,
                        Some(self.particles_flat.len() - 1),
                    );
                }
            }
        }
    }

    pub fn update_physics(&mut self) {
        for index in 0..self.particles_flat.len() {
            let particle = self.particles_flat[index];
            if particle.is_dead {
                continue;
            }
            particle.element.pre_tick(index, particle, self);
        }

        let mut updated = vec![false; self.particles_flat.len()];

        for y in (0..self.particles_grid.height).rev() {
            let flip_x = rand::random::<bool>();

            for i in 0..self.particles_grid.width {
                let x = if flip_x {
                    self.particles_grid.width - 1 - i
                } else {
                    i
                };

                if let CellState::Occupied(index) = self.particles_grid.get(x as isize, y as isize)
                {
                    if updated[index] {
                        continue;
                    }
                    updated[index] = true;

                    let mut particle = self.particles_flat[index];

                    if let Some(life) = self.particles_flat[index].life.as_mut() {
                        *life = life.saturating_sub(1);
                        if *life == 0 {
                            self.kill_particle(index);
                        }
                    }

                    if self.particles_flat[index].is_dead {
                        continue;
                    }

                    // In The Powder Toy (TPT), it looks like settling logic for elements take into account the direction of velocity instead of always assuming downwards motion.
                    // - And yes, this is not based on gravity, i checked.
                    // It also looks like gravity is built into the first part of the settling step
                    // Also decimal gravity needs to be accounted for.
                    // It also seems particles can bounce a bit (restitution property?)
                    // There is no velocity transfer between particles in TPT. A fast moving particle hitting another particle will just stop.
                    // TPT properly checks particles ahead at high speeds, and will stop at the point of impact
                    // Either elements in TPT have friction, or particles stop dead in their tracks when trying to go through another one, regardless of direction.
                    // Upon further investigation, it appears to be the latter.
                    // Implemting all of this may require creating new particle move functions, ones that move in steps and stop at impact, as well as apply restitution

                    let gravity = self.gravity;
                    let (gx, gy) = match gravity {
                        Gravity::None => (0.0_f32, 0.0_f32),
                        Gravity::Linear(gx, gy) => (gx, gy),
                        Gravity::Radial(cx, cy, strength) => {
                            let p = self.particles_flat[index];
                            let norm = crate::vector::normalized((cx - p.x, cy - p.y));
                            (norm.0 * strength, norm.1 * strength)
                        }
                    };

                    match particle.element.kind() {
                        StateOfMatter::Powder | StateOfMatter::Liquid | StateOfMatter::Gas | StateOfMatter::Energy => {
                            self.particles_flat[index].vx += gx;
                            self.particles_flat[index].vy += gy;
                        },
                        StateOfMatter::Solid => {},
                    }

                    self.move_with_velocity(index);

                    let particle_after = self.particles_flat[index];

                    let bias: f32 = if rand::random::<bool>() { 1.0 } else { -1.0 };
                    // let settle_x = if particle_after.vx.abs() > 0.1 { particle_after.vx.signum() } else { bias };
                    // let settle_y = if particle_after.vy.abs() > 0.1 { particle_after.vy.signum() } else { gy.signum() };

                    // something is wrong but im too stupid to figure it out, most notable in radial gravity
                    // also ideally we would use velocity not the direction of gravity for calculating this
                    let vel_angle = vector::angle_of((gx, gy));
                    let (settle_x, settle_y) = vector::rotated((1.0,bias), vel_angle);

                    match particle_after.element.kind() {
                        StateOfMatter::Powder => {
                            if settle_y != 0.0 && !self.try_move_particle_by(index, 0.0, settle_y) {
                                if !self.try_move_particle_by(index, settle_x, settle_y) {
                                    self.try_move_particle_by(index, -settle_x, settle_y);
                                }
                            }
                        }
                        StateOfMatter::Liquid => {
                            // Liquid seems to take stupidly long to settle for some reason
                            // TODO: look into how powder toy combats this. HINT: it isnt by making water move 2+ units instead of 1
                            if settle_y != 0.0 && !self.try_move_particle_by(index, 0.0, settle_y) {
                                if !self.try_move_particle_by(index, settle_x, settle_y) {
                                    if !self.try_move_particle_by(index, -settle_x, settle_y) {
                                        if !self.try_move_particle_by(index, settle_x, 0.0) {
                                            self.try_move_particle_by(index, -settle_x, 0.0);
                                        }
                                    }
                                }
                            }
                        }
                        StateOfMatter::Gas => {
                            // Need to rewrite this to just pick a random direction out of 8 directions.
                            let gas_y = if settle_y != 0.0 { -settle_y } else { -1.0 };
                            if !self.try_move_particle_by(index, settle_x, gas_y) {
                                if !self.try_move_particle_by(index, 0.0, gas_y) {
                                    if !self.try_move_particle_by(index, -settle_x, gas_y) {
                                        if !self.try_move_particle_by(index, settle_x, 0.0) {
                                            self.try_move_particle_by(index, -settle_x, 0.0);
                                        }
                                    }
                                }
                            }
                        }
                        StateOfMatter::Solid | StateOfMatter::Energy => { 
                            // Nada
                        }
                    }

                    let particle_after = self.particles_flat[index];
                    particle_after
                        .element
                        .physics_tick(index, particle_after, self);
                }
            }
        }

        for index in 0..self.particles_flat.len() {
            let particle = self.particles_flat[index];
            if particle.is_dead {
                continue;
            }
            particle.element.post_tick(index, particle, self);
        }

        self.cleanup_dead_particles();
    }

    pub fn cleanup_dead_particles(&mut self) {
        let mut i = self.particles_flat.len();
        while i > 0 {
            i -= 1;

            if self.particles_flat[i].is_dead {
                self.particles_flat.swap_remove(i);

                if i < self.particles_flat.len() {
                    let moved_particle = self.particles_flat[i];
                    self.particles_grid.set(
                        moved_particle.get_grid_x(),
                        moved_particle.get_grid_y(),
                        Some(i),
                    )
                }
            }
        }
    }
}

pub fn f32_to_grid(float: f32) -> isize {
    float.floor() as isize
}
