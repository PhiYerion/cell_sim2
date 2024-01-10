use rapier2d::prelude::*;

use crate::cell::chemicals::{ATP_SIZE, GLUCOSE_SIZE};
use crate::cell::Cell;

use super::ComponentProps;

pub fn glycolysis(props: &ComponentProps, cell: &mut Cell) {
    let amount = props.get_input_output_amt(cell.inner.chemicals.glucose);
    cell.inner.chemicals.glucose -= amount.input;
    cell.inner.chemicals.atp += amount.output;

    cell.modify_size(amount.output * GLUCOSE_SIZE - amount.input * ATP_SIZE);
}
