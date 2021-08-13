//! Utilites related to the axes of a plane.

use std::{fmt, ops::Not, slice};

/// The axes of a plane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(
    feature = "impl-serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum Axis {
    /// The Y axis (changes vertically).
    Y,
    /// The X axis (changes horizontally).
    X,
}

impl Axis {
    /// List of all possible axes. Please note that this requires no
    /// heap-allocation and is very cheap.
    pub const ALL: [Axis; 2] = [Axis::Y, Axis::X];

    /// Iterator over all axes.
    ///
    /// # Examples
    ///
    /// Note that these examples put the axes in a vector, but if you want an
    /// array of axes, just use [`Axis::ALL`].
    ///
    /// ## Default Order
    /// ```rust
    /// use gardiz::axis::{self, Axis};
    ///
    /// # fn main() {
    /// let axes: Vec<Axis> = Axis::iter().collect();
    /// assert_eq!(vec![Axis::Y, Axis::X], axes);
    /// # }
    /// ```
    ///
    /// ## Reverse Order
    /// ```rust
    /// use gardiz::axis::{self, Axis};
    ///
    /// # fn main() {
    /// let axes: Vec<Axis> = Axis::iter().rev().collect();
    /// assert_eq!(vec![Axis::X, Axis::Y], axes);
    /// # }
    /// ```
    pub fn iter() -> Iter {
        Iter { inner: Self::ALL.iter() }
    }
}

impl fmt::Display for Axis {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(match self {
            Axis::Y => "y",
            Axis::X => "x",
        })
    }
}

impl Not for Axis {
    type Output = Axis;

    fn not(self) -> Self::Output {
        match self {
            Axis::Y => Axis::X,
            Axis::X => Axis::Y,
        }
    }
}

/// Iterator over all axes. See [`Axis::iter`].
#[derive(Debug, Clone)]
pub struct Iter {
    inner: slice::Iter<'static, Axis>,
}

impl Iterator for Iter {
    type Item = Axis;

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
