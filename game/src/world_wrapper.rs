use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use cell_sim::physics::World;
use nalgebra::vector;

use crate::cell_bundle::CellBundle;

#[derive(Default)]
pub struct WorldWrapper {
    pub world: World,
}

impl WorldWrapper {
    pub fn add_cell(
        &mut self,
        pos: Vec2,
        commands: &mut Commands,
        meshes: &mut Assets<Mesh>,
        color_materials: &mut Assets<ColorMaterial>,
    ) {
        let cell_idx = self.world.add_cell(vector![pos.x, pos.y]);
        commands.spawn(CellBundle::new(meshes, color_materials, pos, cell_idx));
    }

    pub fn thousand_cells(
        &mut self,
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut color_materials: ResMut<Assets<ColorMaterial>>,
    ) {
        for _ in 0..10000 {
            self.add_cell(
                Vec2::new(rand::random::<f32>() * 1000., rand::random::<f32>() * 1200.),
                &mut commands,
                meshes.as_mut(),
                color_materials.as_mut(),
            );
        }
    }
}
