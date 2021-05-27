use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Axis {
    Y,
    X,
}

impl Axis {
    pub const ALL: [Axis; 2] = [Axis::Y, Axis::X];

    pub fn all() -> impl DoubleEndedIterator<Item = Axis> {
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
