use rapier2d::dynamics::RigidBodySet;
use rapier2d::geometry::ColliderSet;
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

use super::cell_wrapper::CellWrapper;
use super::physics_props::PhysicsPropsStruct;
use super::world::CellChanges;

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

pub fn update_physics(
    physics_props: &mut PhysicsPropsStruct,
    rigid_body_set: &mut RigidBodySet,
    collider_set: &mut ColliderSet,
) {
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
