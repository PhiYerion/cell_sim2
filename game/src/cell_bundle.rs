use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use cell_sim::cell::Cell;

#[derive(Component)]
struct CellId {
    pub cell_id: usize,
}

#[derive(Bundle)]
pub struct CellBundle {
    pub material_mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
    pub cell_id: CellId,
}

const CELL_SIZE_MODIFIER: f32 = 0.02;

impl CellBundle {
    pub fn new(
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<ColorMaterial>,
        pos: Vec2,
        cell_id: usize,
    ) -> Self {
        Self {
            material_mesh_bundle: MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(3.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::PURPLE)),
                transform: Transform::from_xyz(pos.x, pos.y, 0.),
                ..default()
            },
            cell_id: CellId { cell_id },
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
