use crate::prelude::*;

pub struct WorldGenPlugin;

impl Plugin for WorldGenPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system_set(
            AppState::WorldGen,
            ConditionSet::new().with_system(setup_world_gen).into(),
        );
    }
}

fn setup_world_gen(mut cmds: Commands, mut grid: ResMut<Grid>, board_settings: Res<BoardSettings>) {
    for x in 0..board_settings.unit_board.0 {
        for y in 0..board_settings.unit_board.1 {
            let local_coord = Coord::new(x, y);
            let pos = board_settings.offset + grid.to_world(local_coord);
            let cell = cmds
                .spawn_bundle(TransformBundle {
                    local: Transform::from_translation(pos),
                    ..default()
                })
                .insert(Name::new(format!("cell_unit ({}, {})", x, y)))
                .insert(GridSpatial)
                .insert(BattleCell)
                .id();

            grid.maintain_entity(cell, pos.xz());
        }
    }

    let offset = board_settings.offset
        + board_settings.unit_board.1 as f32 * grid.cell_size as f32 * Vec3::Z;
    for x in 0..board_settings.keep_board.0 {
        for y in 0..board_settings.keep_board.1 {
            let local_coord = Coord::new(x, y);
            let pos = offset + grid.to_world(local_coord);
            let cell = cmds
                .spawn_bundle(TransformBundle {
                    local: Transform::from_translation(pos),
                    ..default()
                })
                .insert(Name::new(format!("cell_keep ({}, {})", x, y)))
                .insert(GridSpatial)
                .insert(KeepCell)
                .id();

            grid.maintain_entity(cell, pos.xz());
        }
    }

    cmds.insert_resource(NextState(AppState::InGame));
}
