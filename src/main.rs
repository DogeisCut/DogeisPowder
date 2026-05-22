use rand::Rng;

enum ElementKind {
    Powder,
    Liquid,
    Gas,
    Solid
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
        IndexGrid {
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
        World {
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

    pub fn update_physics(&mut self) {
        for index in 0..self.particles_flat.len() {
            let mut particle = self.particles_flat[index];

            let grid_x: isize = particle.get_grid_x() as isize;
            let grid_y: isize = particle.get_grid_y() as isize;


            let picked: isize = if rand::random::<bool>() { 1 } else { -1 };
        
            if self.particles_grid.get(grid_x, grid_y + 1) == CellState::Empty {
                particle.y += 1.0;
            } else {
                if self.particles_grid.get(grid_x + picked, grid_y + 1) == CellState::Empty {
                    particle.x += picked as f32;
                    particle.y += 1.0;
                } else if self.particles_grid.get(grid_x - picked, grid_y + 1) == CellState::Empty {
                    particle.x -= picked as f32;
                    particle.y += 1.0;
                }
            }
            
            self.particles_flat[index] = particle;
            self.particles_grid.set(grid_x as usize, grid_y as usize, None);

            let grid_x = particle.get_grid_x();
            let grid_y = particle.get_grid_y();
            
            self.particles_grid.set(grid_x, grid_y, Some(index));
        } 
    }
}

const WIDTH: usize = 320;
const HEIGHT: usize = 180;

fn main() {
}
