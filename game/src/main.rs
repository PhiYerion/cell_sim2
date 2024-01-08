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
        .add_systems(Startup, test)
        .run();
}

fn test(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(5.).into()).into(),
        material: color_materials.add(ColorMaterial::from(Color::PURPLE)),
        transform: Transform::from_translation(Vec3::new(
            window.width() / 2.,
            window.height() / 2.,
            0.,
        )),
        ..default()
    });
}
