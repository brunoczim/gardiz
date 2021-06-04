#[cfg(test)]
mod test;

use crate::{
    axis::Axis,
    direc::{DirecVector, Direction},
    excess::{CastSigned, CastUnsigned, ExcessToSigned, HalfExcess},
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
        Pow,
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
    cmp::Ordering,
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
pub struct CoordPair<T> {
    pub y: T,
    pub x: T,
}

impl<T> Index<Axis> for CoordPair<T> {
    type Output = T;

    fn index(&self, axis: Axis) -> &Self::Output {
        match axis {
            Axis::Y => &self.y,
            Axis::X => &self.x,
        }
    }
}

impl<T> IndexMut<Axis> for CoordPair<T> {
    fn index_mut(&mut self, axis: Axis) -> &mut Self::Output {
        match axis {
            Axis::Y => &mut self.y,
            Axis::X => &mut self.x,
        }
    }
}

impl<T> CoordPair<T> {
    pub fn from_axes<F>(mut mapper: F) -> Self
    where
        F: FnMut(Axis) -> T,
    {
        Self { y: mapper(Axis::Y), x: mapper(Axis::X) }
    }

    pub fn as_ref(&self) -> CoordPair<&T> {
        CoordPair { x: &self.x, y: &self.y }
    }

    pub fn as_mut(&mut self) -> CoordPair<&mut T> {
        CoordPair { x: &mut self.x, y: &mut self.y }
    }

    pub fn map_with_axes<F, U>(self, mut mapper: F) -> CoordPair<U>
    where
        F: FnMut(Axis, T) -> U,
    {
        CoordPair { y: mapper(Axis::Y, self.y), x: mapper(Axis::X, self.x) }
    }

    pub fn map<F, U>(self, mut mapper: F) -> CoordPair<U>
    where
        F: FnMut(T) -> U,
    {
        CoordPair { y: mapper(self.y), x: mapper(self.x) }
    }

    pub fn fold<F, U>(self, init: U, mut folder: F) -> U
    where
        F: FnMut(T, U) -> U,
    {
        let acc = folder(self.x, init);
        folder(self.y, acc)
    }

    pub fn fold_rev<F, U>(self, init: U, mut folder: F) -> U
    where
        F: FnMut(U, T) -> U,
    {
        let acc = folder(init, self.y);
        folder(acc, self.x)
    }

    pub fn zip<U>(self, other: CoordPair<U>) -> CoordPair<(T, U)> {
        self.zip_with(other, |this, other| (this, other))
    }

    pub fn zip_with<F, U, B>(
        self,
        other: CoordPair<U>,
        mut zipper: F,
    ) -> CoordPair<B>
    where
        F: FnMut(T, U) -> B,
    {
        CoordPair { x: zipper(self.x, other.x), y: zipper(self.y, other.y) }
    }
}

impl<'elems, T> CoordPair<&'elems T> {
    pub fn cloned(self) -> CoordPair<T>
    where
        T: Clone,
    {
        self.map(Clone::clone)
    }

    pub fn copied(self) -> CoordPair<T>
    where
        T: Copy,
    {
        self.map(|&elem| elem)
    }
}

impl<T> CoordPair<T> {
    pub fn dot<U, A>(self, other: CoordPair<U>) -> A::Output
    where
        T: Mul<U, Output = A>,
        A: Add,
    {
        let prod = self * other;
        prod.y + prod.x
    }

    pub fn dot_ref<'this, 'other, U, A>(
        &'this self,
        other: &'other CoordPair<U>,
    ) -> A::Output
    where
        &'this T: Mul<&'other U, Output = A>,
        A: Add,
    {
        let prod = self.as_ref() * other.as_ref();
        prod.y + prod.x
    }

    pub fn wrapping_dot(&self, other: &Self) -> T
    where
        T: WrappingAdd + WrappingMul,
    {
        let prod = self
            .as_ref()
            .zip_with(other.as_ref(), |this, other| this.wrapping_mul(other));
        prod.y.wrapping_add(&prod.x)
    }

    pub fn saturating_dot(&self, other: &Self) -> T
    where
        T: SaturatingAdd + SaturatingMul,
    {
        let prod = self
            .as_ref()
            .zip_with(other.as_ref(), |this, other| this.saturating_mul(other));
        prod.y.saturating_add(&prod.x)
    }

    pub fn checked_dot(&self, other: &Self) -> Option<T>
    where
        T: CheckedAdd + CheckedMul,
    {
        let prod = self
            .as_ref()
            .zip_with(other.as_ref(), |this, other| this.checked_mul(other))
            .transpose()?;
        prod.y.checked_add(&prod.x)
    }

    pub fn sqr_magnitude<A>(self) -> A::Output
    where
        T: Clone,
        T: Mul<Output = A>,
        A: Add,
    {
        self.clone().dot(self)
    }

    pub fn sqr_magnitude_ref<'this, A>(&'this self) -> A::Output
    where
        &'this T: Mul<Output = A>,
        A: Add,
    {
        self.dot_ref(self)
    }

    pub fn wrapping_sqr_mag(&self) -> T
    where
        T: WrappingAdd + WrappingMul,
    {
        self.wrapping_dot(self)
    }

    pub fn saturating_sqr_mag(&self) -> T
    where
        T: SaturatingMul + SaturatingAdd,
    {
        self.saturating_dot(self)
    }

    pub fn checked_sqr_mag(&self) -> Option<T>
    where
        T: CheckedMul + CheckedAdd,
    {
        self.checked_dot(self)
    }

    pub fn magnitude<A>(self) -> A::Output
    where
        T: Clone,
        T: Mul<Output = A>,
        A: Add,
        A::Output: Float,
    {
        self.sqr_magnitude().sqrt()
    }

    pub fn magnitude_ref<'this, A>(&'this self) -> A::Output
    where
        &'this T: Mul<&'this T, Output = A>,
        A: Add,
        A::Output: Float,
    {
        self.sqr_magnitude_ref().sqrt()
    }

    pub fn wrapping_mag(&self) -> T
    where
        T: WrappingAdd + WrappingMul + Float,
    {
        self.wrapping_dot(self).sqrt()
    }

    pub fn saturating_mag(&self) -> T
    where
        T: SaturatingAdd + SaturatingMul + Float,
    {
        self.saturating_dot(self).sqrt()
    }

    pub fn checked_mag(&self) -> Option<T>
    where
        T: CheckedAdd + CheckedMul + Float,
    {
        let squared = self.checked_dot(self)?;
        Some(squared.sqrt())
    }

    pub fn int_magnitude<A>(self) -> A::Output
    where
        T: Clone,
        T: Mul<Output = A>,
        A: Add,
        A::Output: Roots,
    {
        self.sqr_magnitude().sqrt()
    }

    pub fn int_magnitude_ref<'this, A>(&'this self) -> A::Output
    where
        &'this T: Mul<&'this T, Output = A>,
        A: Add,
        A::Output: Roots,
    {
        self.sqr_magnitude_ref().sqrt()
    }

    pub fn wrapping_int_mag(&self) -> T
    where
        T: WrappingAdd + WrappingMul + Roots,
    {
        self.wrapping_sqr_mag().sqrt()
    }

    pub fn saturating_int_mag(&self) -> T
    where
        T: SaturatingAdd + SaturatingMul + Roots,
    {
        self.saturating_sqr_mag().sqrt()
    }

    pub fn checked_int_mag(&self) -> Option<T>
    where
        T: CheckedAdd + CheckedMul + Roots,
    {
        let squared = self.checked_sqr_mag()?;
        Some(squared.sqrt())
    }

    pub fn move_one(self, direction: Direction) -> Self
    where
        T: Add<Output = T> + Sub<Output = T> + One,
    {
        self.move_by(DirecVector { direction, magnitude: T::one() })
    }

    pub fn wrapping_move(self, direction: Direction) -> Self
    where
        T: WrappingAdd + WrappingSub + One,
    {
        self.wrapping_move_by(&DirecVector { direction, magnitude: T::one() })
    }

    pub fn saturating_move(self, direction: Direction) -> Self
    where
        T: SaturatingAdd + SaturatingSub + One,
    {
        self.saturating_move_by(&DirecVector { direction, magnitude: T::one() })
    }

    pub fn checked_move(self, direction: Direction) -> Option<Self>
    where
        T: CheckedAdd + CheckedSub + One,
    {
        self.checked_move_by(&DirecVector { direction, magnitude: T::one() })
    }

    pub fn direction_to(&self, other: &Self) -> Option<Direction>
    where
        T: Ord,
    {
        let cmping = self
            .as_ref()
            .zip_with(other.as_ref(), |this, other| this.cmp(&other));
        match cmping {
            CoordPair { x: Ordering::Equal, y: Ordering::Greater } => {
                Some(Direction::Up)
            },
            CoordPair { x: Ordering::Equal, y: Ordering::Less } => {
                Some(Direction::Down)
            },
            CoordPair { x: Ordering::Greater, y: Ordering::Equal } => {
                Some(Direction::Left)
            },
            CoordPair { x: Ordering::Less, y: Ordering::Equal } => {
                Some(Direction::Right)
            },
            _ => None,
        }
    }

    pub fn move_by<U>(self, vector: DirecVector<U>) -> Self
    where
        T: Add<U, Output = T> + Sub<U, Output = T>,
    {
        match vector.direction {
            Direction::Up => Self { y: self.y - vector.magnitude, ..self },
            Direction::Down => Self { y: self.y + vector.magnitude, ..self },
            Direction::Left => Self { x: self.x - vector.magnitude, ..self },
            Direction::Right => Self { x: self.x + vector.magnitude, ..self },
        }
    }

    pub fn wrapping_move_by(self, vector: &DirecVector<T>) -> Self
    where
        T: WrappingAdd + WrappingSub,
    {
        match vector.direction {
            Direction::Up => {
                Self { y: self.y.wrapping_sub(&vector.magnitude), ..self }
            },
            Direction::Down => {
                Self { y: self.y.wrapping_add(&vector.magnitude), ..self }
            },
            Direction::Left => {
                Self { x: self.x.wrapping_sub(&vector.magnitude), ..self }
            },
            Direction::Right => {
                Self { x: self.x.wrapping_add(&vector.magnitude), ..self }
            },
        }
    }

    pub fn saturating_move_by(self, vector: &DirecVector<T>) -> Self
    where
        T: SaturatingAdd + SaturatingSub,
    {
        match vector.direction {
            Direction::Up => {
                Self { y: self.y.saturating_sub(&vector.magnitude), ..self }
            },
            Direction::Down => {
                Self { y: self.y.saturating_add(&vector.magnitude), ..self }
            },
            Direction::Left => {
                Self { x: self.x.saturating_sub(&vector.magnitude), ..self }
            },
            Direction::Right => {
                Self { x: self.x.saturating_add(&vector.magnitude), ..self }
            },
        }
    }

    pub fn checked_move_by(self, vector: &DirecVector<T>) -> Option<Self>
    where
        T: CheckedAdd + CheckedSub,
    {
        let result = match vector.direction {
            Direction::Up => {
                Self { y: self.y.checked_sub(&vector.magnitude)?, ..self }
            },
            Direction::Down => {
                Self { y: self.y.checked_add(&vector.magnitude)?, ..self }
            },
            Direction::Left => {
                Self { x: self.x.checked_sub(&vector.magnitude)?, ..self }
            },
            Direction::Right => {
                Self { x: self.x.checked_add(&vector.magnitude)?, ..self }
            },
        };

        Some(result)
    }

    pub fn flip_y(self) -> Self
    where
        T: Sub<Output = T> + Neg<Output = T> + One,
    {
        Self { y: -T::one() - self.y, ..self }
    }

    pub fn center_origin_at<U>(self, origin: &CoordPair<T>) -> CoordPair<U>
    where
        Self: ExcessToSigned<Target = CoordPair<U>>,
        U: Sub<Output = U> + Neg<Output = U> + One,
    {
        self.excess_to_signed(origin).flip_y()
    }

    pub fn center_origin<U>(self) -> CoordPair<U>
    where
        Self: ExcessToSigned<Target = CoordPair<U>> + HalfExcess,
        U: Sub<Output = U> + Neg<Output = U> + One,
    {
        self.half_exc_to_signed().flip_y()
    }
}

