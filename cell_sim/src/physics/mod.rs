mod physics_props;

use nalgebra::Vector2;
use rapier2d::prelude::*;

use crate::cell::Cell;

use self::physics_props::PhysicsPropsStruct;

#[derive(Default)]
pub struct World {
    pub cells: Vec<Cell>,
    pub rigid_body_set: RigidBodySet,
    pub collider_set: ColliderSet,
    pub physics_props: PhysicsPropsStruct,
}

impl World {
    pub fn add_cell(&mut self, position: Vector2<f32>) {
        const SIZE: f32 = 1.0;
        let collider = ColliderBuilder::ball(SIZE).build();
        let rigid_body = RigidBodyBuilder::dynamic().translation(position).build();

        self.cells.push(Cell::default());

        let rigid_body_handle = self.rigid_body_set.insert(rigid_body);
        self.collider_set
            .insert_with_parent(collider, rigid_body_handle, &mut self.rigid_body_set);
    }

    pub fn update(&mut self) {
        self.update_cells();
        self.update_physics();
    }

    pub fn update_cells(&mut self) {
        self.cells.iter_mut()
            .zip(self.rigid_body_set.iter())
            .zip(self.collider_set.iter())
            .for_each(|(( cell, (_, rigid_body)), (_, collider))| {
                cell.run_components(rigid_body, collider);
        });
    }

    pub fn update_physics(&mut self) {
        let physics_props = &mut self.physics_props;
        physics_props.physics_pipeline.step(
            &physics_props.gravity,
            &physics_props.integration_parameters,
            &mut physics_props.island_manager,
            &mut physics_props.broad_phase,
            &mut physics_props.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut physics_props.impulse_joint_set,
            &mut physics_props.multibody_joint_set,
            &mut physics_props.ccd_solver,
            None,
            &(),
            &(),
        );
    }
}

#[cfg(test)]
mod tests {
    use nalgebra::vector;
    use crate::physics::World;

    #[test]
    fn test_phsyics() {
        let mut world = World::default();
        (0..250).for_each(|_| {
            world.add_cell(vector![rand::random(), rand::random()]);
        });

        (0..250).for_each(|_| {
            world.update_physics();
        })
    }

    #[test]
    fn test_cells() {
        let mut world = World::default();
        (0..250).for_each(|_| {
            world.add_cell(vector![rand::random(), rand::random()]);
        });

        (0..250).for_each(|_| {
            world.update_cells();
        })
    }
}
