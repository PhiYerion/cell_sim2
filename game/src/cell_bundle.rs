use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy::prelude::*;
use cell_sim::cell::Cell;

use crate::cell_wrapper::CellWrapper;

#[derive(Bundle)]
pub struct CellBundle {
    pub material_mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
    pub cell: CellWrapper,
}

const CELL_SIZE_MODIFIER: f32 = 0.02;

impl CellBundle {
    pub fn new(
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<ColorMaterial>,
        cell: Cell,
        pos: Vec3,
    ) -> Self {
        Self {
            material_mesh_bundle: MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::Circle::new(3.).into())
                    .into(),
                material: materials.add(ColorMaterial::from(Color::PURPLE)),
                transform: Transform::from_xyz(pos.x, pos.y, 0.),
                ..default()
            },
            cell: CellWrapper { inner: cell },
        }
    }
}

pub fn update_cell_mesh(
    cell: &mut Cell,
    mesh: &mut Mesh2dHandle,
    color: &mut Handle<ColorMaterial>,
    mesh_assets: &mut ResMut<Assets<Mesh>>,
    color_assets: &mut ResMut<Assets<ColorMaterial>>,
) {
}
