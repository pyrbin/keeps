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
        &Transform::from_translation(Vec3::ZERO),
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
    mut grids: Query<(&Grid, &DebugColor)>,
    cells: Query<(&GlobalTransform, &Parent), With<Coord>>,
    mut lines: ResMut<DebugLines>,
) {
    for (transform, parent) in cells.iter() {
        let (grid, debug_color) = grids.get_mut(parent.get()).unwrap();
        lines.square(transform.translation(), grid.cell_size, 0., debug_color.0);
    }
}

#[cfg(feature = "dev")]
fn debug_flowfield_grid(
    grids: Query<&Grid>,
    cells: Query<(&GlobalTransform, &Flow, &Parent)>,
    mut lines: ResMut<DebugLines>,
) {
    for (transform, dir, parent) in cells.iter().filter(|(_, dir, _)| dir.0 != IVec2::ZERO) {
        let grid = grids.get(parent.get()).unwrap();
        let start = transform.translation();
        let end = start + Vec3::new(dir.x as f32, 0.0, dir.y as f32) * 0.35 * grid.cell_size;
        lines.line_colored(start, end, 0.0, Color::BEIGE);
        lines.square(end, 0.085 * grid.cell_size, 0.0, Color::BEIGE);
    }
}
