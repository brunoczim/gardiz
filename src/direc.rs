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
}
