pub mod cell;
pub mod physics;

pub const STEP_SIZE: f32 = 0.01;

#[cfg(test)]
mod tests {
    use crate::cell::Cell;
    use crate::physics::World;
    use nalgebra::vector;

    #[test]
    fn full_test() {
        let mut world = World::default();
        (0..250).for_each(|_| {
            world.add_cell(Cell::new_random(), vector![rand::random(), rand::random()]);
        });

        (0..250).for_each(|_| {
            world.update();
        })
    }
}
