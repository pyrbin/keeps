mod flowfield;
pub use self::flowfield::*;
use crate::prelude::*;

pub struct PathfindingPlugin;

impl Plugin for PathfindingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FlowFieldPlugin);
        app.add_enter_system_set(
            AppState::InGame,
            ConditionSet::new().with_system(setup_grids).into(),
        );

        #[cfg(feature = "dev")]
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(AppState::InGame)
                .with_system(debug_grid)
                .with_system(debug_flowfield_grid)
                .into(),
        );
    }
}

#[derive(Component, Default, Debug, Deref, DerefMut)]
pub struct DebugColor(Color);

/// TODO: This is a temporary solution to debug the grid.
fn setup_grids(mut cmds: Commands, mut ev_compute: EventWriter<ComputeFlowField>) {
    let flowfield = create_flowfield(
        &mut cmds,
        25,
        25,
        0.5,
        &Transform::from_translation(Vec3::new(0.5 / 2., 0.0, 0.5 / 2.)),
    );

    cmds.entity(flowfield)
        .insert(Name::new("FlowField"))
        .insert(DebugColor(Color::BEIGE));

    log::info!("Flowfield grid spawned {:?}", flowfield);

    ev_compute.send(ComputeFlowField {
        goal: Coord::new(5, 5),
        grid_entity: flowfield,
    });
}

#[cfg(feature = "dev")]
fn debug_grid(
    mut grids: Query<(&Grid, &Transform, &DebugColor)>,
    cells: Query<(&Coord, &Parent)>,
    mut lines: ResMut<DebugLines>,
) {
    for (coord, parent) in cells.iter() {
        let (grid, grid_transform, debug_color) = grids.get_mut(parent.get()).unwrap();
        let translation = grid.coord_to_world(&coord, grid_transform);
        lines.square(translation, grid.cell_size, 0., debug_color.0);
    }
}

#[cfg(feature = "dev")]
fn debug_flowfield_grid(
    grids: Query<(&Grid, &Transform)>,
    cells: Query<(&Coord, &Flow, &Parent)>,
    mut lines: ResMut<DebugLines>,
) {
    for (coord, dir, parent) in cells.iter().filter(|(_, dir, _)| dir.0 != IVec2::ZERO) {
        let (grid, grid_transform) = grids.get(parent.get()).unwrap();
        let start = grid.coord_to_world(&coord, grid_transform);
        let end = start + Vec3::new(dir.x as f32, 0.0, dir.y as f32) * 0.35 * grid.cell_size;
        lines.line_colored(start, end, 0.0, Color::BEIGE);
        lines.square(end, 0.085 * grid.cell_size, 0.0, Color::BEIGE);
    }
}
