use crate::{
    coord::Vec2,
    direc::{DirecVector, Direction},
};
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
    let pair: Vec2<i32> = Vec2 { x: 5, y: -9 };
    assert_eq!(-pair, Vec2 { x: -5, y: 9 });
    assert_eq!(pair + Vec2 { x: 1, y: 2 }, Vec2 { x: 6, y: -7 });
    assert_eq!(pair - Vec2 { x: 1, y: 2 }, Vec2 { x: 4, y: -11 });
    assert_eq!(pair * Vec2 { x: -2, y: 3 }, Vec2 { x: -10, y: -27 });
    assert_eq!(pair / Vec2 { x: 5, y: -3 }, Vec2 { x: 1, y: 3 });

    let other_pair: Vec2<i32> = Vec2 { x: 8, y: -10 };
    assert_eq!(pair + &other_pair, Vec2 { x: 13, y: -19 });
    assert_eq!(&pair + other_pair, Vec2 { x: 13, y: -19 });
    assert_eq!(&pair + &other_pair, Vec2 { x: 13, y: -19 });
    assert_eq!(pair.as_ref() + other_pair.as_ref(), Vec2 { x: 13, y: -19 });
    assert_eq!(pair.as_ref() + &other_pair, Vec2 { x: 13, y: -19 });
    assert_eq!(&pair + other_pair.as_ref(), Vec2 { x: 13, y: -19 });
}

#[test]
fn assign_math() {
    let mut pair: Vec2<i32> = Vec2 { x: 5, y: -9 };
    pair += Vec2 { x: 1, y: 2 };
    assert_eq!(pair, Vec2 { x: 6, y: -7 });
    pair -= Vec2 { x: 1, y: 2 };
    assert_eq!(pair, Vec2 { x: 5, y: -9 });
    pair *= Vec2 { x: 3, y: 2 };
    assert_eq!(pair, Vec2 { x: 15, y: -18 });
    pair /= Vec2 { x: 3, y: 2 };
    assert_eq!(pair, Vec2 { x: 5, y: -9 });
    pair += &Vec2 { x: 1, y: 2 };
    assert_eq!(pair, Vec2 { x: 6, y: -7 });
}

#[test]
fn wrapping_math() {
    let pair: Vec2<u8> = Vec2 { x: 3, y: 250 };
    assert_eq!(pair.wrapping_add(&Vec2 { x: 2, y: 1 }), Vec2 { x: 5, y: 251 });
    assert_eq!(
        pair.wrapping_add(&Vec2 { x: 254, y: 2 }),
        Vec2 { x: 1, y: 252 }
    );
    assert_eq!(
        pair.wrapping_add(&Vec2 { x: 2, y: 254 }),
        Vec2 { x: 5, y: 248 }
    );

    assert_eq!(pair.wrapping_sub(&Vec2 { x: 2, y: 1 }), Vec2 { x: 1, y: 249 });
    assert_eq!(
        pair.wrapping_sub(&Vec2 { x: 2, y: 254 }),
        Vec2 { x: 1, y: 252 }
    );
    assert_eq!(
        pair.wrapping_sub(&Vec2 { x: 254, y: 2 }),
        Vec2 { x: 5, y: 248 }
    );

    assert_eq!(pair.wrapping_mul(&Vec2 { x: 9, y: 1 }), Vec2 { x: 27, y: 250 });
    assert_eq!(pair.wrapping_mul(&Vec2 { x: 5, y: 2 }), Vec2 { x: 15, y: 244 });
    assert_eq!(pair.wrapping_mul(&Vec2 { x: 2, y: 5 }), Vec2 { x: 6, y: 226 });

    assert_eq!(Vec2 { x: -5i8, y: 127 }.wrapping_neg(), Vec2 { x: 5, y: -127 });
    assert_eq!(
        Vec2 { x: -5i8, y: -128 }.wrapping_neg(),
        Vec2 { x: 5, y: -128 }
    );
}

#[test]
fn saturating_math() {
    let pair: Vec2<u8> = Vec2 { x: 3, y: 250 };
    assert_eq!(
        pair.saturating_add(&Vec2 { x: 2, y: 1 }),
        Vec2 { x: 5, y: 251 }
    );
    assert_eq!(
        pair.saturating_add(&Vec2 { x: 254, y: 2 }),
        Vec2 { x: 255, y: 252 }
    );
    assert_eq!(
        pair.saturating_add(&Vec2 { x: 2, y: 254 }),
        Vec2 { x: 5, y: 255 }
    );

    assert_eq!(
        pair.saturating_sub(&Vec2 { x: 2, y: 1 }),
        Vec2 { x: 1, y: 249 }
    );
    assert_eq!(
        pair.saturating_sub(&Vec2 { x: 2, y: 254 }),
        Vec2 { x: 1, y: 0 }
    );
    assert_eq!(
        pair.saturating_sub(&Vec2 { x: 254, y: 2 }),
        Vec2 { x: 0, y: 248 }
    );

    assert_eq!(
        pair.saturating_mul(&Vec2 { x: 9, y: 1 }),
        Vec2 { x: 27, y: 250 }
    );
    assert_eq!(
        pair.saturating_mul(&Vec2 { x: 5, y: 2 }),
        Vec2 { x: 15, y: 255 }
    );
    assert_eq!(
        pair.saturating_mul(&Vec2 { x: 2, y: 5 }),
        Vec2 { x: 6, y: 255 }
    );
}

