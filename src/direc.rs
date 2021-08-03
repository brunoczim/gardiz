//! Utilities related to directions in the plane.

use crate::axis::Axis;
use std::ops::{Index, IndexMut, Not};

/// Basic direction in a plane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    /// Direction up (towards negative Y).
    Up,
    /// Direction down (towards positive Y).
    Down,
    /// Direction left (towards negative X).
    Left,
    /// Direction right (towards positive X).
    Right,
}

impl Direction {
    /// List of all possible directions.
    pub const ALL: [Direction; 4] =
        [Direction::Up, Direction::Down, Direction::Left, Direction::Right];

    /// Iterator over all possible directions.
    pub fn iter() -> impl DoubleEndedIterator<Item = Direction> {
        Self::ALL.iter().copied()
    }

    /// Axis on which the direciton varies.
    pub fn axis(self) -> Axis {
        match self {
            Direction::Up | Direction::Down => Axis::Y,
            Direction::Left | Direction::Right => Axis::X,
        }
    }

    /// Rotates the direction clockwise.
    pub fn rotate_clockwise(self) -> Self {
        match self {
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
        }
    }

    /// Rotates the direction counter-clockwise.
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

/// A vector written as a magnitude and a direction.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DirecVector<T> {
    /// Mangitude, should be numeric.
    pub magnitude: T,
    /// Direction of the vector.
    pub direction: Direction,
}

/// A mapping from all directions to the given data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DirecMap<T> {
    /// Data associated with `Direction::Up`.
    pub up: T,
    /// Data associated with `Direction::Left`.
    pub left: T,
    /// Data associated with `Direction::Down`.
    pub down: T,
    /// Data associated with `Direction::Right`.
    pub right: T,
}

impl<T> DirecMap<T> {
    /// Creates a mapping from a function.
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
