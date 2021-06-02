use crate::{coord::CoordPair, direc::Direction};
use num::traits::{
    CheckedAdd,
    CheckedDiv,
    CheckedMul,
    CheckedNeg,
    CheckedRem,
    CheckedSub,
    SaturatingAdd,
    SaturatingMul,
    SaturatingSub,
    WrappingAdd,
    WrappingMul,
    WrappingNeg,
    WrappingSub,
};

#[test]
fn basic_math() {
    let pair: CoordPair<i32> = CoordPair { x: 5, y: -9 };
    assert_eq!(-pair, CoordPair { x: -5, y: 9 });
    assert_eq!(pair + CoordPair { x: 1, y: 2 }, CoordPair { x: 6, y: -7 });
    assert_eq!(pair - CoordPair { x: 1, y: 2 }, CoordPair { x: 4, y: -11 });
    assert_eq!(pair * CoordPair { x: -2, y: 3 }, CoordPair { x: -10, y: -27 });
    assert_eq!(pair / CoordPair { x: 5, y: -3 }, CoordPair { x: 1, y: 3 });

    let other_pair: CoordPair<i32> = CoordPair { x: 8, y: -10 };
    assert_eq!(pair + &other_pair, CoordPair { x: 13, y: -19 });
    assert_eq!(&pair + other_pair, CoordPair { x: 13, y: -19 });
    assert_eq!(&pair + &other_pair, CoordPair { x: 13, y: -19 });
    assert_eq!(
        pair.as_ref() + other_pair.as_ref(),
        CoordPair { x: 13, y: -19 }
    );
    assert_eq!(pair.as_ref() + &other_pair, CoordPair { x: 13, y: -19 });
    assert_eq!(&pair + other_pair.as_ref(), CoordPair { x: 13, y: -19 });
}

#[test]
fn assign_math() {
    let mut pair: CoordPair<i32> = CoordPair { x: 5, y: -9 };
    pair += CoordPair { x: 1, y: 2 };
    assert_eq!(pair, CoordPair { x: 6, y: -7 });
    pair -= CoordPair { x: 1, y: 2 };
    assert_eq!(pair, CoordPair { x: 5, y: -9 });
    pair *= CoordPair { x: 3, y: 2 };
    assert_eq!(pair, CoordPair { x: 15, y: -18 });
    pair /= CoordPair { x: 3, y: 2 };
    assert_eq!(pair, CoordPair { x: 5, y: -9 });
    pair += &CoordPair { x: 1, y: 2 };
    assert_eq!(pair, CoordPair { x: 6, y: -7 });
}

#[test]
fn wrapping_math() {
    let pair: CoordPair<u8> = CoordPair { x: 3, y: 250 };
    assert_eq!(
        pair.wrapping_add(&CoordPair { x: 2, y: 1 }),
        CoordPair { x: 5, y: 251 }
    );
    assert_eq!(
        pair.wrapping_add(&CoordPair { x: 254, y: 2 }),
        CoordPair { x: 1, y: 252 }
    );
    assert_eq!(
        pair.wrapping_add(&CoordPair { x: 2, y: 254 }),
        CoordPair { x: 5, y: 248 }
    );

    assert_eq!(
        pair.wrapping_sub(&CoordPair { x: 2, y: 1 }),
        CoordPair { x: 1, y: 249 }
    );
    assert_eq!(
        pair.wrapping_sub(&CoordPair { x: 2, y: 254 }),
        CoordPair { x: 1, y: 252 }
    );
    assert_eq!(
        pair.wrapping_sub(&CoordPair { x: 254, y: 2 }),
        CoordPair { x: 5, y: 248 }
    );

    assert_eq!(
        pair.wrapping_mul(&CoordPair { x: 9, y: 1 }),
        CoordPair { x: 27, y: 250 }
    );
    assert_eq!(
        pair.wrapping_mul(&CoordPair { x: 5, y: 2 }),
        CoordPair { x: 15, y: 244 }
    );
    assert_eq!(
        pair.wrapping_mul(&CoordPair { x: 2, y: 5 }),
        CoordPair { x: 6, y: 226 }
    );

    assert_eq!(
        CoordPair { x: -5i8, y: 127 }.wrapping_neg(),
        CoordPair { x: 5, y: -127 }
    );
    assert_eq!(
        CoordPair { x: -5i8, y: -128 }.wrapping_neg(),
        CoordPair { x: 5, y: -128 }
    );
}

