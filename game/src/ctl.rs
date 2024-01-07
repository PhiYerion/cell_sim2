use crate::cell_wrapper::CellWrapper;

use super::cell_bundle::{update_cell_mesh, CellBundle};
use bevy::log;
use bevy::window::PrimaryWindow;
use bevy::{prelude::*, sprite::Mesh2dHandle};
use cell_sim::cell::Cell;

type CellZip<'a> = (
    Entity,
    &'a mut CellWrapper,
    &'a mut Mesh2dHandle,
    &'a mut Handle<ColorMaterial>,
    &'a mut Transform,
);

pub fn update_all_cells(
    mut commands: Commands,
    mut cell_zip: Query<CellZip>,
    time: Res<Time>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut color_assets: ResMut<Assets<ColorMaterial>>,
) {
    log::info!("cells: {}", cell_zip.iter().len());
    let cell_count = cell_zip.iter().len();
    for (
        entity,
        mut cell,
        mut mesh,
        mut color,
        mut transform,
    ) in cell_zip.iter_mut()
    {
        update_cell_mesh(
            &mut cell.inner,
            &mut mesh,
            &mut color,
            &mut mesh_assets,
            &mut color_assets,
        );
    }
}

fn spawn_cell(
    cell: Cell,
    commands: &mut Commands,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
    position: Vec3,
) {
    let bundle = CellBundle::new(meshes, materials, cell, position);
    commands.spawn(bundle);
}

pub fn spawn_cells(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();
    (0..1000).enumerate().for_each(|_| {
        spawn_cell(
            Cell::default(),
            &mut commands,
            &mut materials,
            &mut meshes,
            Vec3::new(
                rand::random::<f32>() * window.width(),
                rand::random::<f32>() * window.height(),
                0.,
            ),
        )
    });
}
