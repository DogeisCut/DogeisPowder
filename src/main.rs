struct Color {
    r: u8,
    g: u8,
    b: u8
} impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self {
            r,
            g,
            b,
        }
    }
    pub fn new_from_brightness(brightness: u8) -> Self {
        Self {
            r: brightness,
            g: brightness,
            b: brightness,
        }
    }
    pub fn new_from_hex_code() -> Self {
        todo!()
    }

    pub fn to_u32(&self) -> u32 {
        let r = self.r as u32;
        let g = self.g as u32;
        let b = self.b as u32;
        
        (r << 16) | (g << 8) | b
    }
}

enum ElementKind {
    Powder,
    Liquid,
    Gas,
    Solid,
    Energy
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Element {
    Sand,
    Water,
    Wood,
} impl Element {
    pub fn density(&self) -> f32 {
        match self {
            // These numbers are based on real life g/cm3
            Element::Sand => 1.5,
            Element::Water => 1.0,
            Element::Wood => 0.75,
        }
    }
    pub fn kind(&self) -> ElementKind {
        match self {
            Element::Sand => ElementKind::Powder,
            Element::Water => ElementKind::Liquid,
            Element::Wood => ElementKind::Solid,
        }
    }
    pub fn color(&self) -> Color {
        match self {
            Element::Sand => Color::new(255, 229, 125),
            Element::Water => Color::new(36, 116, 255),
            Element::Wood => Color::new(89, 75, 51),
        }
    }
    pub fn color_varries(&self) -> bool {
        match self {
            Element::Sand => true,
            Element::Water => false,
            Element::Wood => false,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Particle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    element_type: Element,
} impl Particle {
    pub fn get_grid_x(&self) -> usize {
        (self.x + 0.5) as usize
    }
    pub fn get_grid_y(&self) -> usize {
        (self.y + 0.5) as usize
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum CellState {
    Empty,
    Occupied(usize),
    OutOfBounds
}

struct IndexGrid {
    width: usize,
    height: usize,
    cells: Vec<Option<usize>>,
} impl IndexGrid {
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

struct World {
    particles_flat: Vec<Particle>,
    particles_grid: IndexGrid,
} impl World {
    pub fn new() -> Self {
        Self {
            particles_flat: Vec::new(),
            particles_grid: IndexGrid::new(WIDTH, HEIGHT),
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
                self.particles_grid.set(target_grid_x as usize, target_grid_y as usize, Some(index));

                true
            },
            CellState::Occupied(other_index) => {
                let mut particle = self.particles_flat[index];
                let mut other_particle = self.particles_flat[other_index];

                if particle.element_type.density() <= other_particle.element_type.density() {
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
                
                self.particles_grid.set(target_grid_x as usize, target_grid_y as usize, Some(index));
                self.particles_grid.set(old_grid_x, old_grid_y, Some(other_index));
                
                true
            },
            CellState::OutOfBounds => false,
        }
    }

    pub fn update_physics(&mut self) {
        for index in 0..self.particles_flat.len() {
            let mut particle = self.particles_flat[index];


            match particle.element_type.kind() {
                ElementKind::Powder => {
                    let bias: f32 = if rand::random::<bool>() { 1.0 } else { -1.0 };

                    if !self.try_move_particle(index, particle.x, particle.y + 1.0) {
                        if !self.try_move_particle(index, particle.x + bias, particle.y + 1.0) {
                            self.try_move_particle(index, particle.x - bias, particle.y + 1.0);
                        };
                    };
                },
                ElementKind::Liquid => {
                    let bias: f32 = if rand::random::<bool>() { 1.0 } else { -1.0 };

                    if !self.try_move_particle(index, particle.x, particle.y + 1.0) {
                        if !self.try_move_particle(index, particle.x + bias, particle.y + 1.0) {
                            if !self.try_move_particle(index, particle.x - bias, particle.y + 1.0) {
                                if !self.try_move_particle(index, particle.x + bias, particle.y) {
                                    self.try_move_particle(index, particle.x - bias, particle.y);
                                };
                            };
                        };
                    };
                },
                ElementKind::Gas => {
                    let bias: f32 = if rand::random::<bool>() { 1.0 } else { -1.0 };

                    if !self.try_move_particle(index, particle.x + bias, particle.y - 1.0) {
                        if !self.try_move_particle(index, particle.x, particle.y - 1.0) {
                            if !self.try_move_particle(index, particle.x + bias, particle.y - 1.0) {
                                if !self.try_move_particle(index, particle.x - bias, particle.y - 1.0) {
                                    if !self.try_move_particle(index, particle.x + bias, particle.y) {
                                        self.try_move_particle(index, particle.x - bias, particle.y);
                                    };
                                };
                            };
                        };
                    };
                },
                ElementKind::Solid => {
                    // Solids sit still, they don't even move with velocity.
                },
                ElementKind::Energy => {
                    // Normally energy particles would move with velocity but we don't have that implemented.
                },
            }
        } 
    }
}

const WIDTH: usize = 320;
const HEIGHT: usize = 180;

fn main() {
    let world: World = World::new();
}
