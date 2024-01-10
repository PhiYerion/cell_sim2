pub mod chemicals;
pub mod component;
mod inner;
mod membrane;

use nalgebra::{Vector2, vector};

use self::chemicals::Chemicals;
use self::component::{ComponentProps, COMPONENT_COUNT, get_components};
use self::inner::Inner;
use self::membrane::Membrane;


#[derive(Clone, Copy, Default)]
pub struct Cell {
    pub inner: Inner,
    pub membrane: Membrane,
    pub components: [Option<ComponentProps>; COMPONENT_COUNT],
    size: f32,
    pub vel: Vector2<f32>,
    pub size_changed: bool,
    pub velocity_changed: bool,
}

impl Cell {
    pub fn new(
        inner: Inner,
        membrane: Membrane,
        components: [Option<ComponentProps>; COMPONENT_COUNT],
    ) -> Self {
        let mut size = inner.size() + membrane.size();
        components.iter().flatten().for_each(|component| {
            size += component.size();
        });

        Self {
            inner,
            membrane,
            components,
            size,
            size_changed: false,
            vel: vector![0.0, 0.0],
            velocity_changed: false,
        }
    }

    pub fn modify_size(&mut self, size_change: f32) {
        self.size += size_change;
        self.size_changed = true;
    }

    pub fn size(&self) -> f32 {
        debug_assert!(self.size >= 0.);
        self.size * 0.001
    }

    pub fn set_velocity(&mut self, vel: Vector2<f32>) {
        self.vel = vel;
        self.velocity_changed = true;
    }

    pub fn new_random() -> Self {
        let inner = Inner {
            chemicals: Chemicals {
                atp: rand::random::<f32>() * 10.,
                ..Default::default()
            },
            ph: rand::random::<f32>() * 10.,
            test: rand::random::<f32>() * 10.,
            nucleotides: rand::random::<f32>() * 10.,
            proteins: rand::random::<f32>() + 10.,
        };
        let membrane = Membrane {};
        let mut components = [None; COMPONENT_COUNT];
        (0..COMPONENT_COUNT).for_each(|i| {
            components[i] = Some(ComponentProps::random());
        });

        Self::new(inner, membrane, components)
    }

    pub fn inject_component(&mut self, compoent_index: usize, component: ComponentProps) {
        self.components
            .get_mut(compoent_index)
            .unwrap()
            .replace(component);

        self.size += component.size();
        self.size_changed = true;
    }

    pub fn generate_size(&self) -> f32 {
        let mut size = 0.0;
        self.components.iter().flatten().for_each(|component| {
            size += component.size();
        });

        size + self.inner.size() + self.membrane.size()
    }

    #[rustfmt::skip]
    pub fn run_components(&mut self) {
        let component_functions = get_components();
        let component_props = self.components;
        component_functions
            .iter()
            .zip(component_props.iter())
            .for_each(|(component_function, component_props_option)| 
                match component_props_option {
                    Some(component_props) => {
                            component_function(component_props, self);
                    }
                    None => {}
                },
            )
    }
}
