use crate::{
    axis::Axis,
    excess::{CastSigned, CastUnsigned},
};
use num::{
    integer::Roots,
    traits::{
        Bounded,
        CheckedAdd,
        CheckedDiv,
        CheckedMul,
        CheckedNeg,
        CheckedRem,
        CheckedSub,
        Float,
        Num,
        One,
        SaturatingAdd,
        SaturatingMul,
        SaturatingSub,
        Signed,
        Unsigned,
        WrappingAdd,
        WrappingMul,
        WrappingNeg,
        WrappingSub,
        Zero,
    },
};
use std::{
    error::Error,
    fmt,
    ops::{
        Add,
        AddAssign,
        Div,
        DivAssign,
        Index,
        IndexMut,
        Mul,
        MulAssign,
        Neg,
        Rem,
        RemAssign,
        Sub,
        SubAssign,
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct CoordPair<C> {
    pub y: C,
    pub x: C,
}

impl<C> Index<Axis> for CoordPair<C> {
    type Output = C;

    fn index(&self, axis: Axis) -> &Self::Output {
        match axis {
            Axis::Y => &self.y,
            Axis::X => &self.x,
        }
    }
}

impl<C> IndexMut<Axis> for CoordPair<C> {
    fn index_mut(&mut self, axis: Axis) -> &mut Self::Output {
        match axis {
            Axis::Y => &mut self.y,
            Axis::X => &mut self.x,
        }
    }
}

impl<C> CoordPair<C> {
    pub fn from_axes<F>(mut mapper: F) -> Self
    where
        F: FnMut(Axis) -> C,
    {
        Self { y: mapper(Axis::Y), x: mapper(Axis::X) }
    }

    pub fn as_ref(&self) -> CoordPair<&C> {
        CoordPair { x: &self.x, y: &self.y }
    }

    pub fn as_mut(&mut self) -> CoordPair<&mut C> {
        CoordPair { x: &mut self.x, y: &mut self.y }
    }

    pub fn map_with_axes<F, A>(self, mut mapper: F) -> CoordPair<A>
    where
        F: FnMut(Axis, C) -> A,
    {
        CoordPair { y: mapper(Axis::Y, self.y), x: mapper(Axis::X, self.x) }
    }

    pub fn map<F, A>(self, mut mapper: F) -> CoordPair<A>
    where
        F: FnMut(C) -> A,
    {
        CoordPair { y: mapper(self.y), x: mapper(self.x) }
    }

    pub fn fold<F, A>(self, init: A, mut folder: F) -> A
    where
        F: FnMut(C, A) -> A,
    {
        let acc = folder(self.x, init);
        folder(self.y, acc)
    }

    pub fn fold_rev<F, A>(self, init: A, mut folder: F) -> A
    where
        F: FnMut(A, C) -> A,
    {
        let acc = folder(init, self.y);
        folder(acc, self.x)
    }

    pub fn zip<A>(self, other: CoordPair<A>) -> CoordPair<(C, A)> {
        self.zip_with(other, |this, other| (this, other))
    }

    pub fn zip_with<F, A, B>(
        self,
        other: CoordPair<A>,
        mut zipper: F,
    ) -> CoordPair<B>
    where
        F: FnMut(C, A) -> B,
    {
        CoordPair { x: zipper(self.x, other.x), y: zipper(self.y, other.y) }
    }

    pub fn dot<A>(
        self,
        other: CoordPair<A>,
    ) -> <<C as Mul<A>>::Output as Add>::Output
    where
        C: Mul<A>,
        <C as Mul<A>>::Output: Add,
    {
        let prod = self.zip_with(other, |this, other| this * other);
        prod.y + prod.x
    }

    pub fn dot_ref<'this, 'other, A>(
        &'this self,
        other: &'other CoordPair<A>,
    ) -> <<&'this C as Mul<&'other A>>::Output as Add>::Output
    where
        &'this C: Mul<&'other A>,
        <&'this C as Mul<&'other A>>::Output: Add,
    {
        let prod =
            self.as_ref().zip_with(other.as_ref(), |this, other| this * other);
        prod.y + prod.x
    }

    pub fn wrapping_dot(&self, other: &Self) -> C
    where
        C: WrappingAdd + WrappingMul,
    {
        let prod = self
            .as_ref()
            .zip_with(other.as_ref(), |this, other| this.wrapping_mul(other));
        prod.y.wrapping_add(&prod.x)
    }

    pub fn saturating_dot(&self, other: &Self) -> C
    where
        C: SaturatingAdd + SaturatingMul,
    {
        let prod = self
            .as_ref()
            .zip_with(other.as_ref(), |this, other| this.saturating_mul(other));
        prod.y.saturating_add(&prod.x)
    }

    pub fn checked_dot(&self, other: &Self) -> Option<C>
    where
        C: CheckedAdd + CheckedMul,
    {
        let prod = self
            .as_ref()
            .zip_with(other.as_ref(), |this, other| this.checked_mul(other))
            .transpose()?;
        prod.y.checked_add(&prod.x)
    }

    pub fn sqr_magnitude(self) -> <C::Output as Add>::Output
    where
        C: Clone,
        C: Mul,
        C::Output: Add,
    {
        self.clone().dot(self)
    }

    /*
    pub fn sqr_magnitude_ref<'this>(
        &'this self,
    ) -> <<&'this C as Mul>::Output as Add>::Output
    where
        &'this C: Mul,
        <&'this C as Mul>::Output: Add,
    {
        self.dot_ref(self)
    }
    */

    pub fn magnitude(self) -> <C::Output as Add>::Output
    where
        C: Clone,
        C: Mul,
        C::Output: Add,
        <C::Output as Add>::Output: Float,
    {
        self.sqr_magnitude().sqrt()
    }

    pub fn int_magnitude(self) -> <C::Output as Add>::Output
    where
        C: Clone,
        C: Mul,
        C::Output: Add,
        <C::Output as Add>::Output: Roots,
    {
        self.sqr_magnitude().sqrt()
    }
}

impl CoordPair<i32> {
    pub fn sqr_magnitude_ref<'this>(&'this self) -> i32 {
        self.dot_ref(self)
    }
}

impl<C> CoordPair<Option<C>> {
    pub fn transpose(self) -> Option<CoordPair<C>> {
        match (self.y, self.x) {
            (Some(y), Some(x)) => Some(CoordPair { y, x }),
            _ => None,
        }
    }
}

impl<C> Zero for CoordPair<C>
where
    C: Zero,
{
    fn zero() -> Self {
        Self::from_axes(|_| C::zero())
    }

    fn is_zero(&self) -> bool {
        self.as_ref().fold(true, |elem, acc| acc && elem.is_zero())
    }

    fn set_zero(&mut self) {
        for axis in Axis::all() {
            self[axis].set_zero();
        }
    }
}

impl<C> One for CoordPair<C>
where
    C: One,
{
    fn one() -> Self {
        Self::from_axes(|_| C::one())
    }

    fn set_one(&mut self) {
        for axis in Axis::all() {
            self[axis].set_one();
        }
    }
}

macro_rules! elemwise_binop {
    ($trait:ident, $method:ident) => {
        impl<C, A> $trait<CoordPair<A>> for CoordPair<C>
        where
            C: $trait<A>,
        {
            type Output = CoordPair<C::Output>;

            fn $method(self, other: CoordPair<A>) -> Self::Output {
                self.zip_with(other, |this, other| this.$method(other))
            }
        }

        impl<'param, C, A> $trait<&'param CoordPair<A>> for CoordPair<C>
        where
            C: $trait<&'param A>,
        {
            type Output = CoordPair<C::Output>;

            fn $method(self, other: &'param CoordPair<A>) -> Self::Output {
                self.zip_with(other.as_ref(), |this, other| this.$method(other))
            }
        }

        impl<'this, C, A> $trait<CoordPair<A>> for &'this CoordPair<C>
        where
            &'this C: $trait<A>,
        {
            type Output = CoordPair<<&'this C as $trait<A>>::Output>;

            fn $method(self, other: CoordPair<A>) -> Self::Output {
                self.as_ref().zip_with(other, |this, other| this.$method(other))
            }
        }

        impl<'this, 'param, C, A> $trait<&'param CoordPair<A>>
            for &'this CoordPair<C>
        where
            &'this C: $trait<&'param A>,
        {
            type Output = CoordPair<<&'this C as $trait<&'param A>>::Output>;

            fn $method(self, other: &'param CoordPair<A>) -> Self::Output {
                self.as_ref()
                    .zip_with(other.as_ref(), |this, other| this.$method(other))
            }
        }
    };
}

macro_rules! elemwise_assign {
    ($trait:ident, $method:ident) => {
        impl<C, A> $trait<CoordPair<A>> for CoordPair<C>
        where
            C: $trait<A>,
        {
            fn $method(&mut self, other: CoordPair<A>) {
                self.y.$method(other.y);
                self.x.$method(other.x);
            }
        }

        impl<'param, C, A> $trait<&'param CoordPair<A>> for CoordPair<C>
        where
            C: $trait<&'param A>,
        {
            fn $method(&mut self, other: &'param CoordPair<A>) {
                for axis in Axis::all() {
                    self[axis].$method(&other[axis]);
                }
            }
        }

        impl<'this, C, A> $trait<CoordPair<A>> for &'this mut CoordPair<C>
        where
            for<'a> &'a mut C: $trait<A>,
        {
            fn $method(&mut self, other: CoordPair<A>) {
                (&mut self.y).$method(other.y);
                (&mut self.x).$method(other.x);
            }
        }

        impl<'this, 'param, C, A> $trait<&'param CoordPair<A>>
            for &'this mut CoordPair<C>
        where
            for<'a> &'a mut C: $trait<&'param A>,
        {
            fn $method(&mut self, other: &'param CoordPair<A>) {
                for axis in Axis::all() {
                    (&mut self[axis]).$method(&other[axis]);
                }
            }
        }
    };
}

elemwise_binop! { Add, add }
elemwise_assign! { AddAssign, add_assign }
elemwise_binop! { Sub, sub }
elemwise_assign! { SubAssign, sub_assign }
elemwise_binop! { Mul, mul }
elemwise_assign! { MulAssign, mul_assign }
elemwise_binop! { Div, div }
elemwise_assign! { DivAssign, div_assign }
elemwise_binop! { Rem, rem }
elemwise_assign! { RemAssign, rem_assign }

impl<C> Neg for CoordPair<C>
where
    C: Neg,
{
    type Output = CoordPair<C::Output>;

    fn neg(self) -> Self::Output {
        self.map(|elem| -elem)
    }
}

impl<'this, C> Neg for &'this CoordPair<C>
where
    &'this C: Neg,
{
    type Output = CoordPair<<&'this C as Neg>::Output>;

    fn neg(self) -> Self::Output {
        self.as_ref().map(|elem| -elem)
    }
}

macro_rules! elemwise_overflow {
    ($trait:ident, $method:ident) => {
        impl<C> $trait for CoordPair<C>
        where
            C: $trait,
        {
            fn $method(&self, other: &Self) -> Self {
                self.as_ref()
                    .zip_with(other.as_ref(), |this, other| this.$method(other))
            }
        }
    };
}

elemwise_overflow! { WrappingAdd, wrapping_add}
elemwise_overflow! { WrappingSub, wrapping_sub}
elemwise_overflow! { WrappingMul, wrapping_mul}
elemwise_overflow! { SaturatingAdd, saturating_add}
elemwise_overflow! { SaturatingSub, saturating_sub}
elemwise_overflow! { SaturatingMul, saturating_mul}

impl<C> WrappingNeg for CoordPair<C>
where
    C: WrappingNeg,
{
    fn wrapping_neg(&self) -> Self {
        self.as_ref().map(|elem| elem.wrapping_neg())
    }
}

macro_rules! elemwise_checked {
    ($trait:ident, $method:ident) => {
        impl<C> $trait for CoordPair<C>
        where
            C: $trait,
        {
            fn $method(&self, other: &Self) -> Option<Self> {
                self.as_ref()
                    .zip_with(other.as_ref(), |this, other| this.$method(other))
                    .transpose()
            }
        }
    };
}

elemwise_checked! { CheckedAdd, checked_add }
elemwise_checked! { CheckedSub, checked_sub }
elemwise_checked! { CheckedMul, checked_mul }
elemwise_checked! { CheckedDiv, checked_div }
elemwise_checked! { CheckedRem, checked_rem }

impl<C> CheckedNeg for CoordPair<C>
where
    C: CheckedNeg,
{
    fn checked_neg(&self) -> Option<Self> {
        self.as_ref().map(|elem| elem.checked_neg()).transpose()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum FromStrRadixErr<E> {
    MissingSep,
    BadCoord(Axis, E),
}

impl<E> fmt::Display for FromStrRadixErr<E>
where
    E: fmt::Display,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FromStrRadixErr::MissingSep => {
                fmt.write_str("Missing separator comma in coordinate pair")
            },
            FromStrRadixErr::BadCoord(axis, err) => {
                write!(fmt, "Parse error in axis {}: {}", axis, err)
            },
        }
    }
}