impl<T> CoordPair<Option<T>> {
    pub fn transpose(self) -> Option<CoordPair<T>> {
        match (self.y, self.x) {
            (Some(y), Some(x)) => Some(CoordPair { y, x }),
            _ => None,
        }
    }
}

impl<T> Zero for CoordPair<T>
where
    T: Zero,
{
    fn zero() -> Self {
        Self::from_axes(|_| T::zero())
    }

    fn is_zero(&self) -> bool {
        self.as_ref().fold(true, |elem, acc| acc && elem.is_zero())
    }

    fn set_zero(&mut self) {
        for axis in Axis::iter() {
            self[axis].set_zero();
        }
    }
}

impl<T> One for CoordPair<T>
where
    T: One,
{
    fn one() -> Self {
        Self::from_axes(|_| T::one())
    }

    fn set_one(&mut self) {
        for axis in Axis::iter() {
            self[axis].set_one();
        }
    }
}

macro_rules! elemwise_binop {
    ($trait:ident, $method:ident) => {
        impl<T, U> $trait<CoordPair<U>> for CoordPair<T>
        where
            T: $trait<U>,
        {
            type Output = CoordPair<T::Output>;

            fn $method(self, other: CoordPair<U>) -> Self::Output {
                self.zip_with(other, |this, other| this.$method(other))
            }
        }

        impl<'other, T, U> $trait<&'other CoordPair<U>> for CoordPair<T>
        where
            T: $trait<&'other U>,
        {
            type Output = CoordPair<T::Output>;

            fn $method(self, other: &'other CoordPair<U>) -> Self::Output {
                self.zip_with(other.as_ref(), |this, other| this.$method(other))
            }
        }

        impl<'this, T, U> $trait<CoordPair<U>> for &'this CoordPair<T>
        where
            &'this T: $trait<U>,
        {
            type Output = CoordPair<<&'this T as $trait<U>>::Output>;

            fn $method(self, other: CoordPair<U>) -> Self::Output {
                self.as_ref().zip_with(other, |this, other| this.$method(other))
            }
        }

        impl<'this, 'other, T, U> $trait<&'other CoordPair<U>>
            for &'this CoordPair<T>
        where
            &'this T: $trait<&'other U>,
        {
            type Output = CoordPair<<&'this T as $trait<&'other U>>::Output>;

            fn $method(self, other: &'other CoordPair<U>) -> Self::Output {
                self.as_ref().$method(other.as_ref())
            }
        }
    };
}

