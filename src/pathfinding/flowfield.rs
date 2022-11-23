use std::{cmp::Reverse, collections::BinaryHeap};

use crate::prelude::*;

pub struct FlowFieldPlugin;

impl Plugin for FlowFieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ComputeFlowField>();
        app.add_system_set_to_stage(
            CoreStage::PreUpdate,
            ConditionSet::new().with_system(compute_flowfield).into(),
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

/// A flow field component. Stores the goal of the flow field & the time it was last updated.
#[derive(Component, Default, Debug)]
pub struct FlowField {
    pub goal: Option<Coord>,
    pub flow: Field<Option<Vec2>>,
    pub integration: Field<Option<i32>>,
}

impl FlowField {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            goal: None,
            flow: Field::new(width, height, vec![None; width * height]),
            integration: Field::new(width, height, vec![None; width * height]),
        }
    }

    pub fn get(&self, coord: &Coord) -> Option<Vec2> {
        if self.flow.within_bounds(coord) {
            self.flow[coord]
        } else {
            None
        }
    }

    pub fn set(&mut self, coord: &Coord, value: Option<Vec2>) {
        if self.flow.within_bounds(coord) {
            self.flow[coord] = value;
        }
    }

    pub fn clear(&mut self) {
        self.flow.clear();
        self.integration.clear();
    }
}

/// Compute flow field event for a given goal.
#[derive(Debug, Clone)]
pub struct ComputeFlowField {
    pub goal: Coord,
    pub grid_entity: Entity,
}

/// Consumes [ComputeFlowField] events and computes & updates the flow field for the given goal.
fn compute_flowfield(
    mut ev_compute: EventReader<ComputeFlowField>,
    mut grids: Query<(&Grid, &mut FlowField)>,
    costs: Query<&Cost>,
) {
    for ev in ev_compute.iter() {
        use std::time::Instant;

        let now = Instant::now();
        let goal = ev.goal;

        let (grid, mut flowfield) = grids
            .get_mut(ev.grid_entity)
            .expect("Grid entity not found");

        log::info!(
            "Compute flowfield {:?} for goal: {:?}.",
            ev.grid_entity,
            goal
        );

        if !grid.within_bounds(&ev.goal) {
            log::error!(
                "Goal {:?} is not within bounds of grid, aborting ...",
                ev.goal
            );
            continue;
        }

        // Set the goal of the flow field.
        flowfield.goal = Some(goal);

        // Reset the integration field
        flowfield.clear();

        // Compute the integration field.
        let mut queue = BinaryHeap::new();

        const ZERO_COST: i32 = 0_i32;
        const MAX_COST: i32 = i32::MAX;

        // Add the goal to the queue with a cost of 0.
        flowfield.integration[&goal] = Some(ZERO_COST);
        queue.push(Reverse((ZERO_COST, goal)));

        while let Some(Reverse((cost, coord))) = queue.pop() {
            for neighbor in grid.data.neighbors8(&coord) {
                let neighbor_entity = match grid.data[&neighbor] {
                    Some(entity) => entity,
                    None => continue,
                };

                let neighbor_cost = match costs.get(neighbor_entity) {
                    Ok(cost) => cost,
                    Err(_) => continue,
                };

                let cost = cost + neighbor_cost.0 as i32 + neighbor.distance(goal) as i32;

                if cost < flowfield.integration[&neighbor].unwrap_or(MAX_COST) {
                    flowfield.integration[&neighbor] = Some(cost);
                    queue.push(Reverse((cost, neighbor)));
                }
            }
        }

        // Compute the flow field from the integration field.

        for y in 0..grid.data.size.width {
            for x in 0..grid.data.size.height {
                let coord = Coord::new(x as i32, y as i32);
                if flowfield.integration[&coord].is_none() {
                    continue;
                }

                let mut min_cost = i32::MAX;
                let mut min_dir = Coord::default();

                if coord == goal {
                    flowfield.set(&coord, Some(min_dir.into()));
                    continue;
                }

                for neighbor in flowfield.integration.neighbors8(&coord) {
                    if let Some(cost) = flowfield.integration[&neighbor] {
                        if cost < min_cost {
                            min_cost = cost;
                            min_dir = neighbor - coord;
                        }
                    }
                }

                flowfield.set(&coord, Some(min_dir.into()));
            }
        }

        log::info!("Compute took: {:.2?}.", now.elapsed());
    }
}
