mod cell_wrapper;
mod physics_props;
mod world;
pub use world::World;

#[cfg(test)]
mod tests {
    use nalgebra::vector;

    use super::World;

    #[test]
    fn test_phsyics() {
        let mut world = World::default();
        (0..250).for_each(|_| {
            world.add_cell(vector![rand::random(), rand::random()]);
        });

        (0..250).for_each(|_| {
            world.update_physics();
        })
    }

    #[test]
    fn test_cells() {
        let mut world = World::default();
        (0..250).for_each(|_| {
            world.add_cell(vector![rand::random(), rand::random()]);
        });

        (0..250).for_each(|_| {
            world.update_cells();
        })
    }
}
