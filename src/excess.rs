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
