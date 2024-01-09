use crate::cell::Cell;
use crate::component::ComponentProps;
use nalgebra::Vector2;
use rapier2d::dynamics::{RigidBody, RigidBodyBuilder, RigidBodyHandle, RigidBodySet};
use rapier2d::geometry::{Collider, ColliderBuilder, ColliderHandle, ColliderSet, SharedShape};

use super::cell_wrapper::CellWrapper;
use super::physics_props::PhysicsPropsStruct;

#[derive(Default)]
pub struct World {
    pub cells: Vec<CellWrapper>,
    pub rigid_body_set: RigidBodySet,
    pub collider_set: ColliderSet,
    pub physics_props: PhysicsPropsStruct,

    free_indexes: Vec<usize>,
}

impl World {
    fn inject_cell(
        &mut self,
        cell: Cell,
        collider_handle: ColliderHandle,
        rigid_body_handle: RigidBodyHandle,
    ) -> usize {
        let mut cell_wrapper = CellWrapper {
            inner: cell,
            collider_handle,
            rigid_body_handle,
            index: 0,
        };
        match self.free_indexes.pop() {
            Some(index) => {
                cell_wrapper.index = index;
                self.cells.get_mut(index).replace(&mut cell_wrapper);

                cell_wrapper.index
            }
            None => {
                cell_wrapper.index = self.cells.len();
                self.cells.push(cell_wrapper);

                self.cells.len() - 1
            }
        }
    }

    fn inject_cell_bundle(
        &mut self,
        cell: Cell,
        collider: Collider,
        rigid_body: RigidBody,
    ) -> usize {
        let rigid_body_handle = self.rigid_body_set.insert(rigid_body);
        let collider_handle = self.collider_set.insert_with_parent(
            collider,
            rigid_body_handle,
            &mut self.rigid_body_set,
        );

        self.inject_cell(cell, collider_handle, rigid_body_handle)
    }

    pub fn add_cell(&mut self, cell: Cell, position: Vector2<f32>) -> usize {
        let collider = ColliderBuilder::ball(cell.size).build();
        let rigid_body = RigidBodyBuilder::dynamic().translation(position).build();

        self.inject_cell_bundle(cell, collider, rigid_body)
    }

    pub fn inject_component(
        &mut self,
        cell_index: usize,
        component_index: usize,
        component: ComponentProps,
    ) {
        let cell_wrapper = self.cells.get_mut(cell_index).unwrap();
        cell_wrapper
            .inner
            .inject_component(component_index, component);
        self.collider_set
            .get_mut(cell_wrapper.collider_handle)
            .unwrap()
            .set_shape(SharedShape::ball(cell_wrapper.inner.size))
    }

    pub fn update(&mut self) {
        self.update_cells();
        self.update_physics();
    }

    pub fn update_cells(&mut self) {
        self.cells
            .iter_mut()
            .zip(self.rigid_body_set.iter())
            .zip(self.collider_set.iter())
            .for_each(|((cell, (_, rigid_body)), (_, collider))| {
                cell.inner.run_components(rigid_body, collider);
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
