mod chlorophyll;
mod flangella;
mod glycolysis;
mod nucleotide_de_novo;
mod protein_de_novo;
mod test_comp;
use rapier2d::dynamics::RigidBody;
use rapier2d::geometry::Collider;

use crate::STEP_SIZE;
use crate::cell::Cell;

use super::inner::PROTEIN_SIZE;

pub const COMPONENT_COUNT: usize = 6;

pub type Component = fn(&ComponentProps, &mut Cell, &RigidBody, &Collider);

#[derive(Debug, Clone, Copy)]
pub struct ComponentProps {
    proteins: f32,
    pub speed: f32,
    pub efficiency: f32,
}

impl ComponentProps {
    pub fn new(proteins: f32, speed: f32) -> Self {
        Self {
            proteins,
            speed,
            efficiency: get_efficiency(speed, proteins),
        }
    }

    pub fn size(&self) -> f32 {
        self.proteins * PROTEIN_SIZE
    }

    pub fn random() -> Self {
        Self::new(rand::random::<f32>() * 10., rand::random::<f32>())
    }

    pub fn get_input_output_amt(&self, constraint: f32) -> Amounts {
        let input = ( STEP_SIZE * self.speed ).min(constraint);
        Amounts {
            input,
            output: input * self.efficiency,
        }
    }
}

pub struct Amounts {
    input: f32,
    output: f32,
}

impl Default for ComponentProps {
    fn default() -> Self {
        Self {
            proteins: 2.0,
            speed: 1.0,
            efficiency: get_efficiency(1.0, 2.0),
        }
    }
}

fn get_efficiency(speed: f32, proteins: f32) -> f32 {
    1. / (1. + speed / proteins)
}

pub fn get_components() -> [Component; COMPONENT_COUNT] {
    [
        test_comp::test_comp,
        flangella::flangella,
        chlorophyll::chlorophyll,
        glycolysis::glycolysis,
        nucleotide_de_novo::nucleotide_de_novo,
        protein_de_novo::protein_de_novo,
    ]
}
