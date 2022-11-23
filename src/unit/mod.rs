use crate::prelude::*;

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, _app: &mut App) {}
}

#[derive(Component, Default)]
pub struct Unit;
