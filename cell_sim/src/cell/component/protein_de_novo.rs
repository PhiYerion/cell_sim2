use rapier2d::prelude::*;

use crate::cell::chemicals::ATP_SIZE;
use crate::cell::inner::PROTEIN_SIZE;
use crate::cell::Cell;

use super::ComponentProps;

pub fn protein_de_novo(props: &ComponentProps, cell: &mut Cell, _: &RigidBody, _: &Collider) {
    let amount = props.get_input_output_amt(cell.inner.chemicals.atp);
    cell.inner.chemicals.atp -= amount.input;
    cell.inner.proteins += amount.output;

    cell.modify_size(amount.output * PROTEIN_SIZE - amount.input * ATP_SIZE);
}
