use crate::prelude::*;
pub struct BoardPlugin {
    pub settings: BoardSettings,
}

impl BoardPlugin {
    pub fn with_settings(settings: BoardSettings) -> Self {
        Self { settings }
    }
}

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.settings.clone());
        app.insert_resource(MouseHoverCoord::default());
        app.add_system_set(SystemSet::on_enter(AppState::WorldGen).with_system(generate_boards));
        app.add_system_set(
            SystemSet::on_update(AppState::WorldGen).with_system(exit_generation_state),
        );

        app.add_system_set(
            SystemSet::on_update(AppState::InGame).with_system(update_mouse_hover_coord),
        );

        #[cfg(debug_assertions)]
        app.add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(debug_mouse_hover_coord)
                .with_system(debug_unit_board_cells)
                .with_system(debug_keep_board_cells),
        );
    }
}

#[derive(Component, Debug)]
pub struct KeepCell;

#[derive(Component, Debug)]
pub struct UnitCell;

#[derive(Default, Debug, Clone)]
pub struct MouseHoverCoord {
    pub coord: GridCoord,
    pub mouse_pos: Vec3,
}

#[derive(Debug, Clone)]
pub struct BoardSettings {
    pub unit_board: (i32, i32),
    pub keep_board: (i32, i32),
    pub offset: Vec3,
}

impl BoardSettings {
    pub fn unit_board_offset_end(&self, grid: &Grid) -> Vec3 {
        self.offset + self.unit_board.1 as f32 * grid.cell_size as f32 * Vec3::Z
    }
}

fn generate_boards(mut cmd: Commands, mut grid: ResMut<Grid>, board_settings: Res<BoardSettings>) {
    for x in 0..board_settings.unit_board.0 {
        for y in 0..board_settings.unit_board.1 {
            let local_coord = GridCoord::new(x, y);
            let pos = board_settings.offset + grid.to_world(local_coord);
            let cell = cmd
                .spawn_bundle(TransformBundle {
                    local: Transform::from_translation(pos),
                    ..default()
                })
                .insert(Cell)
                .insert(UnitCell)
                .insert(Name::new(format!("cell_unit ({}, {})", x, y)))
                .id();

            grid.update_entity(cell, pos.xz());
        }
    }

    let offset = board_settings.unit_board_offset_end(&grid);
    for x in 0..board_settings.keep_board.0 {
        for y in 0..board_settings.keep_board.1 {
            let local_coord = GridCoord::new(x, y);
            let pos = offset + grid.to_world(local_coord);
            let cell = cmd
                .spawn_bundle(TransformBundle {
                    local: Transform::from_translation(pos),
                    ..default()
                })
                .insert(Cell)
                .insert(KeepCell)
                .insert(Name::new(format!("cell_keep ({}, {})", x, y)))
                .id();

            grid.update_entity(cell, pos.xz());
        }
    }
}

fn exit_generation_state(mut app_state: ResMut<State<AppState>>) {
    app_state.set(AppState::InGame).unwrap();
}

fn update_mouse_hover_coord(
    mut cmds: Commands,
    windows: Res<Windows>,
    cameras: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    board_settings: Res<BoardSettings>,
    grid: Res<Grid>,
    mut lines: ResMut<DebugLines>,
) {
    let (camera, camera_transform) = cameras.single();
    let (ray_pos, ray_dir) =
        ray_from_mouse_position(windows.get_primary().unwrap(), camera, camera_transform);
    let (plane_pos, plane_normal) = (board_settings.offset, Vec3::Y);
    let point = plane_intersection(ray_pos, ray_dir, plane_pos, plane_normal);

    if point.is_finite() {
        let coord = grid.to_coord(point.xz());

        #[cfg(debug_assertions)]
        {
            lines.circle(point, Quat::IDENTITY, 0.1, 0.0, Color::YELLOW);
        }

        cmds.insert_resource(MouseHoverCoord {
            coord,
            mouse_pos: point,
        });
    }
}

/// Calculates origin and direction of a ray from cursor to world space.
pub fn ray_from_mouse_position(
    window: &Window,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> (Vec3, Vec3) {
    let mouse_position = window.cursor_position().unwrap_or(Vec2::new(0.0, 0.0));

    let x = 2.0 * (mouse_position.x / window.width() as f32) - 1.0;
    let y = 2.0 * (mouse_position.y / window.height() as f32) - 1.0;

    let camera_inverse_matrix =
        camera_transform.compute_matrix() * camera.projection_matrix().inverse();
    let near = camera_inverse_matrix * Vec3::new(x, y, -1.0).extend(1.0);
    let far = camera_inverse_matrix * Vec3::new(x, y, 1.0).extend(1.0);

    let near = near.truncate() / near.w;
    let far = far.truncate() / far.w;
    let dir: Vec3 = far - near;
    (near, dir)
}

fn debug_mouse_hover_coord(
    mut lines: ResMut<DebugLines>,
    mouse_grid_coord: Res<MouseHoverCoord>,
    grid: Res<Grid>,
) {
    let pos = grid.to_world(mouse_grid_coord.coord);
    let size = grid.cell_size as f32 * 0.7;

    let color = if grid.in_bounds(mouse_grid_coord.coord) {
        Color::GREEN
    } else {
        Color::RED
    };

    lines.square(pos, size, 0.0, color);
}

fn debug_unit_board_cells(
    query: Query<&Transform, With<UnitCell>>,
    grid: Res<Grid>,
    mut lines: ResMut<DebugLines>,
) {
    for transform in query.iter() {
        let pos = transform.translation;
        let size = grid.cell_size as f32;
        lines.square(pos, size, 0.0, Color::CYAN);
    }
}

fn debug_keep_board_cells(
    query: Query<&Transform, With<KeepCell>>,
    grid: Res<Grid>,
    mut lines: ResMut<DebugLines>,
) {
    for transform in query.iter() {
        let pos = transform.translation;
        let size = grid.cell_size as f32;
        lines.square(pos, size, 0.0, Color::GOLD);
    }
}
