use super::coord::{neighbors, neighbors8, Coord};
use std::ops::{Index, IndexMut};

/// A 2D field of values.
#[derive(Clone, Debug, Default)]
pub struct Field<T: Default> {
    pub data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T: Default> Field<T> {
    /// Creates a new field.
    pub fn new(width: usize, height: usize, data: Vec<T>) -> Self {
        Self {
            width,
            height,
            data,
        }
    }

    /// Returns the 1-dimensional index of a coordinate.
    pub fn to_1d(&self, coord: &Coord) -> usize {
        to_1d(coord, self.width)
    }

    /// Returns the 2-dimensional coordinate of a 1-dimensional index.
    pub fn to_coord(&self, index: usize) -> Coord {
        to_coord(index, self.width)
    }

    /// Returns the width of the field.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the height of the field.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Returns the 4-directional neighbors of a coordinate.
    pub fn neighbors<'a>(&'a self, coord: &'a Coord) -> impl Iterator<Item = Coord> + 'a {
        neighbors(coord, self.width, self.height)
    }

    /// Returns the 8-directional neighbors of a coordinate.
    pub fn neighbors8<'a>(&'a self, coord: &'a Coord) -> impl Iterator<Item = Coord> + 'a {
        neighbors8(coord, self.width, self.height)
    }

    /// Iterates over the items of the field.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    /// Iterates over the items of the field.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data.iter_mut()
    }

    /// Iterates over the coordinates of the field.
    pub fn iter_coords(&self) -> impl Iterator<Item = Coord> + '_ {
        iter_coords(self.width, self.height)
    }

    /// Resize the field.
    pub fn resize(&mut self, width: usize, height: usize) {
        self.data.resize_with(width * height, || T::default());
        self.width = width;
        self.height = height;
    }
}

impl<T: Default> Index<&Coord> for Field<T> {
    type Output = T;
    fn index<'a>(&'a self, coord: &Coord) -> &'a T {
        &self.data[self.to_1d(coord)]
    }
}

impl<T: Default> IndexMut<&Coord> for Field<T> {
    fn index_mut<'a>(&'a mut self, coord: &Coord) -> &'a mut T {
        let index = self.to_1d(coord);
        &mut self.data[index]
    }
}

/// Returns the 1-dimensional index of a coordinate.
#[inline]
pub fn to_1d(coord: &Coord, width: usize) -> usize {
    coord.y as usize * width + coord.x as usize
}

/// Returns the 2-dimensional coordinate of a 1-dimensional index.
#[inline]
pub fn to_coord(i: usize, width: usize) -> Coord {
    let x = i % width;
    let y = i / width;
    Coord::new(x as i32, y as i32)
}

/// Iterates over the coordinates of a field with the given width and height.
#[inline]
pub fn iter_coords(width: usize, height: usize) -> impl Iterator<Item = Coord> {
    (0..width * height).map(move |i| to_coord(i, width))
}
