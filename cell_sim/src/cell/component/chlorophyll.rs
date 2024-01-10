use rapier2d::prelude::*;

use crate::cell::chemicals::GLUCOSE_SIZE;
use crate::cell::Cell;

use super::ComponentProps;

pub fn chlorophyll(props: &ComponentProps, cell: &mut Cell, _: &RigidBody, _: &Collider) {
    let amount = props.get_input_output_amt(0.);
    cell.inner.chemicals.glucose += amount.output;
    cell.modify_size(amount.output * GLUCOSE_SIZE);
}
