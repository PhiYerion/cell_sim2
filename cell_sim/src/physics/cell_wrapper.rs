use rapier2d::dynamics::RigidBodyHandle;
use rapier2d::geometry::ColliderHandle;

use crate::cell::Cell;

pub struct CellWrapper {
    pub inner: Cell,
    pub collider_handle: ColliderHandle,
    pub rigid_body_handle: RigidBodyHandle,
    pub index: usize,
}
