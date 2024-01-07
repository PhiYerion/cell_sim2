mod cell_bundle;
mod cell_wrapper;
mod ctl;
mod scene;
use bevy::prelude::*;

use self::ctl::spawn_cells;
use self::scene::spawn_camera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_cells)
        .run();
}
