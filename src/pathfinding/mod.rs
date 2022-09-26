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
    const WIDTH: usize = 20;
    const HEIGHT: usize = 20;
    const CELL_SIZE: f32 = 1.0;
    const FLOW_FIELD_CELL_MODIFIER: f32 = 0.5;

    create_grid(&mut cmds, WIDTH, HEIGHT, CELL_SIZE, Color::PURPLE);

    let flowfield = create_grid(
        &mut cmds,
        (WIDTH as f32 / FLOW_FIELD_CELL_MODIFIER).round() as usize,
        (HEIGHT as f32 / FLOW_FIELD_CELL_MODIFIER).round() as usize,
        CELL_SIZE * FLOW_FIELD_CELL_MODIFIER,
        Color::GRAY,
    );

    cmds.entity(flowfield).insert(FlowField::default());

    ev_compute.send(ComputeFlowField {
        goal: Coord::new(0, 0),
        grid_entity: flowfield,
    });
}

fn create_grid(
    cmds: &mut Commands,
    width: usize,
    height: usize,
    cell_size: f32,
    color: Color,
) -> Entity {
    let half_cell_size = cell_size / 2.0;
    let grid_pos = Vec3::ZERO + Vec3::new(half_cell_size, 0.0, half_cell_size);
    let grid_entity = cmds
        .spawn_bundle(TransformBundle {
            local: Transform::from_translation(grid_pos),
            ..default()
        })
        .insert(Name::new(format!("Grid ({}, {})", width, height)))
        .insert(DebugColor(color))
        .insert(Grid::new(width, height, cell_size))
        .id();

    let mut cells: Vec<Entity> = vec![];
    for x in 0..width {
        for y in 0..height {
            let local_coord = Coord::new(x as i32, y as i32);
            let cost = if x < 1 || x % 4 != 0 {
                if y > 0 && x == 0 {
                    Cost(100)
                } else {
                    Cost::EMPTY
                }
            } else if y % 4 == 0 {
                Cost::EMPTY
            } else {
                Cost::MAX
            };
            let cell = cmds
                .spawn_bundle(TransformBundle {
                    local: Transform::from_translation(coord_to_local(&local_coord, cell_size)),
                    ..default()
                })
                .insert(Name::new(format!("Coord ({}, {})", x, y)))
                .insert(local_coord)
                .insert(cost)
                .insert(FlowDirection::default())
                .id();

            cells.push(cell);
        }
    }

    cmds.entity(grid_entity).push_children(cells.as_slice());
    grid_entity
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
    cells: Query<(&GlobalTransform, &FlowDirection, &Parent)>,
    mut lines: ResMut<DebugLines>,
) {
    for (transform, dir, parent) in cells.iter().filter(|(_, dir, _)| dir.0 != IVec2::ZERO) {
        let grid = grids.get(parent.get()).unwrap();
        let start = transform.translation();
        let end = start + Vec3::new(dir.x as f32, 0.0, dir.y as f32) * 0.35 * grid.cell_size;
        lines.line_colored(start, end, 0.0, Color::BEIGE);
        lines.square(end, 0.1 * grid.cell_size, 0.0, Color::BEIGE);
    }
}
