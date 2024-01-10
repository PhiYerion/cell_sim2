use rapier2d::prelude::*;

use crate::cell::chemicals::ATP_SIZE;
use crate::cell::Cell;

use super::ComponentProps;

pub fn flangella(props: &ComponentProps, cell: &mut Cell, _: &RigidBody, _: &Collider) {
    let amount = props.get_input_output_amt(cell.inner.chemicals.atp);
    cell.inner.chemicals.atp -= amount.input;
    cell.modify_size(-amount.input * ATP_SIZE);

    let left = rand::random::<f32>();
    let right = 1. - left;
    cell.vel += vector![left * amount.output, right * amount.output];
}
