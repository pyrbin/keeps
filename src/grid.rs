use crate::prelude::*;
use std::collections::{HashMap, HashSet};

use crate::AppState;

pub struct GridPlugin {
    pub cell_size: i32,
}

impl GridPlugin {
    pub fn with_cell_size(cell_size: i32) -> Self {
        Self { cell_size }
    }
}

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Grid {
            cell_size: self.cell_size,
            ..Default::default()
        });

        app.add_system_set(
            ConditionSet::new()
                .run_in_state(AppState::InGame)
                .with_system(maintain_grid_cache)
                .into(),
        );
    }
}

#[derive(Component, Debug)]
pub struct GridEntity;

#[derive(Component, Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Coord {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl From<(i32, i32)> for Coord {
    fn from((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }
}

impl From<Coord> for Vec2 {
    fn from(pos: Coord) -> Self {
        Vec2::new(pos.x as f32, pos.y as f32)
    }
}

impl From<&Coord> for Vec2 {
    fn from(pos: &Coord) -> Self {
        Vec2::new(pos.x as f32, pos.y as f32)
    }
}

impl From<Vec2> for Coord {
    fn from(pos: Vec2) -> Self {
        Coord::new(pos.x as i32, pos.y as i32)
    }
}

#[derive(Debug, Default)]
pub struct Grid {
    pub storage: HashMap<Coord, HashSet<Entity>>,
    pub associations: HashMap<Entity, Coord>,
    pub cell_size: i32,
}

impl Grid {
    #[inline]
    pub fn to_world(&self, coord: Coord) -> Vec3 {
        let coord_vec2: Vec2 = coord.into();
        let cell_size = Vec2::splat(self.cell_size as f32 / 2.);
        Vec3::new(coord_vec2.x + cell_size.x, 0., coord_vec2.y + cell_size.y)
    }

    #[inline]
    pub fn to_coord(&self, world: Vec2) -> Coord {
        let offset = (self.cell_size / 2) as f32;
        let xy = world + Vec2::splat(offset);
        xy.floor().into()
    }

    #[inline]
    pub fn maintain_entity(&mut self, entity: Entity, pos: Vec2) {
        let coord = self.to_coord(pos);
        if let Some(old_coord) = self.associations.get(&entity) {
            if *old_coord != coord {
                self.storage.get_mut(old_coord).unwrap().remove(&entity);
                self.storage.entry(coord).or_default().insert(entity);
                self.associations.insert(entity, coord);
            }
        } else {
            self.storage.entry(coord).or_default().insert(entity);
            self.associations.insert(entity, coord);
        }
    }

    pub fn in_bounds(&self, coord: Coord) -> bool {
        self.storage.contains_key(&coord)
    }
}

fn maintain_grid_cache(
    mut grid: ResMut<Grid>,
    query: Query<(Entity, &Transform), (Changed<Transform>, With<GridEntity>)>,
) {
    for (entity, transform) in query.iter() {
        grid.maintain_entity(entity, transform.translation.xz());
    }
}
