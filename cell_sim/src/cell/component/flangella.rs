use rapier2d::prelude::*;

use crate::cell::chemicals::ATP_SIZE;
use crate::cell::Cell;

use super::ComponentProps;

pub fn flangella(props: &ComponentProps, cell: &mut Cell) {
    let amount = props.get_input_output_amt(cell.inner.chemicals.atp);
    cell.inner.chemicals.atp -= amount.input;
    cell.modify_size(-amount.input * ATP_SIZE);

    let (leftneg, rightneg) = (rand::random::<bool>(), rand::random::<bool>());
    let mut left = rand::random::<f32>();
    if leftneg {
        left *= -1.;
    }
    let mut right = 1. - left;
    if rightneg {
        right *= -1.;
    }
    cell.modify_impulse(vector![left * amount.output, right * amount.output]);
}
