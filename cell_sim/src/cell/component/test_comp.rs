use rapier2d::dynamics::RigidBody;
use rapier2d::geometry::Collider;

use crate::cell::Cell;

use super::ComponentProps;

pub fn test_comp(props: &ComponentProps, cell: &mut Cell, _: &RigidBody, _: &Collider) {
    cell.inner.test += props.proteins * props.size();
}
