mod test_comp;
use rapier2d::dynamics::RigidBody;
use rapier2d::geometry::Collider;

use crate::cell::Cell;

pub const COMPONENT_COUNT: usize = 1;

pub type Component = fn(&ComponentProps, &mut Cell, &RigidBody, &Collider);

#[derive(Debug, Clone, Copy)]
pub struct ComponentProps {
    pub proteins: f32,
    pub size: f32,
}

impl Default for ComponentProps {
    fn default() -> Self {
        Self {
            proteins: 2.0,
            size: 2.0,
        }
    }
}

pub fn get_components() -> [Component; COMPONENT_COUNT] {
    [test_comp::test_comp]
}
