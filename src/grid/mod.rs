mod coord;
mod field;



use bevy::ecs::system::EntityCommands;

pub use self::coord::*;
pub use self::field::*;
use crate::prelude::*;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            CoreStage::PostUpdate,
            ConditionSet::new()
                .with_system(maintain_grid_storage_system)
                .into(),
        );
    }
}

#[derive(Bundle, Default)]
pub struct GridBundle {
    pub grid: Grid,
    #[bundle]
    pub transform_bundle: TransformBundle,
}

impl GridBundle {
    pub fn new(width: usize, height: usize, cell_size: f32, transform: &Transform) -> Self {
        Self {
            grid: Grid::new(width, height, cell_size),
            transform_bundle: TransformBundle {
                local: *transform,
                ..Default::default()
            },
        }
    }
}

#[derive(Bundle, Default)]
pub struct CellBundle {
    pub coord: Coord,
}

impl CellBundle {
    pub fn new(coord: Coord) -> Self {
        Self { coord }
    }
}

pub trait GridCommandsExt<'w, 's> {
    fn spawn_grid<'a>(
        &'a mut self,
        width: usize,
        height: usize,
        cell_size: f32,
        transform: &Transform,
        build_fn: fn(&mut EntityCommands<'_, '_, '_>, Coord),
    ) -> EntityCommands<'w, 's, 'a>;
}

impl<'w, 's> GridCommandsExt<'w, 's> for Commands<'w, 's> {
    fn spawn_grid<'a>(
        &'a mut self,
        width: usize,
        height: usize,
        cell_size: f32,
        transform: &Transform,
        child_build_fn: fn(&mut EntityCommands<'_, '_, '_>, Coord),
    ) -> EntityCommands<'w, 's, 'a> {
        let mut grid = self.spawn();
        grid.insert_bundle(GridBundle::new(width, height, cell_size, &transform))
            .with_children(|parent| {
                for coord in iter_coords(width, height) {
                    let mut child = parent.spawn_bundle(CellBundle::new(coord));
                    child_build_fn(&mut child, coord);
                }
            });
        grid
    }
}

/// A 2d grid component with cache storage for entity lookups.
#[derive(Component, Debug, Default, Clone)]
pub struct Grid {
    pub data: Field<Option<Entity>>,
    pub cell_size: f32,
}

impl Grid {
    /// Creates a new grid with the given dimensions.
    pub fn new(width: usize, height: usize, cell_size: f32) -> Self {
        Self {
            data: Field::new(width, height, vec![default(); width * height]),
            cell_size,
            ..default()
        }
    }

    /// Returns the world position of the given coordinate.
    pub fn coord_to_world(&self, coord: &Coord, grid_transform: &Transform) -> Vec3 {
        coord_to_world(coord, self.cell_size, grid_transform)
    }

    /// Returns the local position of the given coordinate.
    pub fn coord_to_local(&self, coord: &Coord) -> Vec3 {
        coord_to_local(coord, self.cell_size)
    }

    /// Returns the coordinate for the given world position.
    pub fn world_to_coord(&self, world_pos: &Vec3, grid_transform: &Transform) -> Coord {
        world_to_coord(world_pos, self.cell_size, grid_transform)
    }

    /// Returns the coordinate for the given local position.
    pub fn local_to_coord(&self, local_pos: &Vec3) -> Coord {
        local_to_coord(local_pos, self.cell_size)
    }

    /// Returns true if the given coordinate is within the grid dimensions.
    pub fn within_bounds(&self, local_coord: &Coord) -> bool {
        self.data.within_bounds(local_coord)
    }

    /// Returns the entity at the given coordinate.
    pub fn get(&self, coord: &Coord) -> Option<Entity> {
        self.data[coord]
    }
}

#[inline]
pub fn coord_to_world(coord: &Coord, cell_size: f32, grid_transform: &Transform) -> Vec3 {
    let local_pos = coord_to_local(coord, cell_size);
    let point = grid_transform.compute_matrix() * local_pos.extend(1.0);
    Vec3::new(point.x, point.y, point.z)
}

#[inline]
pub fn coord_to_local(coord: &Coord, cell_size: f32) -> Vec3 {
    let x = coord.x as f32 * cell_size;
    let y = coord.y as f32 * cell_size;
    Vec3::new(x, 0.0, y)
}

#[inline]
pub fn world_to_coord(world_pos: &Vec3, cell_size: f32, grid_transform: &Transform) -> Coord {
    let local_pos = grid_transform.compute_matrix().inverse() * world_pos.extend(1.0);
    local_to_coord(&local_pos.xyz(), cell_size)
}

#[inline]
pub fn local_to_coord(local_pos: &Vec3, cell_size: f32) -> Coord {
    let x = (local_pos.x / cell_size).round() as i32;
    let y = (local_pos.z / cell_size).round() as i32;
    Coord::new(x, y)
}

fn maintain_grid_storage_system(
    mut grids: Query<&mut Grid>,
    query: Query<(Entity, &Parent, &Coord), Changed<Coord>>,
) {
    for (entity, parent, coord) in query.iter() {
        if let Ok(mut grid) = grids.get_mut(parent.get()) {
            grid.data[&coord] = Some(entity);
        }
    }
}