#[test]
fn saturating_math() {
    let pair: CoordPair<u8> = CoordPair { x: 3, y: 250 };
    assert_eq!(
        pair.saturating_add(&CoordPair { x: 2, y: 1 }),
        CoordPair { x: 5, y: 251 }
    );
    assert_eq!(
        pair.saturating_add(&CoordPair { x: 254, y: 2 }),
        CoordPair { x: 255, y: 252 }
    );
    assert_eq!(
        pair.saturating_add(&CoordPair { x: 2, y: 254 }),
        CoordPair { x: 5, y: 255 }
    );

    assert_eq!(
        pair.saturating_sub(&CoordPair { x: 2, y: 1 }),
        CoordPair { x: 1, y: 249 }
    );
    assert_eq!(
        pair.saturating_sub(&CoordPair { x: 2, y: 254 }),
        CoordPair { x: 1, y: 0 }
    );
    assert_eq!(
        pair.saturating_sub(&CoordPair { x: 254, y: 2 }),
        CoordPair { x: 0, y: 248 }
    );

    assert_eq!(
        pair.saturating_mul(&CoordPair { x: 9, y: 1 }),
        CoordPair { x: 27, y: 250 }
    );
    assert_eq!(
        pair.saturating_mul(&CoordPair { x: 5, y: 2 }),
        CoordPair { x: 15, y: 255 }
    );
    assert_eq!(
        pair.saturating_mul(&CoordPair { x: 2, y: 5 }),
        CoordPair { x: 6, y: 255 }
    );
}

#[test]
fn checked_math() {
    let pair: CoordPair<u8> = CoordPair { x: 3, y: 250 };
    assert_eq!(
        pair.checked_add(&CoordPair { x: 2, y: 1 }),
        Some(CoordPair { x: 5, y: 251 })
    );
    assert_eq!(pair.checked_add(&CoordPair { x: 254, y: 2 }), None);
    assert_eq!(pair.checked_add(&CoordPair { x: 2, y: 254 }), None);

    assert_eq!(
        pair.checked_sub(&CoordPair { x: 2, y: 1 }),
        Some(CoordPair { x: 1, y: 249 })
    );
    assert_eq!(pair.checked_sub(&CoordPair { x: 2, y: 254 }), None);
    assert_eq!(pair.checked_sub(&CoordPair { x: 254, y: 2 }), None);

    assert_eq!(
        pair.checked_mul(&CoordPair { x: 9, y: 1 }),
        Some(CoordPair { x: 27, y: 250 })
    );
    assert_eq!(pair.checked_mul(&CoordPair { x: 5, y: 2 }), None);
    assert_eq!(pair.checked_mul(&CoordPair { x: 2, y: 5 }), None);

    assert_eq!(
        pair.checked_div(&CoordPair { x: 3, y: 2 }),
        Some(CoordPair { x: 1, y: 125 })
    );
    assert_eq!(pair.checked_div(&CoordPair { x: 9, y: 0 }), None);

    assert_eq!(
        pair.checked_rem(&CoordPair { x: 2, y: 9 }),
        Some(CoordPair { x: 1, y: 7 })
    );
    assert_eq!(pair.checked_rem(&CoordPair { x: 8, y: 0 }), None);

    assert_eq!(
        CoordPair { x: -5i8, y: 127 }.checked_neg(),
        Some(CoordPair { x: 5, y: -127 })
    );
    assert_eq!(CoordPair { x: -5i8, y: -128 }.checked_neg(), None);
}

#[test]
fn dot() {
    let pair: CoordPair<u8> = CoordPair { x: 3, y: 5 };
    assert_eq!(pair.dot(CoordPair { x: 2, y: 4 }), 26);
    assert_eq!(pair.dot_ref(&CoordPair { x: 2, y: 4 }), 26);

    assert_eq!(pair.wrapping_dot(&CoordPair { x: 2, y: 4 }), 26);
    assert_eq!(pair.wrapping_dot(&CoordPair { x: 50, y: 25 }), 19);
    assert_eq!(pair.wrapping_dot(&CoordPair { x: 100, y: 2 }), 54);

    assert_eq!(pair.saturating_dot(&CoordPair { x: 2, y: 4 }), 26);
    assert_eq!(pair.saturating_dot(&CoordPair { x: 50, y: 25 }), 255);
    assert_eq!(pair.saturating_dot(&CoordPair { x: 100, y: 2 }), 255);

    assert_eq!(pair.checked_dot(&CoordPair { x: 2, y: 4 }), Some(26));
    assert_eq!(pair.checked_dot(&CoordPair { x: 50, y: 25 }), None);
    assert_eq!(pair.checked_dot(&CoordPair { x: 100, y: 2 }), None);
}

