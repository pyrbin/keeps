use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

use crate::prelude::*;

pub struct FlowFieldPlugin;

impl Plugin for FlowFieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ComputeFlowField>();
        app.add_system_set_to_stage(
            CoreStage::PreUpdate,
            ConditionSet::new()
                .with_system(compute_flowfield_system)
                .into(),
        );
    }
}

/// The cost of a tile when calculating a flow field.
#[cfg_attr(feature = "dev", derive(bevy_inspector_egui::Inspectable))]
#[derive(Component, Debug, Deref, DerefMut, PartialEq, Eq, Clone, Copy)]
pub struct Cost(pub u8);

impl Cost {
    pub const MAX: Self = Self(u8::MAX);
    pub const EMPTY: Self = Self(0);
}

impl Default for Cost {
    fn default() -> Self {
        Self::EMPTY
    }
}

/// The flow direction of a tile in a flow field.
#[cfg_attr(feature = "dev", derive(bevy_inspector_egui::Inspectable))]
#[derive(Component, Default, Debug, Clone, Deref, DerefMut)]
pub struct Flow(pub IVec2);

/// A flow field component. Stores the goal of the flow field & the time it was last updated.
#[cfg_attr(feature = "dev", derive(bevy_inspector_egui::Inspectable))]
#[derive(Component, Default, Debug)]
pub struct FlowField {
    pub goal: Option<Coord>,
    pub last_updated_tick: f64,
}

/// Compute flow field event for a given goal.
#[derive(Debug, Clone)]
pub struct ComputeFlowField {
    pub goal: Coord,
    pub grid_entity: Entity,
}

/// Consumes [ComputeFlowField] events and computes & updates the flow field for the given goal.
fn compute_flowfield_system(
    mut cmds: Commands,
    mut ev_compute: EventReader<ComputeFlowField>,
    time: Res<Time>,
    grids: Query<(Entity, &Grid), With<FlowField>>,
    cells: Query<(Entity, &Cost, &Flow, &Parent, &Coord)>,
) {
    for ((grid_entity, grid), ev) in
        ev_compute
            .iter()
            .filter_map(|ev| match grids.get(ev.grid_entity) {
                Ok(grid) => Some((grid, ev)),
                Err(_) => None,
            })
    {
        use std::time::Instant;

        let now = Instant::now();
        let goal = ev.goal;

        log::info!("Compute for goal: {:?}", goal);

        let mut integration = Field::new(
            grid.storage.width(),
            grid.storage.height(),
            vec![None; grid.storage.width() * grid.storage.height()],
        );

        // Compute the integration field.
        let mut queue = BinaryHeap::new();
        let mut closed = HashSet::new();

        queue.push(Reverse((0_i32, goal)));

        while let Some(Reverse((cost, coord))) = queue.pop() {
            if closed.contains(&coord) {
                continue;
            }

            closed.insert(coord);

            integration[&coord] = Some(cost);

            for neighbor in grid.storage.neighbors8(&coord) {
                if closed.contains(&neighbor) {
                    continue;
                }

                let neighbor_entity = match grid.storage[&neighbor] {
                    Some(entity) => entity,
                    None => continue,
                };

                let neighbor_cost = match cells.get(neighbor_entity) {
                    Ok((_, cost, _, _, _)) => cost,
                    Err(_) => continue,
                };

                if *neighbor_cost == Cost::MAX {
                    continue;
                }

                let cost = cost + neighbor_cost.0 as i32 + neighbor.distance(goal) as i32;
                queue.push(Reverse((cost, neighbor)));
            }
        }

        // Compute the flow field from the integration field.
        for y in 0..grid.storage.height() {
            for x in 0..grid.storage.width() {
                let coord = Coord::new(x as i32, y as i32);

                if integration[&coord].is_none() {
                    continue;
                }

                let entity = match grid.storage[&coord] {
                    Some(entity) => entity,
                    None => continue,
                };

                let mut min_cost = i32::MAX;
                let mut min_coord = Coord::default();

                if coord == goal {
                    cmds.entity(entity).insert(Flow(min_coord.into()));
                    continue;
                }

                for neighbor in integration.neighbors8(&coord) {
                    if let Some(cost) = integration[&neighbor] {
                        if cost < min_cost {
                            min_cost = cost;
                            min_coord = neighbor - coord;
                        }
                    }
                }

                cmds.entity(entity).insert(Flow(min_coord.into()));
            }
        }

        cmds.entity(grid_entity).insert(FlowField {
            goal: Some(goal),
            last_updated_tick: time.seconds_since_startup(),
        });

        log::info!("Compute took: {:.2?}", now.elapsed());
    }
}

// Create a flow field
pub fn create_flowfield(
    cmds: &mut Commands,
    width: usize,
    height: usize,
    cell_size: f32,
    transform: &Transform,
) -> Entity {
    cmds.spawn_bundle(GridBundle::new(width, height, cell_size, transform))
        .with_children(|parent| {
            for coord in iter_coords(width, height) {
                parent
                    .spawn_bundle(CellBundle::new(coord))
                    .insert(Cost::default())
                    .insert(Flow::default());
            }
        })
        .insert(FlowField::default())
        .id()
}
