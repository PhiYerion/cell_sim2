mod cell_bundle;
mod cell_wrapper;
mod scene;
mod world_wrapper;
use self::scene::spawn_camera;
use self::world_wrapper::{thousand_cells, update, WorldWrapper};
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_camera)
        .insert_resource(WorldWrapper::default())
        .add_systems(Startup, thousand_cells)
        .add_systems(Update, update)
        .run();
}
