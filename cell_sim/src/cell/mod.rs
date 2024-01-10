use rapier2d::dynamics::RigidBody;
use rapier2d::geometry::Collider;

use crate::component::get_components;

use self::chemicals::Chemicals;
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
    size: f32,
    pub size_changed: bool,
}

impl Cell {
    pub fn new(
        inner: Inner,
        membrane: Membrane,
        components: [Option<ComponentProps>; COMPONENT_COUNT],
    ) -> Self {
        let mut size = inner.size() + membrane.size();
        components.iter().flatten().for_each(|component| {
            size += component.size;
        });

        Self {
            inner,
            membrane,
            components,
            size,
            size_changed: false,
        }
    }

    #[inline]
    pub fn set_size(&mut self, size: f32) {
        self.size = size
    }

    #[inline]
    pub fn size(&self) -> f32 {
        self.size
    }

    pub fn new_random() -> Self {
        let inner = Inner {
            chemicals: Chemicals {
                atp: rand::random::<f32>() * 10.,
            },
            ph: rand::random::<f32>() * 10.,
            test: rand::random::<f32>() * 10.,
        };
        let membrane = Membrane {};
        let components = [None; COMPONENT_COUNT];

        Self::new(inner, membrane, components)
    }

    pub fn inject_component(&mut self, compoent_index: usize, component: ComponentProps) {
        self.components
            .get_mut(compoent_index)
            .unwrap()
            .replace(component);

        self.size += component.size;
        self.size_changed = true;
    }

    pub fn generate_size(&self) -> f32 {
        let mut size = 0.0;
        self.components.iter().flatten().for_each(|component| {
            size += component.size;
        });

        size + self.inner.size() + self.membrane.size()
    }

    pub fn run_components(&mut self, rigid_body: &RigidBody, collider: &Collider) {
        let component_functions = get_components();
        let component_props = self.components;
        component_functions
            .iter()
            .zip(component_props.iter())
            .for_each(
                |(component_function, component_option)| match component_option {
                    Some(component) => {
                        component_function(component, self, rigid_body, collider);
                    }
                    None => {}
                },
            )
    }
}
