use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

#[derive(Component, Clone, Copy, Debug)]
pub struct CellId {
    pub cell_id: usize,
}

#[derive(Bundle, Clone)]
pub struct CellBundle {
    pub material_mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
    pub cell_id: CellId,
}

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
