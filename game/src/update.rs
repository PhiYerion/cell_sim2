use cell_sim::cell::Cell;
use cell_sim::physics::World;
use nalgebra::vector;

pub fn build_world() -> World {
    let mut world = World::default();
    (0..250).for_each(|_| {
        world.add_cell(
            Cell::new_random(),
            vector![rand::random::<f32>() * 100., rand::random::<f32>() * 100.],
        );
    });

    world
}
