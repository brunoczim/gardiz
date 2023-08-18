//! Utilities related to directions in the plane.

use crate::axis::Axis;
use std::{
    ops::{Index, IndexMut, Mul, Not},
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
///
/// # Warning
/// Zero vectors of _different_ directions **are not** equal to each other. As
/// well as _negative_ vector **isn't** equal to positive in contrary direction.
///
/// ## Example
/// ```
/// # use gardiz::direc::{DirecVector, Direction};
/// let v_zero_left = DirecVector{magnitude: 0, direction: Direction::Left};
/// let v_zero_right = DirecVector{magnitude: 0, direction: Direction::Right};
/// assert_ne!(v_zero_left, v_zero_right);
///
/// let v_zero_up = DirecVector{magnitude: 1, direction: Direction::Up};
/// let v_zero_up_neg = DirecVector{magnitude: -1, direction: Direction::Down};
/// assert_ne!(v_zero_up, v_zero_up_neg);
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(
    feature = "impl-serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct DirecVector<T> {
    /// Magnitude, should be numeric.
    pub magnitude: T,
    /// Direction of the vector.
    pub direction: Direction,
}

/// Scalar multiplication for a vector.
impl<T> Mul<&T> for &DirecVector<T>
where
    T: Mul<Output = T> + Clone,
{
    type Output = DirecVector<T>;
    fn mul(self, rhs: &T) -> Self::Output {
        DirecVector { magnitude: self.magnitude.clone() * rhs.clone(), ..*self }
    }
}

impl<T> Mul<T> for &DirecVector<T>
// https://github.com/brunoczim/gardiz/pull/2#discussion_r1281192652
where
    T: Mul<Output = T> + Clone,
{
    type Output = DirecVector<T>;

    fn mul(self, rhs: T) -> Self::Output {
        self * &rhs
    }
}

macro_rules! direc_vector_mul_impl {
    ($($T: ty),* $(,)*) => {
        $(
            impl Mul<&DirecVector<$T>> for $T
                // where T: Mul<Output=$T> + Clone
            {
                type Output = DirecVector<$T>;
                fn mul(self, rhs: &DirecVector<$T>) -> Self::Output {
                    rhs * self
                }
            }
        )*
    };
}
direc_vector_mul_impl!(
    u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64
);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direcvec_mul_zero() {
        let t = DirecVector { magnitude: -3, direction: Direction::Down };
        let t = &t * 0;
        assert_eq!(t, DirecVector { magnitude: 0, direction: Direction::Down });
    }

    #[test]
    fn direcvec_mul_i32() {
        let t = &DirecVector { magnitude: -3, direction: Direction::Up } * -2;
        assert_eq!(t, DirecVector { magnitude: 6, direction: Direction::Up });
    }

    #[test]
    fn zero_mul_direcvec() {
        let t = 0 * &DirecVector { magnitude: -3, direction: Direction::Up };
        assert_eq!(t, DirecVector { magnitude: 0, direction: Direction::Up });
    }
}
