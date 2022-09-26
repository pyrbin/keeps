use crate::prelude::*;

use super::{Coord, Field};
use std::collections::HashMap;

#[derive(Component, Debug, Default)]
pub struct Grid {
    pub storage: Field<Option<Entity>>,
    pub backward: HashMap<Entity, Coord>,
    pub cell_size: f32,
}

impl Grid {
    pub fn new(width: usize, height: usize, cell_size: f32) -> Self {
        Self {
            storage: Field::new(width, height, vec![default(); width * height]),
            cell_size,
            ..default()
        }
    }

    pub fn coord_to_world(&self, coord: &Coord, grid_transform: &Transform) -> Vec3 {
        coord_to_world(coord, self.cell_size, grid_transform)
    }

    pub fn coord_to_local(&self, coord: &Coord) -> Vec3 {
        coord_to_local(coord, self.cell_size)
    }

    pub fn world_to_coord(&self, world_pos: &Vec3, grid_transform: &Transform) -> Coord {
        world_to_coord(world_pos, self.cell_size, grid_transform)
    }

    pub fn local_to_coord(&self, local_pos: &Vec3) -> Coord {
        local_to_coord(local_pos, self.cell_size)
    }

    pub fn in_bounds(&self, local_coord: &Coord) -> bool {
        local_coord.x >= 0
            && local_coord.y >= 0
            && local_coord.x < self.storage.width() as i32
            && local_coord.y < self.storage.height() as i32
    }

    pub fn maintain_entity(&mut self, entity: Entity, local_pos: &Vec3) {
        let coord = self.local_to_coord(local_pos);
        if let Some(old_coord) = self.backward.get(&entity) {
            if *old_coord != coord {
                self.storage[old_coord] = None;
                self.storage[&coord] = Some(entity);
                self.backward.insert(entity, coord);
            }
        } else {
            self.storage[&coord] = Some(entity);
            self.backward.insert(entity, coord);
        }
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

pub fn maintain_grid_cache_system(
    mut grids: Query<&mut Grid>,
    mut query: Query<(Entity, &Transform, &Parent, &mut Coord), Changed<Transform>>,
) {
    for (entity, transform, parent, mut coord) in query.iter_mut() {
        if let Ok(mut grid) = grids.get_mut(parent.get()) {
            let local_pos = transform.translation;
            grid.maintain_entity(entity, &local_pos);
            let updated_coord = grid.local_to_coord(&local_pos);
            coord.x = updated_coord.x;
            coord.y = updated_coord.y;
        }
    }
}

pub fn coord_propagate_system(
    mut grids: Query<&mut Grid>,
    mut query: Query<(&mut Transform, &Parent, &Coord), Changed<Coord>>,
) {
    for (mut transform, parent, coord) in query.iter_mut() {
        if let Ok(grid) = grids.get_mut(parent.get()) {
            transform.translation = grid.coord_to_local(coord);
        }
    }
}