impl<E> Error for FromStrRadixErr<E> where E: Error {}

impl<C> Num for CoordPair<C>
where
    C: Num,
{
    type FromStrRadixErr = FromStrRadixErr<C::FromStrRadixErr>;

    fn from_str_radix(
        input: &str,
        radix: u32,
    ) -> Result<Self, Self::FromStrRadixErr> {
        use FromStrRadixErr::*;
        let index = input.find(',').ok_or(MissingSep)?;
        let str_x = &input[.. index].trim();
        let str_y = &input[index + 1 ..].trim();
        Ok(Self {
            x: C::from_str_radix(str_x, radix)
                .map_err(|err| BadCoord(Axis::X, err))?,
            y: C::from_str_radix(str_y, radix)
                .map_err(|err| BadCoord(Axis::Y, err))?,
        })
    }
}

impl<C> Unsigned for CoordPair<C> where C: Unsigned {}

impl<C> Signed for CoordPair<C>
where
    C: Signed,
{
    fn abs(&self) -> Self {
        self.as_ref().map(|elem| elem.abs())
    }

    fn abs_sub(&self, other: &Self) -> Self {
        self.as_ref()
            .zip_with(other.as_ref(), |this, other| this.abs_sub(other))
    }

    fn signum(&self) -> Self {
        self.as_ref().map(|elem| elem.signum())
    }

    fn is_positive(&self) -> bool {
        self.as_ref().fold(false, |elem, acc| acc || elem.is_positive())
    }

    fn is_negative(&self) -> bool {
        self.as_ref().fold(false, |elem, acc| acc || elem.is_negative())
    }
}

impl<C> CastSigned for CoordPair<C>
where
    C: CastSigned,
{
    type Target = CoordPair<C::Target>;

    fn cast_signed(&self) -> Self::Target {
        self.as_ref().map(|elem| elem.cast_signed())
    }
}

impl<C> CastUnsigned for CoordPair<C>
where
    C: CastUnsigned,
{
    type Target = CoordPair<C::Target>;

    fn cast_unsigned(&self) -> Self::Target {
        self.as_ref().map(|elem| elem.cast_unsigned())
    }
}

impl<C> Bounded for CoordPair<C>
where
    C: Bounded,
{
    fn min_value() -> Self {
        Self::from_axes(|_| C::min_value())
    }

    fn max_value() -> Self {
        Self::from_axes(|_| C::max_value())
    }
}
