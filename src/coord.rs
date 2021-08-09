//! Utilites related to coordinates.

#[cfg(test)]
mod test;

use crate::{
    axis::Axis,
    bits::{CastSigned, CastUnsigned, ExcessToSigned, HalfExcess},
    direc::{DirecVector, Direction},
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
    borrow::{Borrow, BorrowMut},
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
        Not,
        Rem,
        RemAssign,
        Sub,
        SubAssign,
    },
};

/// Generic 2D vector. It could be a coordinate, it could be size, anything like
/// that.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Vec2<T> {
    /// Value of the Y-coordinate.
    pub y: T,
    /// Value of the X-coordinate.
    pub x: T,
}

impl<T> Index<Axis> for Vec2<T> {
    type Output = T;

    fn index(&self, axis: Axis) -> &Self::Output {
        match axis {
            Axis::Y => &self.y,
            Axis::X => &self.x,
        }
    }
}

impl<T> IndexMut<Axis> for Vec2<T> {
    fn index_mut(&mut self, axis: Axis) -> &mut Self::Output {
        match axis {
            Axis::Y => &mut self.y,
            Axis::X => &mut self.x,
        }
    }
}

impl<T> Not for Vec2<T> {
    type Output = Vec2<T>;

    fn not(self) -> Self::Output {
        Self { x: self.y, y: self.x }
    }
}

impl<T> Vec2<T> {
    /// Creates a vector from a function over axis to data.
    pub fn from_axes<F>(mut mapper: F) -> Self
    where
        F: FnMut(Axis) -> T,
    {
        Self { y: mapper(Axis::Y), x: mapper(Axis::X) }
    }

    /// Maps coordinates to references.
    pub fn as_ref(&self) -> Vec2<&T> {
        Vec2 { x: &self.x, y: &self.y }
    }

    /// Maps coordinates to mutable references.
    pub fn as_mut(&mut self) -> Vec2<&mut T> {
        Vec2 { x: &mut self.x, y: &mut self.y }
    }

    /// Maps each coordinate to a given value and builds a new vector from the
    /// output of the mapping function.
    pub fn map<F, U>(self, mut mapper: F) -> Vec2<U>
    where
        F: FnMut(T) -> U,
    {
        Vec2 { y: mapper(self.y), x: mapper(self.x) }
    }

    /// Maps each coordinate to a given value in a new vector, but the mapping
    /// function gets the axis of each coordinate.
    pub fn map_with_axes<F, U>(self, mut mapper: F) -> Vec2<U>
    where
        F: FnMut(Axis, T) -> U,
    {
        Vec2 { y: mapper(Axis::Y, self.y), x: mapper(Axis::X, self.x) }
    }

    /// Performs a fold/reduce: i.e. accumulates the coordinates into a final
    /// result, given an initial value and an accumulator function, using first
    /// `x` and then `y`.
    pub fn fold<F, U>(self, init: U, mut folder: F) -> U
    where
        F: FnMut(T, U) -> U,
    {
        let acc = folder(self.x, init);
        folder(self.y, acc)
    }

    /// Performs a fold/reduce: i.e. accumulates the coordinates given initial
    /// value and accumulator function, but using first `y` the `x`.
    pub fn fold_rev<F, U>(self, init: U, mut folder: F) -> U
    where
        F: FnMut(U, T) -> U,
    {
        let acc = folder(init, self.y);
        folder(acc, self.x)
    }

    /// Zips two vectors into a vector of tuples. The result is:
    ///
    /// `zip([a,b], [c,d]) = [(a,c), (b,d)]`
    pub fn zip<U>(self, other: Vec2<U>) -> Vec2<(T, U)> {
        self.zip_with(other, |this, other| (this, other))
    }

    /// Zips two vectors using a zipper function `f`. The result is:
    ///
    /// `zip_with(f, [a,b], [c,d]) = [f(a,c), f(b,d)]`
    pub fn zip_with<F, U, B>(self, other: Vec2<U>, mut zipper: F) -> Vec2<B>
    where
        F: FnMut(T, U) -> B,
    {
        Vec2 { x: zipper(self.x, other.x), y: zipper(self.y, other.y) }
    }

