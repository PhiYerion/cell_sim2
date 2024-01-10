use bevy::sprite::Mesh2dHandle;
use bevy::{log, prelude::*};
use cell_sim::cell::Cell;
use cell_sim::physics::World;
use nalgebra::vector;

use crate::cell_bundle::{CellBundle, CellId};

#[derive(Default, Resource)]
pub struct WorldWrapper {
    pub world: World,
    #[cfg(debug_assertions)]
    pub world_update_time: std::time::Duration,
    #[cfg(debug_assertions)]
    pub bevy_update_time: std::time::Duration,
    #[cfg(debug_assertions)]
    pub frames: u32,
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
        let cell_bundle = CellBundle::new(meshes, color_materials, pos, cell.size(), cell_idx);
        commands.spawn(cell_bundle);
    }
}

pub fn thousand_cells(
    mut world_wrapper: ResMut<WorldWrapper>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    for _ in 0..10000 {
        let random_cell = Cell::new_random();
        world_wrapper.add_cell(
            random_cell,
            Vec2::new(
                rand::random::<f32>() * 10000.,
                rand::random::<f32>() * 1200.,
            ),
            &mut commands,
            meshes.as_mut(),
            color_materials.as_mut(),
        );
    }
}

pub fn update(
    mut world_wrapper: ResMut<WorldWrapper>,
    _time: Res<Time>,
    mut cell_bundles: Query<(
        Entity,
        &CellId,
        &mut Mesh2dHandle,
        &mut Handle<ColorMaterial>,
        &mut Transform,
    )>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut _materials: ResMut<Assets<ColorMaterial>>,
) {
    let start_time = std::time::Instant::now(); // For debug
    world_wrapper.world.update();

    let world_update_time = start_time.elapsed(); // For debug
    #[cfg(debug_assertions)]
    {
        world_wrapper.frames += 1;
        world_wrapper.world_update_time += world_update_time;
    }
    cell_bundles
        .iter_mut()
        .for_each(|(_entity, cell_id, mut mesh, mut _color, mut transform)| {
            let cell = world_wrapper.world.cells.get(cell_id.cell_id).unwrap();
            let rigid_body_handle = cell.rigid_body_handle;
            let rigid_body = world_wrapper
                .world
                .rigid_body_set
                .get(rigid_body_handle)
                .unwrap();

            // Mesh
            if cell.inner.size_changed {
                *mesh = meshes
                    .add(shape::Circle::new(cell.inner.size()).into())
                    .into();
            }

            // Translation
            if rigid_body.is_moving() {
                let pos = rigid_body.position().translation.vector;
                transform.translation = Vec3::new(pos.x, pos.y, 0.);
            }
        });

    #[cfg(debug_assertions)]
    {
        world_wrapper.bevy_update_time += start_time.elapsed() - world_update_time;
        log::info!("world_wrapper::update times:\n\tworld update: {:?}/f\n\t\tcell update: {:?}/f, \n\t\tphysics_update: {:?}/f, \n\tbevy update: {:?}/f",
                   world_wrapper.world_update_time / world_wrapper.frames,
                       world_wrapper.world.cell_time / world_wrapper.frames,
                       world_wrapper.world.physics_time / world_wrapper.frames,
                   world_wrapper.bevy_update_time / world_wrapper.frames);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_app() -> App {
        use bevy::core_pipeline::CorePipelinePlugin;
        use bevy::render::RenderPlugin;
        use bevy::sprite::SpritePlugin;
        use bevy::time::TimePlugin;

        let mut app = App::new();
        app.add_plugins((
            AssetPlugin::default(),
            RenderPlugin::default(),
            ImagePlugin::default(),
            CorePipelinePlugin,
            TimePlugin,
            SpritePlugin,
        ));

        app.insert_resource(WorldWrapper::default());

        app
    }

    #[test]
    fn test_world_wrapper_bevy_functions() {
        let mut app = setup_app();

        app.insert_resource(WorldWrapper::default());

        let thousand_cells = app.world.register_system(thousand_cells);
        let updateid = app.world.register_system(update);

        app.world.run_system(thousand_cells).unwrap();

        for _ in 0..10 {
            app.world.run_system(updateid).unwrap();
        }
    }
}
