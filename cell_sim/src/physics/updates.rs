use nalgebra::Vector2;
use rapier2d::dynamics::RigidBodySet;
use rapier2d::geometry::ColliderSet;
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

use super::cell_wrapper::CellWrapper;
use super::physics_props::PhysicsPropsStruct;
use super::world::CellChanges;

pub fn update_cells(cells: &mut [Option<CellWrapper>]) -> Vec<Option<CellChanges>> {
    let update = |cell: &mut CellWrapper| {
        for _ in 0..300 {
            if cell.inner.dead { return None }
            cell.inner.run_components();
        }
        let impulse = match cell.inner.velocity_changed {
            true => {
                let impulse = cell.inner.impulse;
                cell.inner.impulse = Vector2::new(0., 0.);
                Some(impulse)
            }
            false => None,
        };
        let size = match cell.inner.size_changed {
            true => {
                Some(cell.inner.size())
            }
            false => None,
        };

        Some(CellChanges {
            rigid_body_handle: cell.rigid_body_handle,
            collider_handle: cell.collider_handle,
            impulse,
            size,
        })
    };

    #[cfg(feature = "parallel")]
    let collection: Vec<Option<CellChanges>> = cells.par_iter_mut().flatten().map(update).collect();
    #[cfg(not(feature = "parallel"))]
    let collection: Vec<Option<CellChanges>> = cells.iter_mut().flatten().map(update).collect();

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
