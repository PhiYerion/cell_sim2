use bevy::ecs::system::{Commands, Query};
use bevy::time::Time;
use cell_sim::physics::World;
use nalgebra::vector;

use crate::world_wrapper::WorldWrapper;

pub fn build_world() -> World {
    let mut world = World::default();
    (0..250).for_each(|_| {
        world.add_cell(vector![
            rand::random::<f32>() * 100.,
            rand::random::<f32>() * 100.
        ]);
    });

    world
}
