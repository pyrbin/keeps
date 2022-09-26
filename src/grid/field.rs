use super::coord::{neighbors, neighbors8, Coord};
use std::ops::{Index, IndexMut};

#[derive(Clone, Debug, Default)]
pub struct Field<T: Default> {
    pub data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T: Default> Field<T> {
    pub fn new(width: usize, height: usize, data: Vec<T>) -> Self {
        Self {
            width,
            height,
            data,
        }
    }

    pub fn to_1d(&self, coord: &Coord) -> usize {
        to_1d(coord, self.width)
    }

    pub fn to_coord(&self, index: usize) -> Coord {
        to_coord(index, self.width)
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn neighbors<'a>(&'a self, coord: &'a Coord) -> impl Iterator<Item = Coord> + 'a {
        neighbors(coord, self.width, self.height)
    }

    pub fn neighbors8<'a>(&'a self, coord: &'a Coord) -> impl Iterator<Item = Coord> + 'a {
        neighbors8(coord, self.width, self.height)
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data.iter_mut()
    }

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

#[inline]
pub fn to_1d(coord: &Coord, width: usize) -> usize {
    coord.y as usize * width + coord.x as usize
}

#[inline]
pub fn to_coord(i: usize, width: usize) -> Coord {
    let x = i % width;
    let y = i / width;
    Coord::new(x as i32, y as i32)
}