    /// Borrows each coordinate.
    pub fn borrow<K>(&self) -> Vec2<&K>
    where
        T: Borrow<K>,
    {
        self.as_ref().into_borrow()
    }

    /// Borrows each coordinate as mutable references.
    pub fn borrow_mut<K>(&mut self) -> Vec2<&mut K>
    where
        T: BorrowMut<K>,
    {
        self.as_mut().into_borrow_mut()
    }
}

impl<'elems, T> Vec2<&'elems T> {
    /// Clones every coordinate.
    pub fn cloned(self) -> Vec2<T>
    where
        T: Clone,
    {
        self.map(Clone::clone)
    }

    /// Copies every coordinate.
    pub fn copied(self) -> Vec2<T>
    where
        T: Copy,
    {
        self.map(|&elem| elem)
    }

    /// Borrows each coordinate.
    pub fn into_borrow<K>(self) -> Vec2<&'elems K>
    where
        T: Borrow<K>,
    {
        self.map(Borrow::borrow)
    }
}

impl<'elems, T> Vec2<&'elems mut T> {
    /// Borrows each coordinate as mutable references.
    pub fn into_borrow_mut<K>(self) -> Vec2<&'elems mut K>
    where
        T: BorrowMut<K>,
    {
        self.map(BorrowMut::borrow_mut)
    }
}

impl<T> Vec2<T> {
    /// Computes the dot product of the vector, i.e. `x1*x2 + y1*y2`.
    ///
    /// # Examples
    /// ```
    /// # use gardiz::coord::Vec2;
    /// # fn main() {
    /// let left: Vec2<u16> = Vec2 { x: 5, y: 3 };
    /// let right = Vec2 { x: 4, y: 8 };
    /// assert_eq!(left.dot(right), 5 * 4 + 3 * 8);
    /// # }
    /// ```
    pub fn dot<U, A>(self, other: Vec2<U>) -> A::Output
    where
        T: Mul<U, Output = A>,
        A: Add,
    {
        let prod = self * other;
        prod.y + prod.x
    }

    /// Computes the dot product of the vector, by reference.
    pub fn dot_ref<'this, 'other, U, A>(
        &'this self,
        other: &'other Vec2<U>,
    ) -> A::Output
    where
        &'this T: Mul<&'other U, Output = A>,
        A: Add,
    {
        let prod = self.as_ref() * other.as_ref();
        prod.y + prod.x
    }

    /// Computes the dot product of the vector, wrapping around overflow.
    pub fn wrapping_dot(&self, other: &Self) -> T
    where
        T: WrappingAdd + WrappingMul,
    {
        let prod = self
            .as_ref()
            .zip_with(other.as_ref(), |this, other| this.wrapping_mul(other));
        prod.y.wrapping_add(&prod.x)
    }

    /// Computes the dot product of the vector, saturating when it overflows.
    pub fn saturating_dot(&self, other: &Self) -> T
    where
        T: SaturatingAdd + SaturatingMul,
    {
        let prod = self
            .as_ref()
            .zip_with(other.as_ref(), |this, other| this.saturating_mul(other));
        prod.y.saturating_add(&prod.x)
    }

    /// Computes the dot product of the vector, returning `None` if it
    /// overflows.
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

    /// Computes the square of the magnitude of the vector. The formula is: `x
    /// * x + y * y`.
    ///
    /// # Examples
    /// ```
    /// # use gardiz::coord::Vec2;
    /// # fn main() {
    /// let vector: Vec2<u16> = Vec2 { x: 5, y: 3 };
    /// assert_eq!(vector.sqr_magnitude(), 5 * 5 + 3 * 3);
    /// # }
    /// ```
    pub fn sqr_magnitude<A>(self) -> A::Output
    where
        T: Clone,
        T: Mul<Output = A>,
        A: Add,
    {
        self.clone().dot(self)
    }

