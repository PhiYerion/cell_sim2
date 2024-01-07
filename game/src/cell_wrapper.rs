use bevy::ecs::component::Component;
use cell_sim::cell::Cell;

#[derive(Component)]
pub struct CellWrapper {
    pub inner: Cell,
}
