use crate::prelude::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EnemySpawned>()
            .add_event::<SpawnEnemySpawner>()
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(AppState::InGame)
                    .with_system(enemy_spawn)
                    .with_system(enemy_spawner_spawn)
                    .with_system(enemy_walk)
                    .with_system(test_place_enemy_spawner)
                    .into(),
            )
            .add_system_to_stage(
                CoreStage::PostUpdate,
                enemy_destroy.run_in_state(AppState::InGame),
            );
    }
}

#[derive(Component)]
struct EnemySpawner {
    spawn_rate: Timer,
}

#[derive(Component)]
struct Enemy;

pub struct EnemySpawned(pub Entity);

pub struct SpawnEnemySpawner {
    pub coord: Coord,
}

fn test_place_enemy_spawner(
    mut events: EventWriter<SpawnEnemySpawner>,
    mouse: Res<Input<MouseButton>>,
    battle_cells: Query<With<BattleCell>>,
    spawners: Query<Entity, With<EnemySpawner>>,
    grid_selection: Res<Option<GridSelection>>,
    grid: Res<Grid>,
) {
    if let Some(selection) = &*grid_selection {
        let entities = grid.storage.get(&selection.0);
        if let Some(entities) = entities {
            if mouse.just_pressed(MouseButton::Left) {
                if entities.iter().any(|e| battle_cells.get(*e).is_ok())
                    && entities.iter().all(|e| spawners.get(*e).is_err())
                {
                    events.send(SpawnEnemySpawner { coord: selection.0 });
                }
            }
        }
    }
}

fn enemy_spawner_spawn(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut events: EventReader<SpawnEnemySpawner>,
    grid: Res<Grid>,
) {
    for event in events.iter() {
        let pos: Vec3 = grid.to_world(event.coord);
        cmds.spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 1. })),
            material: materials.add(StandardMaterial {
                base_color: Color::RED,
                alpha_mode: AlphaMode::Blend,
                unlit: true,
                ..default()
            }),
            transform: Transform::from_translation(pos),
            ..default()
        })
        .insert(GridSpatial)
        .insert(EnemySpawner {
            spawn_rate: Timer::from_seconds(1.0, true),
        });
    }
}

fn enemy_spawn(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut spawners: Query<(&mut EnemySpawner, &Transform)>,
    mut events_writer: EventWriter<EnemySpawned>,
    time: Res<Time>,
) {
    for (mut spawner, transform) in spawners.iter_mut() {
        spawner.spawn_rate.tick(time.delta());
        if spawner.spawn_rate.just_finished() {
            let enemy = cmds
                .spawn_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Icosphere {
                        radius: 0.2,
                        subdivisions: 1,
                    })),
                    material: materials.add(StandardMaterial {
                        base_color: Color::WHITE,
                        alpha_mode: AlphaMode::Blend,
                        unlit: true,
                        ..default()
                    }),
                    transform: Transform::from_translation(transform.translation),
                    ..default()
                })
                .insert(Unit)
                .insert(Health::new(10.))
                .insert(MovementSpeed(1.))
                .insert(GridSpatial)
                .insert(Enemy)
                .id();

            events_writer.send(EnemySpawned(enemy));
        }
    }
}

fn enemy_walk(mut enemies: Query<(Entity, &mut Transform, &MovementSpeed)>, time: Res<Time>) {
    for (_, mut transform, speed) in enemies.iter_mut() {
        transform.translation += Vec3::Z * time.delta_seconds() * speed.0;
    }
}

fn enemy_destroy(
    mut cmds: Commands,
    query: Query<(Entity, &Health), (With<Enemy>, Changed<Health>)>,
) {
    for (entity, health) in query.iter() {
        if health.is_dead() {
            cmds.entity(entity).despawn_recursive();
        }
    }
}