    /// Computes the square of the magnitude of the vector by reference.
    pub fn sqr_magnitude_ref<'this, A>(&'this self) -> A::Output
    where
        &'this T: Mul<Output = A>,
        A: Add,
    {
        self.dot_ref(self)
    }

    /// Computes the square of the magnitude of the vector, wrapping on
    /// overflow.
    pub fn wrapping_sqr_mag(&self) -> T
    where
        T: WrappingAdd + WrappingMul,
    {
        self.wrapping_dot(self)
    }

    /// Computes the square of the magnitude of the vector, saturating on
    /// overflow.
    pub fn saturating_sqr_mag(&self) -> T
    where
        T: SaturatingMul + SaturatingAdd,
    {
        self.saturating_dot(self)
    }

    /// Computes the square of the magnitude of the vector, returning `None` on
    /// overflow.
    pub fn checked_sqr_mag(&self) -> Option<T>
    where
        T: CheckedMul + CheckedAdd,
    {
        self.checked_dot(self)
    }

    /// Computes the magnitude of a float vector. The formula is `sqrt(x*x +
    /// y*y)`.
    pub fn magnitude<A>(self) -> A::Output
    where
        T: Clone,
        T: Mul<Output = A>,
        A: Add,
        A::Output: Float,
    {
        self.sqr_magnitude().sqrt()
    }

    /// Computes the magnitude of a float vector by reference.
    pub fn magnitude_ref<'this, A>(&'this self) -> A::Output
    where
        &'this T: Mul<&'this T, Output = A>,
        A: Add,
        A::Output: Float,
    {
        self.sqr_magnitude_ref().sqrt()
    }

    /// Computes the magnitude of a float vector wrapping around on overflow.
    pub fn wrapping_mag(&self) -> T
    where
        T: WrappingAdd + WrappingMul + Float,
    {
        self.wrapping_dot(self).sqrt()
    }

    /// Computes the magnitude of a float vector saturating on overflow.
    pub fn saturating_mag(&self) -> T
    where
        T: SaturatingAdd + SaturatingMul + Float,
    {
        self.saturating_dot(self).sqrt()
    }

    /// Computes the magnitude of a float vector returning `None` on overflow.
    pub fn checked_mag(&self) -> Option<T>
    where
        T: CheckedAdd + CheckedMul + Float,
    {
        let squared = self.checked_dot(self)?;
        Some(squared.sqrt())
    }

    /// Computes the magnitude of the vector truncated (as an integer).
    ///
    /// # Examples
    /// ```
    /// # use gardiz::coord::Vec2;
    /// use num::integer::Roots;
    ///
    /// # fn main() {
    /// let vector: Vec2<u16> = Vec2 { x: 4, y: 3 };
    /// assert_eq!(vector.int_magnitude(), 5);
    /// # }
    /// ```
    pub fn int_magnitude<A>(self) -> A::Output
    where
        T: Clone,
        T: Mul<Output = A>,
        A: Add,
        A::Output: Roots,
    {
        self.sqr_magnitude().sqrt()
    }

    /// Computes the magnitude of the vector truncated (as an integer), by
    /// reference.
    pub fn int_magnitude_ref<'this, A>(&'this self) -> A::Output
    where
        &'this T: Mul<&'this T, Output = A>,
        A: Add,
        A::Output: Roots,
    {
        self.sqr_magnitude_ref().sqrt()
    }

    /// Computes the magnitude of the vector truncated (as an integer), wrapping
    /// around on overflow.
    pub fn wrapping_int_mag(&self) -> T
    where
        T: WrappingAdd + WrappingMul + Roots,
    {
        self.wrapping_sqr_mag().sqrt()
    }

    /// Computes the magnitude of the vector truncated (as an integer),
    /// saturating on overflow.
    pub fn saturating_int_mag(&self) -> T
    where
        T: SaturatingAdd + SaturatingMul + Roots,
    {
        self.saturating_sqr_mag().sqrt()
    }

    /// Computes the magnitude of the vector truncated (as an integer),
    /// returning `None` on overflow.
    pub fn checked_int_mag(&self) -> Option<T>
    where
        T: CheckedAdd + CheckedMul + Roots,
    {
        let squared = self.checked_sqr_mag()?;
        Some(squared.sqrt())
    }

    /// Moves this vector in the given direction by one.
    ///
    /// # Examples
    /// ```rust
    /// # use gardiz::coord::Vec2;
    /// use gardiz::direc::Direction;
    ///
    /// # fn main() {
    /// let vector: Vec2<u16> = Vec2 { x: 4, y: 3 };
    /// assert_eq!(vector.move_one(Direction::Up), Vec2 { x: 4, y: 2 });
    /// # }
    /// ```
    pub fn move_one(self, direction: Direction) -> Self
    where
        T: Add<Output = T> + Sub<Output = T> + One,
    {
        self.move_by(DirecVector { direction, magnitude: T::one() })
    }

    /// Moves this vector in the given direction by one, wrapping around on
    /// overflow.
    pub fn wrapping_move(self, direction: Direction) -> Self
    where
        T: WrappingAdd + WrappingSub + One,
    {
        self.wrapping_move_by(&DirecVector { direction, magnitude: T::one() })
    }

    /// Moves this vector in the given direction by one, saturating on
    /// overflow.
    pub fn saturating_move(self, direction: Direction) -> Self
    where
        T: SaturatingAdd + SaturatingSub + One,
    {
        self.saturating_move_by(&DirecVector { direction, magnitude: T::one() })
    }

    /// Moves this vector in the given direction by one, returning `None` on
    /// overflow.
    pub fn checked_move(self, direction: Direction) -> Option<Self>
    where
        T: CheckedAdd + CheckedSub + One,
    {
        self.checked_move_by(&DirecVector { direction, magnitude: T::one() })
    }

    /// Moves this vector in the given direction by the given amount.
    ///
    /// # Examples
    /// ```rust
    /// # use gardiz::coord::Vec2;
    /// use gardiz::direc::{Direction, DirecVector};
    ///
    /// # fn main() {
    /// let vector: Vec2<u16> = Vec2 { x: 4, y: 3 };
    /// let direc_vector = DirecVector { direction: Direction::Right, magnitude: 5 };
    /// assert_eq!(vector.move_by(direc_vector), Vec2 { x: 9, y: 3 });
    /// # }
    /// ```
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

    /// Moves this vector in the given direction by the given amount, wrapping
    /// around on overflow.
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

    /// Moves this vector in the given direction by the given amount, saturating
    /// on overflow.
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

    /// Moves this vector in the given direction by the given amount, returning
    /// `None` on overflow.
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

    /// Returns a "straight" direction into another point. Returns `None` if
    /// there is no straight direction between this point and the other point.
    pub fn direction_to(&self, other: &Self) -> Option<Direction>
    where
        T: Ord,
    {
        let cmping = self
            .as_ref()
            .zip_with(other.as_ref(), |this, other| this.cmp(&other));
        match cmping {
            Vec2 { x: Ordering::Equal, y: Ordering::Greater } => {
                Some(Direction::Up)
            },
            Vec2 { x: Ordering::Equal, y: Ordering::Less } => {
                Some(Direction::Down)
            },
            Vec2 { x: Ordering::Greater, y: Ordering::Equal } => {
                Some(Direction::Left)
            },
            Vec2 { x: Ordering::Less, y: Ordering::Equal } => {
                Some(Direction::Right)
            },
            _ => None,
        }
    }

    /// Useful for showing signed coordinates to humans, when the vector
    /// represents coordinates. Flips the Y coordinate, i.e. inverts the number
    /// line, the greatest value becomes the lesser, the lesser becomes the
    /// greatest, made for 2's complement numbers. The formula is something like
    /// this:
    ///
    /// `flip_y([x,y]) = [x, -1 - y]`
    pub fn flip_y(self) -> Self
    where
        T: Sub<Output = T> + Neg<Output = T> + One,
    {
        Self { y: -T::one() - self.y, ..self }
    }

    /// Useful for showing unsigned coordinates to humans, when the vector
    /// represents coordinates. First, it reinterprets the unsigned coordinats
    /// as "excess of N" numbers, such that `N` is the new origin, then it
    /// flips the Y coordinate. The formula is something like this:
    ///
    /// `center_origin_at([x,y], N) = [x - N, -1 - (y - N)]`
    ///
    /// # Examples
    /// ```rust
    /// # use gardiz::coord::Vec2;
    /// # fn main() {
    /// let vector: Vec2<u8> = Vec2 { x: 105, y: 97 };
    /// let centered: Vec2<i8> = Vec2 { x: 5, y: 102 };
    /// assert_eq!(
    ///     vector.center_origin_at(&Vec2 { x: 100, y: 200 }),
    ///     centered
    /// );
    /// # }
    /// ```
    pub fn center_origin_at<U>(self, origin: &Vec2<T>) -> Vec2<U>
    where
        Self: ExcessToSigned<Target = Vec2<U>>,
        U: Sub<Output = U> + Neg<Output = U> + One,
    {
        self.excess_to_signed(origin).flip_y()
    }

    /// Useful for showing unsigned coordinates to humans, when the vector
    /// represents coordinates. Like `center_origin_at`, but it uses half of the
    /// unsigned type's maximum value as the excess (with truncated division).
    /// First of all, half the maximum value becomes the new zero, then Y-axis
    /// is flipped. The formula is something like this:
    ///
    /// `center_origin_at([x,y]) = [x - 1000...0000, -1 - (y - 1000...0000)]`
    /// # Examples
    /// ```rust
    /// # use gardiz::coord::Vec2;
    /// # fn main() {
    /// let vector: Vec2<u8> = Vec2 { x: 127, y: 130 };
    /// let centered: Vec2<i8> = Vec2 { x: -1, y: -3 };
    /// assert_eq!(vector.center_origin(), centered);
    /// # }
    /// ```
    pub fn center_origin<U>(self) -> Vec2<U>
    where
        Self: ExcessToSigned<Target = Vec2<U>> + HalfExcess,
        U: Sub<Output = U> + Neg<Output = U> + One,
    {
        self.half_exc_to_signed().flip_y()
    }
}

