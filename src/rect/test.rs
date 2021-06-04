use crate::{coord::CoordPair, rect::Rect};

#[test]
fn from_range() {
    assert_eq!(
        Rect::<u16>::from_range(
            CoordPair { x: 1, y: 2 },
            CoordPair { x: 4, y: 3 }
        ),
        Rect {
            start: CoordPair { x: 1, y: 2 },
            size: CoordPair { x: 3, y: 1 }
        }
    );

    assert_eq!(
        Rect::<u16>::from_range(
            CoordPair { x: 5, y: 1 },
            CoordPair { x: 5, y: 1 }
        ),
        Rect {
            start: CoordPair { x: 5, y: 1 },
            size: CoordPair { x: 0, y: 0 }
        }
    );

    assert_eq!(
        Rect::<u16>::from_range(
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
        Rect::<i16>::try_from_range(
            CoordPair { x: 1, y: 2 },
            CoordPair { x: 4, y: 3 }
        ),
        Some(Rect {
            start: CoordPair { x: 1, y: 2 },
            size: CoordPair { x: 3, y: 1 }
        })
    );

    assert_eq!(
        Rect::<i16>::try_from_range(
            CoordPair { x: -5, y: 1 },
            CoordPair { x: -5, y: 1 }
        ),
        Some(Rect {
            start: CoordPair { x: -5, y: 1 },
            size: CoordPair { x: 0, y: 0 }
        })
    );

    assert_eq!(
        Rect::<i16>::try_from_range(
            CoordPair { x: 0, y: 0 },
            CoordPair { x: i16::MAX, y: i16::MAX }
        ),
        Some(Rect {
            start: CoordPair { x: 0, y: 0 },
            size: CoordPair { x: i16::MAX, y: i16::MAX }
        })
    );

    assert_eq!(
        Rect::<i16>::try_from_range(
            CoordPair { x: -2, y: -2 },
            CoordPair { x: i16::MAX, y: i16::MAX }
        ),
        None,
    );
}

#[test]
fn from_range_incl() {
    assert_eq!(
        Rect::<u16>::from_range_incl(
            CoordPair { x: 1, y: 2 },
            CoordPair { x: 3, y: 2 }
        ),
        Rect {
            start: CoordPair { x: 1, y: 2 },
            size: CoordPair { x: 3, y: 1 }
        }
    );

    assert_eq!(
        Rect::<u16>::from_range_incl(
            CoordPair { x: 5, y: 1 },
            CoordPair { x: 4, y: 0 }
        ),
        Rect {
            start: CoordPair { x: 5, y: 1 },
            size: CoordPair { x: 0, y: 0 }
        }
    );

    assert_eq!(
        Rect::<u16>::from_range_incl(
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
        Rect::<i16>::try_from_range_incl(
            CoordPair { x: 1, y: -2 },
            CoordPair { x: 3, y: -2 }
        ),
        Some(Rect {
            start: CoordPair { x: 1, y: -2 },
            size: CoordPair { x: 3, y: 1 }
        })
    );

    assert_eq!(
        Rect::<i16>::try_from_range_incl(
            CoordPair { x: -5, y: 1 },
            CoordPair { x: -4, y: 0 }
        ),
        Some(Rect {
            start: CoordPair { x: -5, y: 1 },
            size: CoordPair { x: 0, y: 0 }
        })
    );

    assert_eq!(
        Rect::<i16>::try_from_range_incl(
            CoordPair { x: 0, y: 0 },
            CoordPair { x: i16::MAX - 1, y: i16::MAX - 1 }
        ),
        Some(Rect {
            start: CoordPair { x: 0, y: 0 },
            size: CoordPair { x: i16::MAX, y: i16::MAX }
        })
    );

    assert_eq!(
        Rect::<i16>::try_from_range_incl(
            CoordPair { x: -5, y: 9 },
            CoordPair { x: i16::MAX - 1, y: i16::MAX - 1 }
        ),
        None
    );

    assert_eq!(
        Rect::<u16>::try_from_range_incl(
            CoordPair { x: 0, y: 0 },
            CoordPair { x: u16::MAX, y: u16::MAX }
        ),
        None,
    );
}

#[test]
fn is_empty() {
    let rect = Rect::<u16> {
        start: CoordPair { x: 3, y: 5 },
        size: CoordPair { x: 10, y: 20 },
    };
    assert!(!rect.is_empty());

    let rect = Rect::<u16> {
        start: CoordPair { x: 3, y: 5 },
        size: CoordPair { x: 0, y: 0 },
    };
    assert!(rect.is_empty());

    let rect = Rect::<u16> {
        start: CoordPair { x: 3, y: 5 },
        size: CoordPair { x: 0, y: 2 },
    };
    assert!(rect.is_empty());
}

#[test]
fn end() {
    let rect = Rect::<u16>::from_range(
        CoordPair { x: 1, y: 2 },
        CoordPair { x: 4, y: 3 },
    );
    assert_eq!(rect.end(), CoordPair { x: 4, y: 3 });

    let rect = Rect::<u16>::from_range(
        CoordPair { x: 5, y: 1 },
        CoordPair { x: 5, y: 1 },
    );
    assert_eq!(rect.end(), CoordPair { x: 5, y: 1 });

    let rect = Rect::<u16>::from_range(
        CoordPair { x: 0, y: 0 },
        CoordPair { x: u16::MAX, y: u16::MAX },
    );
    assert_eq!(rect.end(), CoordPair { x: u16::MAX, y: u16::MAX });
}

#[test]
fn wrapping_end() {
    let rect = Rect::<u16>::from_range(
        CoordPair { x: 1, y: 2 },
        CoordPair { x: 4, y: 3 },
    );
    assert_eq!(rect.wrapping_end(), CoordPair { x: 4, y: 3 });

    let rect = Rect::<u8> {
        start: CoordPair { x: 5, y: 253 },
        size: CoordPair { x: 1, y: 5 },
    };
    assert_eq!(rect.wrapping_end(), CoordPair { x: 6, y: 2 });

    let rect = Rect::<u8> {
        start: CoordPair { x: 255, y: 254 },
        size: CoordPair { x: 1, y: 3 },
    };
    assert_eq!(rect.wrapping_end(), CoordPair { x: 0, y: 1 });
}

#[test]
fn saturating_end() {
    let rect = Rect::<u16>::from_range(
        CoordPair { x: 1, y: 2 },
        CoordPair { x: 4, y: 3 },
    );
    assert_eq!(rect.saturating_end(), CoordPair { x: 4, y: 3 });

    let rect = Rect::<u8> {
        start: CoordPair { x: 5, y: 253 },
        size: CoordPair { x: 1, y: 5 },
    };
    assert_eq!(rect.saturating_end(), CoordPair { x: 6, y: 255 });

    let rect = Rect::<u8> {
        start: CoordPair { x: 255, y: 254 },
        size: CoordPair { x: 1, y: 3 },
    };
    assert_eq!(rect.saturating_end(), CoordPair { x: 255, y: 255 });
}

#[test]
fn checked_end() {
    let rect = Rect::<u16>::from_range(
        CoordPair { x: 1, y: 2 },
        CoordPair { x: 4, y: 3 },
    );
    assert_eq!(rect.checked_end(), Some(CoordPair { x: 4, y: 3 }));

    let rect = Rect::<u8> {
        start: CoordPair { x: 5, y: 253 },
        size: CoordPair { x: 1, y: 5 },
    };
    assert_eq!(rect.checked_end(), None);

    let rect = Rect::<u8> {
        start: CoordPair { x: 255, y: 254 },
        size: CoordPair { x: 1, y: 3 },
    };
    assert_eq!(rect.checked_end(), None);
}

#[test]
fn end_ref() {
    let rect = Rect::<u16>::from_range(
        CoordPair { x: 1, y: 2 },
        CoordPair { x: 4, y: 3 },
    );
    assert_eq!(rect.end_ref(), CoordPair { x: 4, y: 3 });

    let rect = Rect::<u16>::from_range(
        CoordPair { x: 5, y: 1 },
        CoordPair { x: 5, y: 1 },
    );
    assert_eq!(rect.end_ref(), CoordPair { x: 5, y: 1 });

    let rect = Rect::<u16>::from_range(
        CoordPair { x: 0, y: 0 },
        CoordPair { x: u16::MAX, y: u16::MAX },
    );
    assert_eq!(rect.end_ref(), CoordPair { x: u16::MAX, y: u16::MAX });
}

#[test]
fn end_incl() {
    let rect = Rect::<u16>::from_range(
        CoordPair { x: 1, y: 2 },
        CoordPair { x: 4, y: 3 },
    );
    assert_eq!(rect.end_inclusive(), CoordPair { x: 3, y: 2 });

    let rect = Rect::<u16>::from_range(
        CoordPair { x: 5, y: 1 },
        CoordPair { x: 5, y: 1 },
    );
    assert_eq!(rect.end_inclusive(), CoordPair { x: 4, y: 0 });

    let rect = Rect::<u16>::from_range(
        CoordPair { x: 0, y: 0 },
        CoordPair { x: u16::MAX, y: u16::MAX },
    );
    assert_eq!(
        rect.end_inclusive(),
        CoordPair { x: u16::MAX - 1, y: u16::MAX - 1 }
    );
}

#[test]
fn wrapping_end_incl() {
    let rect = Rect::<u16>::from_range(
        CoordPair { x: 1, y: 2 },
        CoordPair { x: 4, y: 3 },
    );
    assert_eq!(rect.wrapping_end_incl(), CoordPair { x: 3, y: 2 });

    let rect = Rect::<u8> {
        start: CoordPair { x: 5, y: 253 },
        size: CoordPair { x: 1, y: 5 },
    };
    assert_eq!(rect.wrapping_end_incl(), CoordPair { x: 5, y: 1 });

    let rect = Rect::<u8> {
        start: CoordPair { x: 255, y: 254 },
        size: CoordPair { x: 1, y: 3 },
    };
    assert_eq!(rect.wrapping_end_incl(), CoordPair { x: 255, y: 0 });
}

#[test]
fn saturating_end_incl() {
    let rect = Rect::<u16>::from_range(
        CoordPair { x: 1, y: 2 },
        CoordPair { x: 4, y: 3 },
    );
    assert_eq!(rect.saturating_end_incl(), CoordPair { x: 3, y: 2 });

    let rect = Rect::<u8> {
        start: CoordPair { x: 5, y: 253 },
        size: CoordPair { x: 1, y: 5 },
    };
    assert_eq!(rect.saturating_end_incl(), CoordPair { x: 5, y: 255 });

    let rect = Rect::<u8> {
        start: CoordPair { x: 255, y: 254 },
        size: CoordPair { x: 1, y: 3 },
    };
    assert_eq!(rect.saturating_end_incl(), CoordPair { x: 255, y: 255 });

    let rect = Rect::<u8> {
        start: CoordPair { x: 0, y: 1 },
        size: CoordPair { x: 5, y: 7 },
    };
    assert_eq!(rect.saturating_end_incl(), CoordPair { x: 4, y: 7 });
}

#[test]
fn checked_end_incl() {
    let rect = Rect::<u16>::from_range(
        CoordPair { x: 1, y: 2 },
        CoordPair { x: 4, y: 3 },
    );
    assert_eq!(rect.checked_end_incl(), Some(CoordPair { x: 3, y: 2 }));

    let rect = Rect::<u8> {
        start: CoordPair { x: 5, y: 253 },
        size: CoordPair { x: 1, y: 5 },
    };
    assert_eq!(rect.checked_end_incl(), None);

    let rect = Rect::<u8> {
        start: CoordPair { x: 255, y: 254 },
        size: CoordPair { x: 1, y: 3 },
    };
    assert_eq!(rect.checked_end_incl(), None);
}

#[test]
fn end_incl_ref() {
    let rect = Rect::<u16>::from_range(
        CoordPair { x: 1, y: 2 },
        CoordPair { x: 4, y: 3 },
    );
    assert_eq!(rect.end_incl_ref(), CoordPair { x: 3, y: 2 });

    let rect = Rect::<u16>::from_range(
        CoordPair { x: 5, y: 1 },
        CoordPair { x: 5, y: 1 },
    );
    assert_eq!(rect.end_incl_ref(), CoordPair { x: 4, y: 0 });

    let rect = Rect::<u16>::from_range(
        CoordPair { x: 0, y: 0 },
        CoordPair { x: u16::MAX, y: u16::MAX },
    );
    assert_eq!(
        rect.end_incl_ref(),
        CoordPair { x: u16::MAX - 1, y: u16::MAX - 1 }
    );
}

#[test]
fn end_non_empty() {
    let rect = Rect::<u16>::from_range(
        CoordPair { x: 1, y: 2 },
        CoordPair { x: 4, y: 3 },
    );
    assert_eq!(rect.end_non_empty(), Some(CoordPair { x: 3, y: 2 }));

    let rect = Rect {
        start: CoordPair { x: 9, y: 7 },
        size: CoordPair { x: 0, y: 0 },
    };
    assert_eq!(rect.end_non_empty(), None);
}

#[test]
fn end_non_empty_ref() {
    let rect = Rect::<u16>::from_range(
        CoordPair { x: 1, y: 2 },
        CoordPair { x: 4, y: 3 },
    );
    assert_eq!(rect.end_non_empty_ref(), Some(CoordPair { x: 3, y: 2 }));

    let rect = Rect {
        start: CoordPair { x: 9, y: 7 },
        size: CoordPair { x: 0, y: 0 },
    };
    assert_eq!(rect.end_non_empty_ref(), None);
}

#[test]
fn has_point() {
    let rect = Rect::<u8> {
        start: CoordPair { x: 9, y: 12 },
        size: CoordPair { x: 10, y: 5 },
    };
    assert!(rect.has_point(CoordPair { x: 9, y: 12 }));
    assert!(rect.has_point(CoordPair { x: 9, y: 15 }));
    assert!(rect.has_point(CoordPair { x: 15, y: 15 }));
    assert!(rect.has_point(CoordPair { x: 18, y: 12 }));
    assert!(rect.has_point(CoordPair { x: 18, y: 15 }));
    assert!(rect.has_point(CoordPair { x: 18, y: 16 }));

    let rect = Rect::<u8> {
        start: CoordPair { x: 254, y: 253 },
        size: CoordPair { x: 2, y: 3 },
    };
    assert!(rect.has_point(CoordPair { x: 255, y: 255 }));
    assert!(rect.has_point(CoordPair { x: 254, y: 253 }));
}

#[test]
fn has_not_point() {
    let rect = Rect::<u8> {
        start: CoordPair { x: 9, y: 12 },
        size: CoordPair { x: 10, y: 5 },
    };
    assert!(!rect.has_point(CoordPair { x: 0, y: 3 }));
    assert!(!rect.has_point(CoordPair { x: 8, y: 11 }));
    assert!(!rect.has_point(CoordPair { x: 8, y: 12 }));
    assert!(!rect.has_point(CoordPair { x: 9, y: 11 }));
    assert!(!rect.has_point(CoordPair { x: 15, y: 11 }));
    assert!(!rect.has_point(CoordPair { x: 8, y: 15 }));
    assert!(!rect.has_point(CoordPair { x: 19, y: 15 }));
    assert!(!rect.has_point(CoordPair { x: 15, y: 17 }));
    assert!(!rect.has_point(CoordPair { x: 19, y: 17 }));
    assert!(!rect.has_point(CoordPair { x: 28, y: 45 }));

    let rect = Rect::<u8> {
        start: CoordPair { x: 254, y: 253 },
        size: CoordPair { x: 2, y: 3 },
    };
    assert!(!rect.has_point(CoordPair { x: 0, y: 0 }));
    assert!(!rect.has_point(CoordPair { x: 253, y: 253 }));
}

#[test]
fn overlaps() {
    let left = Rect::<u8> {
        start: CoordPair { x: 9, y: 12 },
        size: CoordPair { x: 10, y: 5 },
    };
    assert!(left.overlaps(&left));

    let right = Rect::<u8> {
        start: CoordPair { x: 5, y: 6 },
        size: CoordPair { x: 10, y: 11 },
    };
    assert!(left.overlaps(&right));

    let right = Rect::<u8> {
        start: CoordPair { x: 15, y: 13 },
        size: CoordPair { x: 20, y: 24 },
    };
    assert!(left.overlaps(&right));

    let right = Rect::<u8> {
        start: CoordPair { x: 5, y: 13 },
        size: CoordPair { x: 15, y: 24 },
    };
    assert!(left.overlaps(&right));

    let right = Rect::<u8> {
        start: CoordPair { x: 15, y: 6 },
        size: CoordPair { x: 20, y: 17 },
    };
    assert!(left.overlaps(&right));

    let right = Rect::<u8> {
        start: CoordPair { x: 0, y: 13 },
        size: CoordPair { x: 10, y: 15 },
    };
    assert!(left.overlaps(&right));

    let right = Rect::<u8> {
        start: CoordPair { x: 12, y: 0 },
        size: CoordPair { x: 5, y: 13 },
    };
    assert!(left.overlaps(&right));

    let right = Rect::<u8> {
        start: CoordPair { x: 18, y: 13 },
        size: CoordPair { x: 2, y: 15 },
    };
    assert!(left.overlaps(&right));

    let right = Rect::<u8> {
        start: CoordPair { x: 12, y: 16 },
        size: CoordPair { x: 5, y: 50 },
    };
    assert!(left.overlaps(&right));

    let left = Rect::<u8> {
        start: CoordPair { x: 254, y: 253 },
        size: CoordPair { x: 2, y: 3 },
    };
    assert!(left.overlaps(&left));

    let right = Rect::<u8> {
        start: CoordPair { x: 253, y: 251 },
        size: CoordPair { x: 2, y: 4 },
    };
    assert!(left.overlaps(&right));
}

#[test]
fn does_not_overlap() {
    let left = Rect::<u8> {
        start: CoordPair { x: 9, y: 12 },
        size: CoordPair { x: 10, y: 5 },
    };

    let right = Rect::<u8> {
        start: CoordPair { x: 12, y: 17 },
        size: CoordPair { x: 5, y: 50 },
    };
    assert!(!left.overlaps(&right));

    let right = Rect::<u8> {
        start: CoordPair { x: 19, y: 13 },
        size: CoordPair { x: 5, y: 90 },
    };
    assert!(!left.overlaps(&right));

    let right = Rect::<u8> {
        start: CoordPair { x: 12, y: 0 },
        size: CoordPair { x: 8, y: 11 },
    };
    assert!(!left.overlaps(&right));

    let right = Rect::<u8> {
        start: CoordPair { x: 0, y: 13 },
        size: CoordPair { x: 8, y: 15 },
    };
    assert!(!left.overlaps(&right));

    let right = Rect::<u8> {
        start: CoordPair { x: 0, y: 3 },
        size: CoordPair { x: 1, y: 2 },
    };
    assert!(!left.overlaps(&right));

    let right = Rect::<u8> {
        start: CoordPair { x: 90, y: 45 },
        size: CoordPair { x: 120, y: 75 },
    };
    assert!(!left.overlaps(&right));

    let left = Rect::<u8> {
        start: CoordPair { x: 254, y: 253 },
        size: CoordPair { x: 2, y: 3 },
    };
    let right = Rect::<u8> {
        start: CoordPair { x: 0, y: 0 },
        size: CoordPair { x: 2, y: 4 },
    };
    assert!(!left.overlaps(&right));

    let right = Rect::<u8> {
        start: CoordPair { x: 253, y: 252 },
        size: CoordPair { x: 1, y: 1 },
    };
    assert!(!left.overlaps(&right));
}

#[test]
fn overlapped() {
    let left = Rect::<u8> {
        start: CoordPair { x: 9, y: 12 },
        size: CoordPair { x: 10, y: 5 },
    };
    assert_eq!(left.overlapped(&left), left);

    let right = Rect::<u8> {
        start: CoordPair { x: 5, y: 6 },
        size: CoordPair { x: 10, y: 11 },
    };
    assert_eq!(
        left.overlapped(&right),
        Rect {
            start: CoordPair { x: 9, y: 12 },
            size: CoordPair { x: 6, y: 5 },
        }
    );

    let left = Rect::<u8> {
        start: CoordPair { x: 253, y: 252 },
        size: CoordPair { x: 3, y: 4 },
    };
    assert_eq!(left.overlapped(&left), left);

    let right = Rect::<u8> {
        start: CoordPair { x: 250, y: 251 },
        size: CoordPair { x: 4, y: 4 },
    };
    assert_eq!(
        left.overlapped(&right),
        Rect {
            start: CoordPair { x: 253, y: 252 },
            size: CoordPair { x: 1, y: 3 },
        }
    );
}

#[test]
fn wrapping_overlapped() {
    let left = Rect::<u8> {
        start: CoordPair { x: 9, y: 12 },
        size: CoordPair { x: 10, y: 5 },
    };
    assert_eq!(left.wrapping_overlapped(&left), left);

    let right = Rect::<u8> {
        start: CoordPair { x: 5, y: 6 },
        size: CoordPair { x: 10, y: 11 },
    };
    assert_eq!(
        left.wrapping_overlapped(&right),
        Rect {
            start: CoordPair { x: 9, y: 12 },
            size: CoordPair { x: 6, y: 5 },
        }
    );

    let left = Rect::<u8> {
        start: CoordPair { x: 253, y: 252 },
        size: CoordPair { x: 3, y: 4 },
    };
    assert_eq!(left.wrapping_overlapped(&left), left);

    let right = Rect::<u8> {
        start: CoordPair { x: 250, y: 251 },
        size: CoordPair { x: 4, y: 4 },
    };
    assert_eq!(
        left.wrapping_overlapped(&right),
        Rect {
            start: CoordPair { x: 253, y: 252 },
            size: CoordPair { x: 1, y: 3 },
        }
    );

    let left = Rect::<u8> {
        start: CoordPair { x: 253, y: 252 },
        size: CoordPair { x: 12, y: 9 },
    };
    assert_eq!(left.wrapping_overlapped(&left), left);
    assert_eq!(
        left.wrapping_overlapped(&right),
        Rect {
            start: CoordPair { x: 253, y: 252 },
            size: CoordPair { x: 1, y: 3 },
        }
    );
}

#[test]
fn saturating_overlapped() {
    let left = Rect::<u8> {
        start: CoordPair { x: 9, y: 12 },
        size: CoordPair { x: 10, y: 5 },
    };
    assert_eq!(left.saturating_overlapped(&left), left);

    let right = Rect::<u8> {
        start: CoordPair { x: 5, y: 6 },
        size: CoordPair { x: 10, y: 11 },
    };
    assert_eq!(
        left.saturating_overlapped(&right),
        Rect {
            start: CoordPair { x: 9, y: 12 },
            size: CoordPair { x: 6, y: 5 },
        }
    );

    let left = Rect::<u8> {
        start: CoordPair { x: 253, y: 252 },
        size: CoordPair { x: 3, y: 4 },
    };
    assert_eq!(left.saturating_overlapped(&left), left);

    let right = Rect::<u8> {
        start: CoordPair { x: 250, y: 251 },
        size: CoordPair { x: 4, y: 4 },
    };
    assert_eq!(
        left.saturating_overlapped(&right),
        Rect {
            start: CoordPair { x: 253, y: 252 },
            size: CoordPair { x: 1, y: 3 },
        }
    );

    let left = Rect::<u8> {
        start: CoordPair { x: 0, y: 1 },
        size: CoordPair { x: 5, y: 7 },
    };
    assert_eq!(left.saturating_overlapped(&left), left);
    let right = Rect::<u8> {
        start: CoordPair { x: 3, y: 6 },
        size: CoordPair { x: 10, y: 11 },
    };
    assert_eq!(
        left.saturating_overlapped(&right),
        Rect {
            start: CoordPair { x: 3, y: 6 },
            size: CoordPair { x: 2, y: 2 },
        }
    );
}

#[test]
fn checked_overlapped() {
    let left = Rect::<u8> {
        start: CoordPair { x: 9, y: 12 },
        size: CoordPair { x: 10, y: 5 },
    };
    assert_eq!(left.checked_overlapped(&left), Some(left));

    let right = Rect::<u8> {
        start: CoordPair { x: 5, y: 6 },
        size: CoordPair { x: 10, y: 11 },
    };
    assert_eq!(
        left.checked_overlapped(&right),
        Some(Rect {
            start: CoordPair { x: 9, y: 12 },
            size: CoordPair { x: 6, y: 5 },
        })
    );

    let left = Rect::<u8> {
        start: CoordPair { x: 253, y: 252 },
        size: CoordPair { x: 3, y: 4 },
    };
    assert_eq!(left.checked_overlapped(&left), Some(left));

    let right = Rect::<u8> {
        start: CoordPair { x: 250, y: 251 },
        size: CoordPair { x: 4, y: 4 },
    };
    assert_eq!(
        left.checked_overlapped(&right),
        Some(Rect {
            start: CoordPair { x: 253, y: 252 },
            size: CoordPair { x: 1, y: 3 },
        })
    );

    let left = Rect::<u8> {
        start: CoordPair { x: 0, y: 1 },
        size: CoordPair { x: 5, y: 7 },
    };
    assert_eq!(left.checked_overlapped(&left), Some(left));

    let right = Rect::<u8> {
        start: CoordPair { x: 3, y: 6 },
        size: CoordPair { x: 10, y: 11 },
    };
    assert_eq!(
        left.checked_overlapped(&right),
        Some(Rect {
            start: CoordPair { x: 3, y: 6 },
            size: CoordPair { x: 2, y: 2 },
        })
    );

    let right = Rect::<u8> {
        start: CoordPair { x: 0, y: 0 },
        size: CoordPair { x: 2, y: 1 },
    };
    assert_eq!(left.checked_overlapped(&right), None,);
}

#[test]
fn columns() {
    let rect = Rect {
        start: CoordPair { x: 1, y: 3 },
        size: CoordPair { x: 4, y: 3 },
    };
    let points: Vec<_> = rect.columns().collect();
    let mut answer = vec![
        CoordPair { x: 1, y: 3 },
        CoordPair { x: 1, y: 4 },
        CoordPair { x: 1, y: 5 },
        CoordPair { x: 2, y: 3 },
        CoordPair { x: 2, y: 4 },
        CoordPair { x: 2, y: 5 },
        CoordPair { x: 3, y: 3 },
        CoordPair { x: 3, y: 4 },
        CoordPair { x: 3, y: 5 },
        CoordPair { x: 4, y: 3 },
        CoordPair { x: 4, y: 4 },
        CoordPair { x: 4, y: 5 },
    ];
    assert_eq!(points, answer);

    answer.reverse();
    let points: Vec<_> = rect.columns().rev().collect();
    assert_eq!(points, answer);

    let rect = Rect::<u16> {
        start: CoordPair { x: 1, y: 3 },
        size: CoordPair { x: 0, y: 0 },
    };
    assert_eq!(rect.columns().collect::<Vec<_>>(), Vec::new());
}

#[test]
fn rows() {
    let rect = Rect {
        start: CoordPair { x: 1, y: 3 },
        size: CoordPair { x: 4, y: 3 },
    };
    let points: Vec<_> = rect.rows().collect();
    let mut answer = vec![
        CoordPair { x: 1, y: 3 },
        CoordPair { x: 2, y: 3 },
        CoordPair { x: 3, y: 3 },
        CoordPair { x: 4, y: 3 },
        CoordPair { x: 1, y: 4 },
        CoordPair { x: 2, y: 4 },
        CoordPair { x: 3, y: 4 },
        CoordPair { x: 4, y: 4 },
        CoordPair { x: 1, y: 5 },
        CoordPair { x: 2, y: 5 },
        CoordPair { x: 3, y: 5 },
        CoordPair { x: 4, y: 5 },
    ];
    assert_eq!(points, answer);

    answer.reverse();
    let points: Vec<_> = rect.rows().rev().collect();
    assert_eq!(points, answer);

    let rect = Rect::<u16> {
        start: CoordPair { x: 1, y: 3 },
        size: CoordPair { x: 0, y: 0 },
    };
    assert_eq!(Vec::<CoordPair<u16>>::new(), rect.rows().collect::<Vec<_>>());
}

#[test]
fn borders() {
    let rect = Rect {
        start: CoordPair { x: 1, y: 3 },
        size: CoordPair { x: 4, y: 3 },
    };
    let points: Vec<_> = rect.borders().collect();
    let answer = vec![
        CoordPair { x: 1, y: 3 },
        CoordPair { x: 1, y: 4 },
        CoordPair { x: 1, y: 5 },
        CoordPair { x: 4, y: 3 },
        CoordPair { x: 4, y: 4 },
        CoordPair { x: 4, y: 5 },
        CoordPair { x: 2, y: 3 },
        CoordPair { x: 3, y: 3 },
        CoordPair { x: 4, y: 3 },
        CoordPair { x: 2, y: 5 },
        CoordPair { x: 3, y: 5 },
        CoordPair { x: 4, y: 5 },
    ];
    assert_eq!(points, answer);

    let rect = Rect::<u16> {
        start: CoordPair { x: 1, y: 3 },
        size: CoordPair { x: 0, y: 0 },
    };
    assert_eq!(rect.borders().collect::<Vec<_>>(), Vec::new());
}