#[test]
fn checked_math() {
    let pair: Vec2<u8> = Vec2 { x: 3, y: 250 };
    assert_eq!(
        pair.checked_add(&Vec2 { x: 2, y: 1 }),
        Some(Vec2 { x: 5, y: 251 })
    );
    assert_eq!(pair.checked_add(&Vec2 { x: 254, y: 2 }), None);
    assert_eq!(pair.checked_add(&Vec2 { x: 2, y: 254 }), None);

    assert_eq!(
        pair.checked_sub(&Vec2 { x: 2, y: 1 }),
        Some(Vec2 { x: 1, y: 249 })
    );
    assert_eq!(pair.checked_sub(&Vec2 { x: 2, y: 254 }), None);
    assert_eq!(pair.checked_sub(&Vec2 { x: 254, y: 2 }), None);

    assert_eq!(
        pair.checked_mul(&Vec2 { x: 9, y: 1 }),
        Some(Vec2 { x: 27, y: 250 })
    );
    assert_eq!(pair.checked_mul(&Vec2 { x: 5, y: 2 }), None);
    assert_eq!(pair.checked_mul(&Vec2 { x: 2, y: 5 }), None);

    assert_eq!(
        pair.checked_div(&Vec2 { x: 3, y: 2 }),
        Some(Vec2 { x: 1, y: 125 })
    );
    assert_eq!(pair.checked_div(&Vec2 { x: 9, y: 0 }), None);

    assert_eq!(
        pair.checked_rem(&Vec2 { x: 2, y: 9 }),
        Some(Vec2 { x: 1, y: 7 })
    );
    assert_eq!(pair.checked_rem(&Vec2 { x: 8, y: 0 }), None);

    assert_eq!(
        Vec2 { x: -5i8, y: 127 }.checked_neg(),
        Some(Vec2 { x: 5, y: -127 })
    );
    assert_eq!(Vec2 { x: -5i8, y: -128 }.checked_neg(), None);
}

#[test]
fn dot() {
    let pair: Vec2<u8> = Vec2 { x: 3, y: 5 };
    assert_eq!(pair.dot(Vec2 { x: 2, y: 4 }), 26);
    assert_eq!(pair.dot_ref(&Vec2 { x: 2, y: 4 }), 26);

    assert_eq!(pair.wrapping_dot(&Vec2 { x: 2, y: 4 }), 26);
    assert_eq!(pair.wrapping_dot(&Vec2 { x: 50, y: 25 }), 19);
    assert_eq!(pair.wrapping_dot(&Vec2 { x: 100, y: 2 }), 54);

    assert_eq!(pair.saturating_dot(&Vec2 { x: 2, y: 4 }), 26);
    assert_eq!(pair.saturating_dot(&Vec2 { x: 50, y: 25 }), 255);
    assert_eq!(pair.saturating_dot(&Vec2 { x: 100, y: 2 }), 255);

    assert_eq!(pair.checked_dot(&Vec2 { x: 2, y: 4 }), Some(26));
    assert_eq!(pair.checked_dot(&Vec2 { x: 50, y: 25 }), None);
    assert_eq!(pair.checked_dot(&Vec2 { x: 100, y: 2 }), None);
}

#[test]
fn sqr_magnitude() {
    let pair: Vec2<u8> = Vec2 { x: 3, y: 5 };
    assert_eq!(pair.sqr_magnitude(), 34);
    assert_eq!(pair.sqr_magnitude_ref(), 34);
    assert_eq!(pair.wrapping_sqr_mag(), 34);
    assert_eq!(pair.saturating_sqr_mag(), 34);
    assert_eq!(pair.checked_sqr_mag(), Some(34));

    let pair: Vec2<u8> = Vec2 { x: 16, y: 12 };
    assert_eq!(pair.wrapping_sqr_mag(), 144);
    assert_eq!(pair.saturating_sqr_mag(), 255);
    assert_eq!(pair.checked_sqr_mag(), None);

    let pair: Vec2<u8> = Vec2 { x: 11, y: 12 };
    assert_eq!(pair.wrapping_sqr_mag(), 9);
    assert_eq!(pair.saturating_sqr_mag(), 255);
    assert_eq!(pair.checked_sqr_mag(), None);
}