impl<T> Vec2<Option<T>> {
    /// Transpose a vector of options into an option of vectors: a single
    /// coordinate with `None` makes the return value be `None`, while both
    /// being `Some` make it be `Some`.
    pub fn transpose(self) -> Option<Vec2<T>> {
        match (self.y, self.x) {
            (Some(y), Some(x)) => Some(Vec2 { y, x }),
            _ => None,
        }
    }
}

impl<T, E> Vec2<Result<T, E>> {
    /// Transpose a vector of `Result` into a `Result` of vectors: a single
    /// coordinate with `Err` makes the return value be `Err`, while both
    /// being `Ok` make it be `Ok`. An error on `x` gets priority over an error
    /// on `y`.
    pub fn transpose_err_x(self) -> Result<Vec2<T>, E> {
        match (self.y, self.x) {
            (Ok(y), Ok(x)) => Ok(Vec2 { y, x }),
            (_, Err(e)) | (Err(e), _) => Err(e),
        }
    }

    /// Transpose a vector of `Result` into a `Result` of vectors: a single
    /// coordinate with `Err` makes the return value be `Err`, while both
    /// being `Ok` make it be `Ok`. An error on `y` gets priority over an error
    /// on `x`.
    pub fn transpose_err_y(self) -> Result<Vec2<T>, E> {
        match (self.y, self.x) {
            (Ok(y), Ok(x)) => Ok(Vec2 { y, x }),
            (Err(e), _) | (_, Err(e)) => Err(e),
        }
    }
}

