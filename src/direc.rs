//! Utilities related to directions in the plane.

use crate::axis::Axis;
use std::{
    ops::{Index, IndexMut, Not},
    slice,
};

/// Basic direction in a plane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(
    feature = "impl-serde",
    derive(serde::Serialize, serde::Deserialize)
)]
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
    /// List of all possible directions. Please note that this requires no
    /// heap-allocation and is very cheap.
    pub const ALL: [Direction; 4] =
        [Direction::Up, Direction::Down, Direction::Left, Direction::Right];

    /// Iterator over all directions.
    ///
    /// # Examples
    ///
    /// Note that these examples put the directions in a vector, but if you want
    /// an array of directions, just use [`Direction::ALL`].
    ///
    /// ## Default Order
    /// ```rust
    /// use gardiz::direc::{self, Direction};
    ///
    /// # fn main() {
    /// let direcs: Vec<Direction> = Direction::iter().collect();
    /// assert_eq!(
    ///     vec![Direction::Up, Direction::Down, Direction::Left, Direction::Right],
    ///     direcs
    /// );
    /// # }
    /// ```
    ///
    /// ## Reverse Order
    /// ```rust
    /// use gardiz::direc::{self, Direction};
    ///
    /// # fn main() {
    /// let direcs: Vec<Direction> = Direction::iter().rev().collect();
    /// assert_eq!(
    ///     vec![Direction::Right, Direction::Left, Direction::Down, Direction::Up],
    ///     direcs
    /// );
    /// # }
    /// ```
    pub fn iter() -> Iter {
        Iter { inner: Self::ALL.iter() }
    }

    /// Creates a direction from the given axis in the positive direction (i.e.
    /// `X -> Right` and `Y -> Down`).
    pub fn from_axis_pos(axis: Axis) -> Self {
        match axis {
            Axis::X => Direction::Right,
            Axis::Y => Direction::Down,
        }
    }

    /// Creates a direction from the given axis in the negative direction (i.e.
    /// `X -> Left` and `Y -> Up`).
    pub fn from_axis_neg(axis: Axis) -> Self {
        match axis {
            Axis::X => Direction::Left,
            Axis::Y => Direction::Up,
        }
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

/// Iterator over all "straight" 2D directions. See [`Direction::iter`].
#[derive(Debug, Clone)]
pub struct Iter {
    inner: slice::Iter<'static, Direction>,
}

impl Iterator for Iter {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().copied()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl DoubleEndedIterator for Iter {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back().copied()
    }
}

impl ExactSizeIterator for Iter {}

/// A vector written as a magnitude and a direction.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(
    feature = "impl-serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct DirecVector<T> {
    /// Mangitude, should be numeric.
    pub magnitude: T,
    /// Direction of the vector.
    pub direction: Direction,
}

/// A mapping from all directions to the given data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "impl-serde",
    derive(serde::Serialize, serde::Deserialize)
)]
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
