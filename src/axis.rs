//! Utilites related to the axes of a plane.

use std::{fmt, ops::Not, slice};

/// The axes of a plane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Axis {
    /// The Y axis (changes vertically).
    Y,
    /// The X axis (changes horizontally).
    X,
}

impl Axis {
    /// List of all possible axes.
    pub const ALL: [Axis; 2] = [Axis::Y, Axis::X];

    /// Iterator over all axes.
    pub fn iter() -> AxisIter {
        AxisIter { inner: Self::ALL.iter() }
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
pub struct AxisIter {
    inner: slice::Iter<'static, Axis>,
}

impl Iterator for AxisIter {
    type Item = Axis;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().copied()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl DoubleEndedIterator for AxisIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back().copied()
    }
}

impl ExactSizeIterator for AxisIter {}