impl<T> Zero for Vec2<T>
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

impl<T> One for Vec2<T>
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
        impl<T, U> $trait<Vec2<U>> for Vec2<T>
        where
            T: $trait<U>,
        {
            type Output = Vec2<T::Output>;

            fn $method(self, other: Vec2<U>) -> Self::Output {
                self.zip_with(other, |this, other| this.$method(other))
            }
        }

        impl<'other, T, U> $trait<&'other Vec2<U>> for Vec2<T>
        where
            T: $trait<&'other U>,
        {
            type Output = Vec2<T::Output>;

            fn $method(self, other: &'other Vec2<U>) -> Self::Output {
                self.zip_with(other.as_ref(), |this, other| this.$method(other))
            }
        }

        impl<'this, T, U> $trait<Vec2<U>> for &'this Vec2<T>
        where
            &'this T: $trait<U>,
        {
            type Output = Vec2<<&'this T as $trait<U>>::Output>;

            fn $method(self, other: Vec2<U>) -> Self::Output {
                self.as_ref().zip_with(other, |this, other| this.$method(other))
            }
        }

        impl<'this, 'other, T, U> $trait<&'other Vec2<U>> for &'this Vec2<T>
        where
            &'this T: $trait<&'other U>,
        {
            type Output = Vec2<<&'this T as $trait<&'other U>>::Output>;

            fn $method(self, other: &'other Vec2<U>) -> Self::Output {
                self.as_ref().$method(other.as_ref())
            }
        }
    };
}

