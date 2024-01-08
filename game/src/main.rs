mod cell_bundle;
mod cell_wrapper;
mod scene;
mod update;
mod world_wrapper;
use self::cell_bundle::CellId;
use self::scene::spawn_camera;
use self::world_wrapper::{WorldWrapper, thousand_cells, update};
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy::transform::commands;
use bevy::window::PrimaryWindow;

fn main() {
    let mut world = WorldWrapper::default();
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_camera)
        .insert_resource(WorldWrapper::default())
        .add_systems(Startup, thousand_cells)
        .add_systems(Update, update)
        .run();
}
