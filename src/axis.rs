//! Utilites related to the axes of a plane.

use std::{fmt, ops::Not};

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
    pub fn iter() -> impl DoubleEndedIterator<Item = Axis> {
        Self::ALL.iter().copied()
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
