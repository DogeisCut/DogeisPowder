
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
}

#[derive(Clone, Copy, Debug)]
struct Particle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    element_type: Element,
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

    pub fn get(&self, x: usize, y: usize) -> Option<usize> {
        self.cells[x + (y * self.width)]
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
            
            let grid_x = (particle.x + 0.5) as usize;
            let grid_y = (particle.y + 0.5) as usize;
            
            self.particles_grid.set(grid_x, grid_y, Some(index));
        }
    }

    pub fn update_physics(&mut self) {
        
    }
}

const WIDTH: usize = 320;
const HEIGHT: usize = 180;

fn main() {
}
