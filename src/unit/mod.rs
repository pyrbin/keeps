use bevy_spatial::{KDTreeAccess2D, KDTreePlugin2D};

use crate::prelude::*;

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(KDTreePlugin2D::<Unit> { ..default() });
        app.add_system(unit_movement);
    }
}

#[derive(Component)]
pub struct Unit;

pub type UnitSpacialTree = KDTreeAccess2D<Unit>;

#[derive(Default, Component)]
pub struct MovementSpeed(pub f32);

#[derive(Default, Component)]
pub struct MoveDirection(pub Option<Vec3>);

fn unit_movement(
    mut query: Query<(
        &Unit,
        &MovementSpeed,
        &MoveDirection,
        &mut Transform,
        Entity,
    )>,
) {
    for (_, speed, direction, mut transform, _) in query.iter_mut() {
        if let Some(move_dir) = direction.0 {
            transform.translation += move_dir * speed.0;
        }
    }
}
