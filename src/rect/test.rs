use crate::{coord::CoordPair, rect::Rect};

#[test]
fn from_range() {
    assert_eq!(
        Rect::<u16, u16>::from_range(
            CoordPair { x: 1, y: 2 },
            CoordPair { x: 4, y: 3 }
        ),
        Rect {
            start: CoordPair { x: 1, y: 2 },
            size: CoordPair { x: 3, y: 1 }
        }
    );

    assert_eq!(
        Rect::<u16, u16>::from_range(
            CoordPair { x: 5, y: 1 },
            CoordPair { x: 5, y: 1 }
        ),
        Rect {
            start: CoordPair { x: 5, y: 1 },
            size: CoordPair { x: 0, y: 0 }
        }
    );

    assert_eq!(
        Rect::<u16, u16>::from_range(
            CoordPair { x: 0, y: 0 },
            CoordPair { x: u16::MAX, y: u16::MAX }
        ),
        Rect {
            start: CoordPair { x: 0, y: 0 },
            size: CoordPair { x: u16::MAX, y: u16::MAX }
        }
    );
}

#[test]
fn try_from_range() {
    assert_eq!(
        Rect::<i16, i16>::try_from_range(
            CoordPair { x: 1, y: 2 },
            CoordPair { x: 4, y: 3 }
        ),
        Some(Rect {
            start: CoordPair { x: 1, y: 2 },
            size: CoordPair { x: 3, y: 1 }
        })
    );

    assert_eq!(
        Rect::<i16, i16>::try_from_range(
            CoordPair { x: -5, y: 1 },
            CoordPair { x: -5, y: 1 }
        ),
        Some(Rect {
            start: CoordPair { x: -5, y: 1 },
            size: CoordPair { x: 0, y: 0 }
        })
    );

    assert_eq!(
        Rect::<i16, i16>::try_from_range(
            CoordPair { x: 0, y: 0 },
            CoordPair { x: i16::MAX, y: i16::MAX }
        ),
        Some(Rect {
            start: CoordPair { x: 0, y: 0 },
            size: CoordPair { x: i16::MAX, y: i16::MAX }
        })
    );

    assert_eq!(
        Rect::<i16, i16>::try_from_range(
            CoordPair { x: -2, y: -2 },
            CoordPair { x: i16::MAX, y: i16::MAX }
        ),
        None,
    );
}

#[test]
fn from_range_incl() {
    assert_eq!(
        Rect::<u16, u16>::from_range_incl(
            CoordPair { x: 1, y: 2 },
            CoordPair { x: 3, y: 2 }
        ),
        Rect {
            start: CoordPair { x: 1, y: 2 },
            size: CoordPair { x: 3, y: 1 }
        }
    );

    assert_eq!(
        Rect::<u16, u16>::from_range_incl(
            CoordPair { x: 5, y: 1 },
            CoordPair { x: 4, y: 0 }
        ),
        Rect {
            start: CoordPair { x: 5, y: 1 },
            size: CoordPair { x: 0, y: 0 }
        }
    );

    assert_eq!(
        Rect::<u16, u16>::from_range_incl(
            CoordPair { x: 0, y: 0 },
            CoordPair { x: u16::MAX - 1, y: u16::MAX - 1 }
        ),
        Rect {
            start: CoordPair { x: 0, y: 0 },
            size: CoordPair { x: u16::MAX, y: u16::MAX }
        }
    );
}

#[test]
fn try_from_range_incl() {
    assert_eq!(
        Rect::<i16, i16>::try_from_range_incl(
            CoordPair { x: 1, y: -2 },
            CoordPair { x: 3, y: -2 }
        ),
        Some(Rect {
            start: CoordPair { x: 1, y: -2 },
            size: CoordPair { x: 3, y: 1 }
        })
    );

    assert_eq!(
        Rect::<i16, i16>::try_from_range_incl(
            CoordPair { x: -5, y: 1 },
            CoordPair { x: -4, y: 0 }
        ),
        Some(Rect {
            start: CoordPair { x: -5, y: 1 },
            size: CoordPair { x: 0, y: 0 }
        })
    );

    assert_eq!(
        Rect::<i16, i16>::try_from_range_incl(
            CoordPair { x: 0, y: 0 },
            CoordPair { x: i16::MAX - 1, y: i16::MAX - 1 }
        ),
        Some(Rect {
            start: CoordPair { x: 0, y: 0 },
            size: CoordPair { x: i16::MAX, y: i16::MAX }
        })
    );

    assert_eq!(
        Rect::<i16, i16>::try_from_range_incl(
            CoordPair { x: -5, y: 9 },
            CoordPair { x: i16::MAX - 1, y: i16::MAX - 1 }
        ),
        None
    );

    assert_eq!(
        Rect::<u16, u16>::try_from_range_incl(
            CoordPair { x: 0, y: 0 },
            CoordPair { x: u16::MAX, y: u16::MAX }
        ),
        None,
    );
}
