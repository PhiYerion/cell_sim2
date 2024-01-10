mod cell_wrapper;
mod physics_props;
mod world;
pub use world::World;

#[cfg(test)]
mod tests {
    use nalgebra::vector;

    use crate::cell::Cell;

    use super::World;

    #[test]
    fn test_phsyics() {
        let mut world = World::default();
        (0..250).for_each(|_| {
            world.add_cell(Cell::new_random(), vector![rand::random(), rand::random()]);
        });

        (0..250).for_each(|_| {
            World::update_physics(
                &mut world.physics_props,
                &mut world.rigid_body_set,
                &mut world.collider_set,
            );
        })
    }

    #[test]
    fn test_cells() {
        let mut world = World::default();
        (0..250).for_each(|_| {
            world.add_cell(Cell::new_random(), vector![rand::random(), rand::random()]);
        });

        (0..250).for_each(|_| {
            World::update_cells(&mut world.cells);
        })
    }
}
