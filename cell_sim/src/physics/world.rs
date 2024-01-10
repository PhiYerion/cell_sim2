use std::time::Duration;

use crate::cell::component::ComponentProps;
use crate::cell::Cell;
use crate::physics::updates::{update_physics, update_cells};
use nalgebra::Vector2;
use rapier2d::dynamics::{RigidBody, RigidBodyBuilder, RigidBodyHandle, RigidBodySet};
use rapier2d::geometry::{Collider, ColliderBuilder, ColliderHandle, ColliderSet, SharedShape};

use super::cell_wrapper::CellWrapper;
use super::physics_props::PhysicsPropsStruct;

#[derive(Default)]
pub struct World {
    pub cells: Vec<Option<CellWrapper>>,
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
    pub rigid_body_handle: RigidBodyHandle,
    pub collider_handle: ColliderHandle,
    pub impulse: Option<Vector2<f32>>,
    pub size: Option<f32>,
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
                self.cells[index] = Some(cell_wrapper);

                index
            }
            None => {
                cell_wrapper.index = self.cells.len();
                self.cells.push(Some(cell_wrapper));

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

    pub fn remove_cell(&mut self, cell_idx: usize) {
        self.cells[cell_idx] = None;
        self.free_indexes.push(cell_idx)
    }

    pub fn inject_component(
        &mut self,
        cell_index: usize,
        component_index: usize,
        component: ComponentProps,
    ) {
        let cell_wrapper_option = self.cells.get_mut(cell_index).unwrap();
        if let Some(cell_wrapper) = cell_wrapper_option {
            cell_wrapper
                .inner
                .inject_component(component_index, component);
            self.collider_set
                .get_mut(cell_wrapper.collider_handle)
                .unwrap()
                .set_shape(SharedShape::ball(cell_wrapper.inner.size()))
        }
    }

    pub fn update(&mut self) {
        let mut cell_changes: Vec<Option<CellChanges>> = Vec::with_capacity(self.cells.len());
        #[cfg(feature = "parallel")]
        {
            let mut update_cells_time = Duration::default();
            let mut update_physics_time = Duration::default();
            rayon::join(
                || {
                    let start_time = std::time::Instant::now();
                    cell_changes = update_cells(&mut self.cells);
                    update_cells_time = start_time.elapsed();
                },
                || {
                    let start_time = std::time::Instant::now();
                    update_physics(&mut self.physics_props, &mut self.rigid_body_set, &mut self.collider_set);
                    update_physics_time = start_time.elapsed();
                }
            );

            #[cfg(debug_assertions)] {
                self.cell_time += update_cells_time;
                self.physics_time += update_physics_time;
            }
        }

        #[cfg(not(feature = "parallel"))] {
            let cell_changes = update_cells(self.cells.as_mut_slice());
            update_physics(&mut self.physics_props, &mut self.rigid_body_set, &mut self.collider_set);
        }

        let start_time = std::time::Instant::now();
        cell_changes.iter().enumerate().for_each(|( idx, change_option )| {
            match change_option {
                Some(change) => {
                    if let Some(impulse) = change.impulse {
                        let rigid_body = self.rigid_body_set.get_mut(change.rigid_body_handle).unwrap();
                        rigid_body.apply_impulse(impulse * 100., true);
                    }
                    if let Some(size) = change.size {
                        let collider = self.collider_set.get_mut(change.collider_handle).unwrap();
                        collider.set_shape(SharedShape::ball(size));
                    }
                }
                None => {
                    self.remove_cell(idx);
                }
            }
        });
        #[cfg(debug_assertions)] {
            self.replication_time += start_time.elapsed();
        }
    }
}
