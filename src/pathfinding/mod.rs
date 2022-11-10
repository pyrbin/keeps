mod flowfield;
pub use self::flowfield::*;
use crate::prelude::*;

pub struct PathfindingPlugin;

impl Plugin for PathfindingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FlowFieldPlugin);
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

#[cfg(feature = "dev")]
fn debug_grid(
    mut grids: Query<(&Grid, &Transform, Option<&DebugColor>)>,
    cells: Query<(&Coord, &Parent, &Cost)>,
    mut lines: ResMut<DebugLines>,
) {
    for (coord, parent, cost) in cells.iter() {
        let (grid, grid_transform, debug_color) = grids.get_mut(parent.get()).unwrap();

        let color = if *cost == Cost::MAX {
            Color::RED
        } else {
            match debug_color {
                Some(DebugColor(color)) => *color,
                None => Color::WHITE,
            }
        };

        let translation = grid.coord_to_world(&coord, grid_transform);
        lines.square(translation, grid.cell_size, 0., color);
    }
}

#[cfg(feature = "dev")]
fn debug_flowfield_grid(
    grids: Query<(&Grid, &Transform, &FlowField)>,
    cells: Query<(&Coord, &Parent)>,
    mut lines: ResMut<DebugLines>,
) {
    for (coord, parent) in cells.iter() {
        let (grid, grid_transform, flowfield) = grids.get(parent.get()).unwrap();
        let start = grid.coord_to_world(&coord, grid_transform);
        if let Some(dir) = flowfield.get(&coord) {
            let end = start + Vec3::new(dir.x as f32, 0.0, dir.y as f32) * 0.35 * grid.cell_size;
            lines.line_colored(start, end, 0.0, Color::BEIGE);
            lines.square(end, 0.085 * grid.cell_size, 0.0, Color::BEIGE);
        }
    }
}