#[test]
fn sqr_magnitude() {
    let pair: CoordPair<u8> = CoordPair { x: 3, y: 5 };
    assert_eq!(pair.sqr_magnitude(), 34);
    assert_eq!(pair.sqr_magnitude_ref(), 34);
    assert_eq!(pair.wrapping_sqr_mag(), 34);
    assert_eq!(pair.saturating_sqr_mag(), 34);
    assert_eq!(pair.checked_sqr_mag(), Some(34));

    let pair: CoordPair<u8> = CoordPair { x: 16, y: 12 };
    assert_eq!(pair.wrapping_sqr_mag(), 144);
    assert_eq!(pair.saturating_sqr_mag(), 255);
    assert_eq!(pair.checked_sqr_mag(), None);

    let pair: CoordPair<u8> = CoordPair { x: 11, y: 12 };
    assert_eq!(pair.wrapping_sqr_mag(), 9);
    assert_eq!(pair.saturating_sqr_mag(), 255);
    assert_eq!(pair.checked_sqr_mag(), None);
}

#[test]
fn magnitude() {
    let float_eq = |a: f64, b: f64| (a - b).abs() <= 10e-10;
    let pair: CoordPair<f64> = CoordPair { x: 3.0, y: 5.0 };
    assert!(float_eq(pair.magnitude(), 34f64.sqrt()));
    assert!(float_eq(pair.magnitude_ref(), 34f64.sqrt()));

    let pair: CoordPair<f64> = CoordPair { x: -12.0, y: 99.5 };
    assert!(float_eq(pair.magnitude(), 10044.25f64.sqrt()));
    assert!(float_eq(pair.magnitude_ref(), 10044.25f64.sqrt()));
}

#[test]
fn int_magnitude() {
    let pair: CoordPair<u8> = CoordPair { x: 3, y: 5 };
    assert_eq!(pair.int_magnitude(), 5);
    assert_eq!(pair.int_magnitude_ref(), 5);
    assert_eq!(pair.wrapping_int_mag(), 5);
    assert_eq!(pair.saturating_int_mag(), 5);
    assert_eq!(pair.checked_int_mag(), Some(5));

    let pair: CoordPair<u8> = CoordPair { x: 16, y: 12 };
    assert_eq!(pair.wrapping_int_mag(), 12);
    assert_eq!(pair.saturating_int_mag(), 15);
    assert_eq!(pair.checked_int_mag(), None);

    let pair: CoordPair<u8> = CoordPair { x: 11, y: 12 };
    assert_eq!(pair.wrapping_int_mag(), 3);
    assert_eq!(pair.saturating_int_mag(), 15);
    assert_eq!(pair.checked_int_mag(), None);
}

#[test]
fn move_direction() {
    let pair: CoordPair<i16> = CoordPair { x: 3, y: -9 };
    assert_eq!(pair.move_by(Direction::Up, 8), CoordPair { x: 3, y: -17 });
    assert_eq!(pair.move_by(Direction::Down, 3), CoordPair { x: 3, y: -6 });
    assert_eq!(pair.move_by(Direction::Left, 9), CoordPair { x: -6, y: -9 });
    assert_eq!(pair.move_by(Direction::Right, 7), CoordPair { x: 10, y: -9 });

    assert_eq!(
        pair.wrapping_move_by(Direction::Up, &i16::MAX),
        CoordPair { x: 3, y: 32760 }
    );
    assert_eq!(
        pair.saturating_move_by(Direction::Right, &i16::MAX),
        CoordPair { x: i16::MAX, y: -9 }
    );
    assert_eq!(
        pair.checked_move_by(Direction::Left, &2),
        Some(CoordPair { x: 1, y: -9 })
    );
    assert_eq!(pair.checked_move_by(Direction::Down, &i16::MIN), None);

    let pair: CoordPair<u16> = CoordPair { x: 0xffff, y: 0 };
    assert_eq!(pair.move_one(Direction::Left), CoordPair { x: 0xfffe, y: 0 });
    assert_eq!(
        pair.wrapping_move(Direction::Up),
        CoordPair { x: 0xffff, y: 0xffff }
    );
    assert_eq!(
        pair.wrapping_move(Direction::Down),
        CoordPair { x: 0xffff, y: 1 }
    );
    assert_eq!(
        pair.saturating_move(Direction::Left),
        CoordPair { x: 0xfffe, y: 0 }
    );
    assert_eq!(
        pair.saturating_move(Direction::Right),
        CoordPair { x: 0xffff, y: 0 }
    );
    assert_eq!(pair.checked_move(Direction::Up), None);
    assert_eq!(
        pair.checked_move(Direction::Left),
        Some(CoordPair { x: 0xfffe, y: 0 })
    );
}

#[test]
fn center_origin() {
    let pair: CoordPair<u8> = CoordPair { x: 3, y: 130 };
    assert_eq!(
        pair.center_origin_at(&CoordPair { x: 20, y: 30 }),
        CoordPair { x: -17, y: -101 }
    );
    assert_eq!(pair.center_origin(), CoordPair { x: -125, y: -3 });
}
