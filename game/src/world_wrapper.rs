use bevy::prelude::*;
use bevy::sprite::Mesh2dHandle;
use cell_sim::cell::Cell;
use cell_sim::physics::World;
use nalgebra::vector;

use crate::cell_bundle::{CellBundle, CellId};

#[derive(Default, Resource)]
pub struct WorldWrapper {
    pub world: World,
}

impl WorldWrapper {
    pub fn add_cell(
        &mut self,
        cell: Cell,
        pos: Vec2,
        commands: &mut Commands,
        meshes: &mut Assets<Mesh>,
        color_materials: &mut Assets<ColorMaterial>,
    ) {
        let cell_idx = self.world.add_cell(cell, vector![pos.x, pos.y]);
        let cell_bundle = CellBundle::new(meshes, color_materials, pos, cell_idx);
        commands.spawn(cell_bundle);
    }
}

pub fn thousand_cells(
    mut world_wrapper: ResMut<WorldWrapper>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    for _ in 0..1000 {
        let random_cell = Cell::new_random();
        world_wrapper.add_cell(
            random_cell,
            Vec2::new(rand::random::<f32>() * 1000., rand::random::<f32>() * 1200.),
            &mut commands,
            meshes.as_mut(),
            color_materials.as_mut(),
        );
    }
}

pub fn update(
    mut world_wrapper: ResMut<WorldWrapper>,
    time: Res<Time>,
    mut cell_bundles: Query<(
        Entity,
        &CellId,
        &mut Mesh2dHandle,
        &mut Handle<ColorMaterial>,
        &mut Transform,
    )>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    world_wrapper.world.update();
    cell_bundles
        .iter_mut()
        .for_each(|(entity, cell_id, mut mesh, mut color, mut transform)| {
            let cell = world_wrapper.world.cells.get(cell_id.cell_id).unwrap();

            // Mesh
            *mesh = meshes
                .add(shape::Circle::new(cell.inner.size).into())
                .into();

            // Translation
            let rigid_body_handle = cell.rigid_body_handle;
            let rigid_body = world_wrapper
                .world
                .rigid_body_set
                .get(rigid_body_handle)
                .unwrap();
            let pos = rigid_body.position().translation.vector;
            transform.translation = Vec3::new(
                pos.x,
                pos.y,
                0.,
            );
        });
}