macro_rules! elemwise_assign {
    ($trait:ident, $method:ident) => {
        impl<T, U> $trait<CoordPair<U>> for CoordPair<T>
        where
            T: $trait<U>,
        {
            fn $method(&mut self, other: CoordPair<U>) {
                self.y.$method(other.y);
                self.x.$method(other.x);
            }
        }

        impl<'param, T, U> $trait<&'param CoordPair<U>> for CoordPair<T>
        where
            T: $trait<&'param U>,
        {
            fn $method(&mut self, other: &'param CoordPair<U>) {
                for axis in Axis::iter() {
                    self[axis].$method(&other[axis]);
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
elemwise_binop! { Pow, pow }

impl<T> Neg for CoordPair<T>
where
    T: Neg,
{
    type Output = CoordPair<T::Output>;

    fn neg(self) -> Self::Output {
        self.map(|elem| -elem)
    }
}

impl<'this, T> Neg for &'this CoordPair<T>
where
    &'this T: Neg,
{
    type Output = CoordPair<<&'this T as Neg>::Output>;

    fn neg(self) -> Self::Output {
        self.as_ref().map(|elem| -elem)
    }
}

macro_rules! elemwise_overflow {
    ($trait:ident, $method:ident) => {
        impl<T> $trait for CoordPair<T>
        where
            T: $trait,
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

impl<T> WrappingNeg for CoordPair<T>
where
    T: WrappingNeg,
{
    fn wrapping_neg(&self) -> Self {
        self.as_ref().map(|elem| elem.wrapping_neg())
    }
}

macro_rules! elemwise_checked {
    ($trait:ident, $method:ident) => {
        impl<T> $trait for CoordPair<T>
        where
            T: $trait,
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

impl<T> CheckedNeg for CoordPair<T>
where
    T: CheckedNeg,
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

impl<T> Num for CoordPair<T>
where
    T: Num,
{
    type FromStrRadixErr = FromStrRadixErr<T::FromStrRadixErr>;

    fn from_str_radix(
        input: &str,
        radix: u32,
    ) -> Result<Self, Self::FromStrRadixErr> {
        use FromStrRadixErr::*;
        let index = input.find(',').ok_or(MissingSep)?;
        let str_x = &input[.. index].trim();
        let str_y = &input[index + 1 ..].trim();
        Ok(Self {
            x: T::from_str_radix(str_x, radix)
                .map_err(|err| BadCoord(Axis::X, err))?,
            y: T::from_str_radix(str_y, radix)
                .map_err(|err| BadCoord(Axis::Y, err))?,
        })
    }
}

impl<T> Unsigned for CoordPair<T> where T: Unsigned {}

impl<T> Signed for CoordPair<T>
where
    T: Signed,
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

impl<T> CastSigned for CoordPair<T>
where
    T: CastSigned,
{
    type Target = CoordPair<T::Target>;

    fn cast_signed(&self) -> Self::Target {
        self.as_ref().map(|elem| elem.cast_signed())
    }
}

impl<T> CastUnsigned for CoordPair<T>
where
    T: CastUnsigned,
{
    type Target = CoordPair<T::Target>;

    fn cast_unsigned(&self) -> Self::Target {
        self.as_ref().map(|elem| elem.cast_unsigned())
    }
}

impl<T> Bounded for CoordPair<T>
where
    T: Bounded,
{
    fn min_value() -> Self {
        Self::from_axes(|_| T::min_value())
    }

    fn max_value() -> Self {
        Self::from_axes(|_| T::max_value())
    }
}