#[test]
fn magnitude() {
    let float_eq = |a: f64, b: f64| (a - b).abs() <= 10e-10;
    let pair: Vec2<f64> = Vec2 { x: 3.0, y: 5.0 };
    assert!(float_eq(pair.magnitude(), 34f64.sqrt()));
    assert!(float_eq(pair.magnitude_ref(), 34f64.sqrt()));

    let pair: Vec2<f64> = Vec2 { x: -12.0, y: 99.5 };
    assert!(float_eq(pair.magnitude(), 10044.25f64.sqrt()));
    assert!(float_eq(pair.magnitude_ref(), 10044.25f64.sqrt()));
}

#[test]
fn int_magnitude() {
    let pair: Vec2<u8> = Vec2 { x: 3, y: 5 };
    assert_eq!(pair.int_magnitude(), 5);
    assert_eq!(pair.int_magnitude_ref(), 5);
    assert_eq!(pair.wrapping_int_mag(), 5);
    assert_eq!(pair.saturating_int_mag(), 5);
    assert_eq!(pair.checked_int_mag(), Some(5));

    let pair: Vec2<u8> = Vec2 { x: 16, y: 12 };
    assert_eq!(pair.wrapping_int_mag(), 12);
    assert_eq!(pair.saturating_int_mag(), 15);
    assert_eq!(pair.checked_int_mag(), None);

    let pair: Vec2<u8> = Vec2 { x: 11, y: 12 };
    assert_eq!(pair.wrapping_int_mag(), 3);
    assert_eq!(pair.saturating_int_mag(), 15);
    assert_eq!(pair.checked_int_mag(), None);
}

#[test]
fn move_direction() {
    let pair: Vec2<i16> = Vec2 { x: 3, y: -9 };
    assert_eq!(
        pair.move_by(DirecVector { direction: Direction::Up, magnitude: 8 }),
        Vec2 { x: 3, y: -17 }
    );
    assert_eq!(
        pair.move_by(DirecVector { direction: Direction::Down, magnitude: 3 }),
        Vec2 { x: 3, y: -6 }
    );
    assert_eq!(
        pair.move_by(DirecVector { direction: Direction::Left, magnitude: 9 }),
        Vec2 { x: -6, y: -9 }
    );
    assert_eq!(
        pair.move_by(DirecVector { direction: Direction::Right, magnitude: 7 }),
        Vec2 { x: 10, y: -9 }
    );

    assert_eq!(
        pair.wrapping_move_by(&DirecVector {
            direction: Direction::Up,
            magnitude: i16::MAX
        }),
        Vec2 { x: 3, y: 32760 }
    );
    assert_eq!(
        pair.saturating_move_by(&DirecVector {
            direction: Direction::Right,
            magnitude: i16::MAX
        }),
        Vec2 { x: i16::MAX, y: -9 }
    );
    assert_eq!(
        pair.checked_move_by(&DirecVector {
            direction: Direction::Left,
            magnitude: 2,
        }),
        Some(Vec2 { x: 1, y: -9 })
    );
    assert_eq!(
        pair.checked_move_by(&DirecVector {
            direction: Direction::Down,
            magnitude: i16::MIN,
        }),
        None
    );

    let pair: Vec2<u16> = Vec2 { x: 0xffff, y: 0 };
    assert_eq!(pair.move_one(Direction::Left), Vec2 { x: 0xfffe, y: 0 });
    assert_eq!(
        pair.wrapping_move(Direction::Up),
        Vec2 { x: 0xffff, y: 0xffff }
    );
    assert_eq!(pair.wrapping_move(Direction::Down), Vec2 { x: 0xffff, y: 1 });
    assert_eq!(pair.saturating_move(Direction::Left), Vec2 { x: 0xfffe, y: 0 });
    assert_eq!(
        pair.saturating_move(Direction::Right),
        Vec2 { x: 0xffff, y: 0 }
    );
    assert_eq!(pair.checked_move(Direction::Up), None);
    assert_eq!(
        pair.checked_move(Direction::Left),
        Some(Vec2 { x: 0xfffe, y: 0 })
    );
}

#[test]
fn direction_to() {
    let pair: Vec2<u64> = Vec2 { x: 10033, y: 987654321 };
    assert_eq!(pair.direction_to(&Vec2 { x: 10033, y: 987654321 }), None);
    assert_eq!(
        pair.direction_to(&Vec2 { x: 10033, y: 1 }),
        Some(Direction::Up)
    );
    assert_eq!(
        pair.direction_to(&Vec2 { x: 2, y: 987654321 }),
        Some(Direction::Left)
    );
    assert_eq!(
        pair.direction_to(&Vec2 { x: 10033, y: 10987654321 }),
        Some(Direction::Down)
    );
    assert_eq!(
        pair.direction_to(&Vec2 { x: 21144, y: 987654321 }),
        Some(Direction::Right)
    );
}

#[test]
fn center_origin() {
    let pair: Vec2<u8> = Vec2 { x: 3, y: 130 };
    assert_eq!(
        pair.center_origin_at(&Vec2 { x: 20, y: 30 }),
        Vec2 { x: -17, y: -101 }
    );
    assert_eq!(pair.center_origin(), Vec2 { x: -125, y: -3 });
}