macro_rules! elemwise_assign {
    ($trait:ident, $method:ident) => {
        impl<T, U> $trait<Vec2<U>> for Vec2<T>
        where
            T: $trait<U>,
        {
            fn $method(&mut self, other: Vec2<U>) {
                self.y.$method(other.y);
                self.x.$method(other.x);
            }
        }

        impl<'param, T, U> $trait<&'param Vec2<U>> for Vec2<T>
        where
            T: $trait<&'param U>,
        {
            fn $method(&mut self, other: &'param Vec2<U>) {
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

impl<T> Neg for Vec2<T>
where
    T: Neg,
{
    type Output = Vec2<T::Output>;

    fn neg(self) -> Self::Output {
        self.map(|elem| -elem)
    }
}

impl<'this, T> Neg for &'this Vec2<T>
where
    &'this T: Neg,
{
    type Output = Vec2<<&'this T as Neg>::Output>;

    fn neg(self) -> Self::Output {
        self.as_ref().map(|elem| -elem)
    }
}

macro_rules! elemwise_overflow {
    ($trait:ident, $method:ident) => {
        impl<T> $trait for Vec2<T>
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

impl<T> WrappingNeg for Vec2<T>
where
    T: WrappingNeg,
{
    fn wrapping_neg(&self) -> Self {
        self.as_ref().map(|elem| elem.wrapping_neg())
    }
}

macro_rules! elemwise_checked {
    ($trait:ident, $method:ident) => {
        impl<T> $trait for Vec2<T>
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

impl<T> CheckedNeg for Vec2<T>
where
    T: CheckedNeg,
{
    fn checked_neg(&self) -> Option<Self> {
        self.as_ref().map(|elem| elem.checked_neg()).transpose()
    }
}

/// Error when parsing from string.
#[derive(Debug, Clone, Copy)]
pub enum FromStrRadixErr<E> {
    /// Thrown when a missing comma is found.
    MissingSep,
    /// A coordinate could not be parsed correctly as an internal error of the
    /// coordinate type.
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

impl<T> Num for Vec2<T>
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

impl<T> Unsigned for Vec2<T> where T: Unsigned {}

impl<T> Signed for Vec2<T>
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

impl<T> CastSigned for Vec2<T>
where
    T: CastSigned,
{
    type Target = Vec2<T::Target>;

    fn cast_signed(&self) -> Self::Target {
        self.as_ref().map(|elem| elem.cast_signed())
    }
}

impl<T> CastUnsigned for Vec2<T>
where
    T: CastUnsigned,
{
    type Target = Vec2<T::Target>;

    fn cast_unsigned(&self) -> Self::Target {
        self.as_ref().map(|elem| elem.cast_unsigned())
    }
}

impl<T> Bounded for Vec2<T>
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

impl<'this, T> From<&'this Vec2<T>> for Vec2<&'this T> {
    fn from(input: &'this Vec2<T>) -> Self {
        input.as_ref()
    }
}

impl<'this, T> From<&'this mut Vec2<T>> for Vec2<&'this mut T> {
    fn from(input: &'this mut Vec2<T>) -> Self {
        input.as_mut()
    }
}
