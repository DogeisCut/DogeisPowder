use crate::world::World;

pub struct Game {
    world: World,
} impl Game {
    fn tick(&mut self) {
        self.world.populate_grid();
        self.world.update_physics();
    }
}