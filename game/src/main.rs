mod cell_bundle;
mod cell_wrapper;
mod scene;
mod world_wrapper;
use self::scene::{spawn_camera, move_camera};
use self::world_wrapper::{thousand_cells, update, WorldWrapper};
use bevy::prelude::*;
use bevy_fps_counter::FpsCounterPlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins(FpsCounterPlugin);
    app.add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_camera)
        .insert_resource(WorldWrapper::default())
        .add_systems(Startup, thousand_cells)
        .add_systems(Update, update)
        .add_systems(Update, move_camera)
        .run();
}
