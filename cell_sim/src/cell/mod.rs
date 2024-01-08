use rapier2d::dynamics::RigidBody;
use rapier2d::geometry::Collider;

use crate::component::get_components;

use self::inner::Inner;
use self::membrane::Membrane;
use super::component::{ComponentProps, COMPONENT_COUNT};

mod chemicals;
mod inner;
mod membrane;

#[derive(Clone, Copy, Default)]
pub struct Cell {
    pub inner: Inner,
    pub membrane: Membrane,
    pub components: [Option<ComponentProps>; COMPONENT_COUNT],
}

impl Cell {
    pub fn inject_component(&mut self, compoent_index: usize, component: ComponentProps) {
        self.components
            .get_mut(compoent_index)
            .unwrap()
            .replace(component);
    }

    pub fn size(&self) -> f32 {
        let mut size = 0.0;
        self.components.iter().flatten().for_each(|component| {
            size += component.size;
        });

        size + self.inner.size() + self.membrane.size()
    }

    pub fn run_components(&mut self, rigid_body: &RigidBody, collider: &Collider) {
        let component_functions = get_components();
        let component_props = self.components;
        component_props.iter().enumerate().for_each(
            |(idx, component_option)| match component_option {
                Some(component) => {
                    component_functions[idx](component, self, rigid_body, collider);
                }
                None => {}
            },
        )
    }
}
