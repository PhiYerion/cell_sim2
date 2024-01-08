pub mod cell;
pub mod component;
pub mod genetics;
pub mod physics;

#[cfg(test)]
mod tests {
    use crate::physics::World;
    use nalgebra::vector;

    #[test]
    fn full_test() {
        let mut world = World::default();
        (0..250).for_each(|_| {
            world.add_cell(vector![rand::random(), rand::random()]);
        });

        (0..250).for_each(|_| {
            world.update();
        })
    }
}
