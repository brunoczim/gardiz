use crate::axis::Axis;
use std::ops::{Index, IndexMut, Not};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub const ALL: [Direction; 4] =
        [Direction::Up, Direction::Down, Direction::Left, Direction::Right];

    pub fn iter() -> impl DoubleEndedIterator<Item = Direction> {
        Self::ALL.iter().copied()
    }

    pub fn axis(self) -> Axis {
        match self {
            Direction::Up | Direction::Down => Axis::Y,
            Direction::Left | Direction::Right => Axis::X,
        }
    }

    pub fn rotate_clockwise(self) -> Self {
        match self {
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
        }
    }

    pub fn rotate_countercw(self) -> Self {
        match self {
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
            Direction::Up => Direction::Left,
        }
    }
}

impl Not for Direction {
    type Output = Direction;

    fn not(self) -> Self::Output {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DirecVector<T> {
    pub magnitude: T,
    pub direction: Direction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DirecMap<T> {
    pub up: T,
    pub left: T,
    pub down: T,
    pub right: T,
}

impl<T> DirecMap<T> {
    pub fn from_direcs<F>(mut map: F) -> Self
    where
        F: FnMut(Direction) -> T,
    {
        Self {
            up: map(Direction::Up),
            left: map(Direction::Left),
            down: map(Direction::Down),
            right: map(Direction::Right),
        }
    }
}

impl<T> Index<Direction> for DirecMap<T> {
    type Output = T;

    fn index(&self, index: Direction) -> &Self::Output {
        match index {
            Direction::Up => &self.up,
            Direction::Left => &self.left,
            Direction::Down => &self.down,
            Direction::Right => &self.right,
        }
    }
}

impl<T> IndexMut<Direction> for DirecMap<T> {
    fn index_mut(&mut self, index: Direction) -> &mut Self::Output {
        match index {
            Direction::Up => &mut self.up,
            Direction::Left => &mut self.left,
            Direction::Down => &mut self.down,
            Direction::Right => &mut self.right,
        }
    }
}
