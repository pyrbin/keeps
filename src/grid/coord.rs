use std::{
    fmt::Debug,
    ops::{Add, Mul, Sub},
};

use crate::prelude::*;

pub const NEIGHBORS_8: [Coord; 8] = [
    Coord { x: -1, y: -1 },
    Coord { x: 0, y: -1 },
    Coord { x: 1, y: -1 },
    Coord { x: -1, y: 0 },
    Coord { x: 1, y: 0 },
    Coord { x: -1, y: 1 },
    Coord { x: 0, y: 1 },
    Coord { x: 1, y: 1 },
];

pub const NEIGHBORS: [Coord; 4] = [
    Coord { x: 0, y: -1 },
    Coord { x: -1, y: 0 },
    Coord { x: 1, y: 0 },
    Coord { x: 0, y: 1 },
];

/// A coordinate in a 2D grid.
#[cfg_attr(feature = "dev", derive(bevy_inspector_egui::Inspectable))]
#[derive(Component, Default, Debug, PartialEq, Eq, Hash, Copy, Clone, PartialOrd, Ord)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Coord {
    /// Creates a new coord.
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Returns the 4-directional neighbors of a coordinate.
    pub fn neighbors(self) -> impl Iterator<Item = Coord> {
        NEIGHBORS.iter().map(move |&dir| self + dir)
    }

    /// Returns the 8-directional neighbors of a coordinate.
    pub fn neighbors8(self) -> impl Iterator<Item = Coord> {
        NEIGHBORS_8.iter().map(move |&dir| self + dir)
    }

    /// Returns the distance between two coordinates
    pub fn distance(&self, other: Coord) -> u16 {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as u16
    }
}

impl Add<Coord> for Coord {
    type Output = Coord;

    fn add(self, rhs: Coord) -> Self::Output {
        Self {
            x: self.x.wrapping_add(rhs.x),
            y: self.y.wrapping_add(rhs.y),
        }
    }
}

impl Sub<Coord> for Coord {
    type Output = Coord;

    fn sub(self, rhs: Coord) -> Self::Output {
        Self {
            x: self.x.wrapping_sub(rhs.x),
            y: self.y.wrapping_sub(rhs.y),
        }
    }
}

impl Mul<i32> for Coord {
    type Output = Coord;

    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x.wrapping_mul(rhs),
            y: self.y.wrapping_mul(rhs),
        }
    }
}

impl From<(i32, i32)> for Coord {
    fn from(tuple: (i32, i32)) -> Self {
        Self {
            x: tuple.0,
            y: tuple.1,
        }
    }
}

impl From<(usize, usize)> for Coord {
    fn from(tuple: (usize, usize)) -> Self {
        Self {
            x: tuple.0 as i32,
            y: tuple.1 as i32,
        }
    }
}

impl From<(f32, f32)> for Coord {
    fn from(tuple: (f32, f32)) -> Self {
        Self {
            x: tuple.0 as i32,
            y: tuple.1 as i32,
        }
    }
}

impl From<IVec2> for Coord {
    fn from(vec: IVec2) -> Self {
        Self { x: vec.x, y: vec.y }
    }
}

impl From<Coord> for IVec2 {
    fn from(coord: Coord) -> IVec2 {
        IVec2 {
            x: coord.x,
            y: coord.y,
        }
    }
}

/// Returns the 4-directional neighbors of a coordinate within bounds of given width and height.
pub fn neighbors(
    coord: &'_ Coord,
    width: usize,
    height: usize,
) -> impl Iterator<Item = Coord> + '_ {
    coord
        .neighbors()
        .filter(move |&c| c.x >= 0 && c.y >= 0 && c.x < width as i32 && c.y < height as i32)
}

/// Returns the 8-directional neighbors of a coordinate within bounds of given width and height.
pub fn neighbors8(
    coord: &'_ Coord,
    width: usize,
    height: usize,
) -> impl Iterator<Item = Coord> + '_ {
    coord
        .neighbors8()
        .filter(move |&c| c.x >= 0 && c.y >= 0 && c.x < width as i32 && c.y < height as i32)
}
