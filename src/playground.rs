use crate::prelude::*;

pub struct PlaygroundPlugin;

impl Plugin for PlaygroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system_set(
            AppState::InGame,
            ConditionSet::new().with_system(setup_playground).into(),
        );
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(AppState::InGame)
                .with_system(update_flow_field_goal)
                .with_system(follow_flowfield_system)
                .with_system(update_mouse_hover_coord)
                .with_system(debug_mouse_position)
                .into(),
        );
        app.insert_resource(MousePosition::default());
        app.insert_resource(PaintData::default());
    }
}

#[derive(Resource, Default, Debug, Clone)]
pub struct MousePosition(pub Option<Vec3>);

fn update_mouse_hover_coord(
    mut cmds: Commands,
    windows: Res<Windows>,
    cameras: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = cameras.single();
    let (ray_pos, ray_dir) =
        ray_from_mouse_position(windows.get_primary().unwrap(), camera, camera_transform);
    let (plane_pos, plane_normal) = (Vec3::ZERO, Vec3::Y);
    let point = plane_intersection(ray_pos, ray_dir, plane_pos, plane_normal);

    if point.is_finite() {
        cmds.insert_resource(MousePosition(Some(point)));
    } else {
        cmds.insert_resource(MousePosition(None));
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

fn debug_mouse_position(mut lines: ResMut<DebugLines>, mouse_position: Res<MousePosition>) {
    if let Some(point) = mouse_position.0 {
        // draw a crosshair at the mouse position
        lines.line_colored(
            point + Vec3::new(-0.1, 0.0, 0.0),
            point + Vec3::new(0.1, 0.0, 0.0),
            0.0,
            Color::PURPLE,
        );
        lines.line_colored(
            point + Vec3::new(0.0, 0.0, -0.1),
            point + Vec3::new(0.0, 0.0, 0.1),
            0.0,
            Color::PURPLE,
        );
        lines.circle(point, 0.2, 0.0, Color::PURPLE);
    }
}

#[derive(Component, Default)]
pub struct UnitFlowFieldGrid;

#[derive(Resource, Default)]
pub struct PaintData {
    pub block: bool,
    pub is_painting: bool,
}

fn update_flow_field_goal(
    mouse_pos: Res<MousePosition>,
    buttons: Res<Input<MouseButton>>,
    mut paint_data: ResMut<PaintData>,
    grid_query: Query<(Entity, &Grid, &Transform), With<UnitFlowFieldGrid>>,
    mut cells_query: Query<&mut Cost>,
    mut ev_compute: EventWriter<ComputeFlowField>,
) {
    if let Some(point) = mouse_pos.0 {
        let (entity, grid, grid_transform) = grid_query.single();
        let coord = grid.world_to_coord(&point, &grid_transform);

        if !grid.within_bounds(&coord) {
            return;
        }

        if buttons.just_pressed(MouseButton::Left) {
            ev_compute.send(ComputeFlowField {
                goal: coord,
                grid_entity: entity,
            });
        }

        if buttons.just_pressed(MouseButton::Right) {
            paint_data.is_painting = true;
            let cell_entity = grid.data[&coord];
            if let Some(entity) = cell_entity {
                let cost = cells_query.get(entity).unwrap();
                if *cost == Cost::MAX {
                    paint_data.block = false;
                } else {
                    paint_data.block = true;
                };
            }
        }

        if buttons.pressed(MouseButton::Right) && paint_data.is_painting {
            let cell_entity = grid.data[&coord];
            if let Some(entity) = cell_entity {
                let mut cost = cells_query.get_mut(entity).unwrap();
                cost.0 = *(if paint_data.block {
                    Cost::MAX
                } else {
                    Cost::EMPTY
                });
            }
        }

        if buttons.just_released(MouseButton::Right) {
            paint_data.is_painting = false;
        }
    }
}

fn setup_playground(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut ev_compute: EventWriter<ComputeFlowField>,
) {
    let (width, height) = (25, 25);
    let flowfield = cmds
        .spawn_grid(
            width,
            height,
            0.25,
            &Transform::from_translation(Vec3::new(0.5 / 2., 0.0, 0.5 / 2.)),
            |cell, coord| {
                cell.insert(Cost::EMPTY)
                    .insert(Name::new(format!("Cell {:} {:}", coord.x, coord.y)));
            },
        )
        .insert((
            FlowField::new(width, height),
            UnitFlowFieldGrid,
            Name::new("FlowField"),
            DebugColor(Color::BEIGE),
        ))
        .id();

    log::info!("Flowfield grid spawned {:?}.", flowfield);

    ev_compute.send(ComputeFlowField {
        goal: Coord::new(1, 1),
        grid_entity: flowfield,
    });

    let unit = cmds
        .spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Icosphere {
                    radius: 0.1,
                    subdivisions: 3,
                })),
                material: materials.add(Color::YELLOW.into()),
                transform: Transform::from_xyz(5.0, 0.25, 5.0),
                ..default()
            },
            Unit,
            MovementSpeed(0.01),
            Name::new("Unit"),
            DebugColor(Color::RED),
            MoveDirection::default(),
            Agent { flowfield },
        ))
        .id();

    log::info!("Unit spawned {:?}.", unit);
}

#[cfg_attr(feature = "dev", derive(bevy_inspector_egui::Inspectable))]
#[derive(Component, Debug)]
pub struct Agent {
    pub flowfield: Entity,
}

fn follow_flowfield_system(
    mut agents: Query<(&Agent, &Transform, &mut MoveDirection)>,
    grids: Query<(&FlowField, &Grid, &Transform)>,
) {
    for (agent, transform, mut move_dir) in agents.iter_mut() {
        let (flowfield, grid, grid_transform) = grids
            .get(agent.flowfield)
            .expect("Flow field grid not found");

        let coord = grid.world_to_coord(&transform.translation, &grid_transform);

        if !grid.within_bounds(&coord) {
            continue;
        }

        if let Some(flow) = flowfield.get(&coord) {
            move_dir.0 = Some(Vec3::new(flow.x as f32, 0.0, flow.y as f32));
        }
    }
}
