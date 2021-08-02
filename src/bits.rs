use num::{
    traits::{WrappingAdd, WrappingSub},
    Bounded,
    Signed,
    Unsigned,
};
use std::ops::Sub;

pub trait CastSigned: Unsigned {
    type Target;

    fn cast_signed(&self) -> Self::Target;
}

pub trait CastUnsigned: Signed {
    type Target;

    fn cast_unsigned(&self) -> Self::Target;
}

macro_rules! cast_signedness {
    ($utype:ident, $itype:ident) => {
        impl CastSigned for $utype {
            type Target = $itype;

            fn cast_signed(&self) -> Self::Target {
                *self as $itype
            }
        }

        impl CastUnsigned for $itype {
            type Target = $utype;

            fn cast_unsigned(&self) -> Self::Target {
                *self as $utype
            }
        }
    };
}

cast_signedness! { u8, i8 }
cast_signedness! { u16, i16 }
cast_signedness! { u32, i32 }
cast_signedness! { u64, i64 }
cast_signedness! { u128, i128 }
cast_signedness! { usize, isize }

pub trait Distance<Rhs> {
    type Output;

    fn distance(self, other: Rhs) -> Self::Output;
}

impl<A> Distance<A> for A
where
    A: Ord + Sub<Self>,
{
    type Output = A::Output;

    fn distance(self, other: Self) -> Self::Output {
        if self > other {
            self - other
        } else {
            other - self
        }
    }
}

pub trait HalfExcess: Unsigned + Bounded {
    fn half_excess() -> Self;
}

impl<N> HalfExcess for N
where
    N: Unsigned + Bounded,
{
    fn half_excess() -> Self {
        Self::max_value() / (Self::one() + Self::one()) + Self::one()
    }
}

pub trait ExcessToSigned
where
    Self: CastSigned,
{
    fn excess_to_signed(&self, excess: &Self) -> Self::Target;

    fn half_exc_to_signed(&self) -> Self::Target
    where
        Self: HalfExcess,
    {
        self.excess_to_signed(&Self::half_excess())
    }
}

impl<N> ExcessToSigned for N
where
    N: CastSigned + WrappingSub,
{
    fn excess_to_signed(&self, excess: &Self) -> Self::Target {
        self.wrapping_sub(excess).cast_signed()
    }
}

pub trait SignedToExcess
where
    Self: CastUnsigned,
{
    fn signed_to_excess(&self, excess: &Self::Target) -> Self::Target;

    fn signed_to_half_exc(&self) -> Self::Target
    where
        Self::Target: HalfExcess,
    {
        self.signed_to_excess(&Self::Target::half_excess())
    }
}

impl<N> SignedToExcess for N
where
    N: CastUnsigned,
    N::Target: WrappingAdd,
{
    fn signed_to_excess(&self, excess: &Self::Target) -> Self::Target {
        self.cast_unsigned().wrapping_add(excess)
    }
}

#[cfg(test)]
mod test {
    use super::{
        CastSigned,
        CastUnsigned,
        Distance,
        ExcessToSigned,
        HalfExcess,
        SignedToExcess,
    };

    #[test]
    fn distance() {
        assert_eq!(8u8.distance(3), 5);
        assert_eq!(4u8.distance(7), 3);
        assert_eq!(5u8.distance(5), 0);
        assert_eq!((-9i8).distance(1), 10);
        assert_eq!((-2i8).distance(-17), 15);
    }

    #[test]
    fn half_excesss() {
        assert_eq!(u8::half_excess(), 0x80);
        assert_eq!(u16::half_excess(), 0x8000);
    }

    #[test]
    fn excesss_to_signed() {
        assert_eq!(0u8.excess_to_signed(&10), -10);
        assert_eq!(9u8.excess_to_signed(&10), -1);
        assert_eq!(10u8.excess_to_signed(&10), 0);
        assert_eq!(11u8.excess_to_signed(&10), 1);
        assert_eq!(13u8.excess_to_signed(&9), 4);
        assert_eq!(130u8.excess_to_signed(&128), 2);
    }

    #[test]
    fn signed_to_excesss() {
        assert_eq!((-10i8).signed_to_excess(&10), 0);
        assert_eq!((-1i8).signed_to_excess(&10), 9);
        assert_eq!(0i8.signed_to_excess(&10), 10);
        assert_eq!(1i8.signed_to_excess(&10), 11);
        assert_eq!(4i8.signed_to_excess(&9), 13);
        assert_eq!(2i8.signed_to_excess(&128), 130);
    }

    #[test]
    fn signed_to_unsigned() {
        assert_eq!(127i8.cast_unsigned(), 127);
        assert_eq!(9i8.cast_unsigned(), 9);
        assert_eq!(1i8.cast_unsigned(), 1);
        assert_eq!(0i8.cast_unsigned(), 0);
        assert_eq!((-1i8).cast_unsigned(), 255);
        assert_eq!((-3i8).cast_unsigned(), 253);
        assert_eq!((-127i8).cast_unsigned(), 129);
        assert_eq!((-128i8).cast_unsigned(), 128);
    }

    #[test]
    fn unsigned_to_signed() {
        assert_eq!(127u8.cast_signed(), 127);
        assert_eq!(9u8.cast_signed(), 9);
        assert_eq!(1u8.cast_signed(), 1);
        assert_eq!(0u8.cast_signed(), 0);
        assert_eq!(255u8.cast_signed(), -1);
        assert_eq!(253u8.cast_signed(), -3);
        assert_eq!(129u8.cast_signed(), -127);
        assert_eq!(128u8.cast_signed(), -128);
    }
}
