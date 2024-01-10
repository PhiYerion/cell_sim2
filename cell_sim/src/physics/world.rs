use std::time::Duration;

use crate::cell::component::ComponentProps;
use crate::cell::Cell;
use nalgebra::Vector2;
use rapier2d::dynamics::{RigidBody, RigidBodyBuilder, RigidBodyHandle, RigidBodySet};
use rapier2d::geometry::{Collider, ColliderBuilder, ColliderHandle, ColliderSet, SharedShape};
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

use super::cell_wrapper::CellWrapper;
use super::physics_props::PhysicsPropsStruct;

#[derive(Default)]
pub struct World {
    pub cells: Vec<CellWrapper>,
    pub rigid_body_set: RigidBodySet,
    pub collider_set: ColliderSet,
    pub physics_props: PhysicsPropsStruct,

    free_indexes: Vec<usize>,

    #[cfg(debug_assertions)]
    pub cell_time: Duration,
    #[cfg(debug_assertions)]
    pub physics_time: Duration,
    #[cfg(debug_assertions)]
    pub replication_time: Duration,
}

pub struct CellChanges {
    rigid_body_handle: RigidBodyHandle,
    collider_handle: ColliderHandle,
    velocity: Option<Vector2<f32>>,
    size: Option<f32>,
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
        let collider = ColliderBuilder::ball(cell.size()).build();
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
            .set_shape(SharedShape::ball(cell_wrapper.inner.size()))
    }

    pub fn update(&mut self) {
        #[cfg(feature = "parallel")]
        {
            let mut cell_changes: Vec<CellChanges> = Vec::with_capacity(self.cells.len());
            let mut update_cells_time = Duration::default();
            let mut update_physics_time = Duration::default();
            rayon::join(
                || {
                    let start_time = std::time::Instant::now();
                    cell_changes = World::update_cells(self.cells.as_mut_slice());
                    update_cells_time = start_time.elapsed();
                },
                || {
                    let start_time = std::time::Instant::now();
                    World::update_physics(&mut self.physics_props, &mut self.rigid_body_set, &mut self.collider_set);
                    update_physics_time = start_time.elapsed();
                }
            );

            let start_time = std::time::Instant::now();
            cell_changes.iter().for_each(|change| {
                if let Some(velocity) = change.velocity {
                    let rigid_body = self.rigid_body_set.get_mut(change.rigid_body_handle).unwrap();
                    rigid_body.set_linvel(velocity, true);
                }
                if let Some(size) = change.size {
                    let collider = self.collider_set.get_mut(change.collider_handle).unwrap();
                    collider.set_shape(SharedShape::ball(size));
                }
            });
            #[cfg(debug_assertions)] {
                self.replication_time += start_time.elapsed();
                self.cell_time += update_cells_time;
                self.physics_time += update_physics_time;
            }
        }
        #[cfg(not(feature = "parallel"))] {
            let cell_changes = World::update_cells(self.cells.as_mut_slice());
            cell_changes.iter().for_each(|change| {
                if let Some(velocity) = change.velocity {
                    let rigid_body = self.rigid_body_set.get_mut(change.rigid_body_handle).unwrap();
                    rigid_body.set_linvel(velocity, true);
                }
                if let Some(size) = change.size {
                    let collider = self.collider_set.get_mut(change.collider_handle).unwrap();
                    collider.set_shape(SharedShape::ball(size));
                }
            });
            World::update_physics(&mut self.physics_props, &mut self.rigid_body_set, &mut self.collider_set);
        }
    }

    pub fn update_cells(cells: &mut [CellWrapper]) -> Vec<CellChanges> {
        let update = |cell: &mut CellWrapper| {
            (0..400).for_each(|_| {
                cell.inner.run_components();
            });
            let velocity = match cell.inner.velocity_changed {
                true => {
                    cell.inner.velocity_changed = false;
                    Some(cell.inner.vel)
                }
                false => None,
            };
            let size = match cell.inner.size_changed {
                true => {
                    cell.inner.size_changed = false;
                    Some(cell.inner.size())
                }
                false => None,
            };

            CellChanges {
                rigid_body_handle: cell.rigid_body_handle,
                collider_handle: cell.collider_handle,
                velocity,
                size,
            }
        };

        #[cfg(feature = "parallel")]
        let collection: Vec<CellChanges> = cells.par_iter_mut().map(update).collect();
        #[cfg(not(feature = "parallel"))]
        let collection: Vec<CellChanges> = cells.iter_mut().map(update).collect();

        collection
    }

    pub fn update_physics(physics_props: &mut PhysicsPropsStruct, rigid_body_set: &mut RigidBodySet, collider_set: &mut ColliderSet) {
        physics_props.physics_pipeline.step(
            &physics_props.gravity,
            &physics_props.integration_parameters,
            &mut physics_props.island_manager,
            &mut physics_props.broad_phase,
            &mut physics_props.narrow_phase,
            rigid_body_set,
            collider_set,
            &mut physics_props.impulse_joint_set,
            &mut physics_props.multibody_joint_set,
            &mut physics_props.ccd_solver,
            None,
            &(),
            &(),
        );
    }
}
