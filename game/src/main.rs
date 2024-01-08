mod cell_bundle;
mod cell_wrapper;
mod scene;
mod update;
mod world_wrapper;
use self::scene::spawn_camera;
use self::world_wrapper::WorldWrapper;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::transform::commands;
use bevy::window::PrimaryWindow;

fn main() {
    let mut world = WorldWrapper::default();
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_camera)
        .add_systems(
            Startup,
            move |commands: Commands,
                  meshes: ResMut<Assets<Mesh>>,
                  color_materials: ResMut<Assets<ColorMaterial>>| {
                world.thousand_cells(commands, meshes, color_materials)
            },
        )
        .run();
}
