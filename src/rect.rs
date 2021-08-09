//! This module exports rectangle utilities.

#[cfg(test)]
mod test;

use crate::{axis::Axis, coord::Vec2};
use num::traits::{
    CheckedAdd,
    CheckedSub,
    One,
    SaturatingAdd,
    SaturatingSub,
    WrappingAdd,
    WrappingSub,
    Zero,
};
use std::ops::{Add, AddAssign, Sub, SubAssign};

/// A rectangle in a plane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Rect<T, S = T> {
    /// Starting top-left point.
    pub start: Vec2<T>,
    /// Size at each dimension.
    pub size: Vec2<S>,
}

impl<T, S> Rect<T, S> {
    /// Builds the rectangle from the given range `start .. end` (i.e. end
    /// excluded).
    ///
    /// # Examples
    /// ```rust
    /// use gardiz::{rect::Rect, coord::Vec2};
    ///
    /// # fn main() {
    /// let built_from_range = Rect::<u16>::from_range(
    ///     Vec2 { x: 5, y: 3 },
    ///     Vec2 { x: 7, y: 9 },
    /// );
    /// let actual = Rect {
    ///     start: Vec2 { x: 5, y: 3 },
    ///     size: Vec2 { x: 2, y: 6 },
    /// };
    /// assert_eq!(built_from_range, actual);
    /// # }
    /// ```
    pub fn from_range<U>(start: Vec2<T>, end: Vec2<U>) -> Self
    where
        T: Clone,
        U: Sub<T, Output = S>,
    {
        let size = end - start.clone();
        Self { start, size }
    }
}

impl<T> Rect<T> {
    /// Tries to make a rectangle from a given range (end excluded), and returns
    /// `None` if overflows.
    pub fn try_from_range(start: Vec2<T>, end: Vec2<T>) -> Option<Self>
    where
        T: Clone + CheckedSub,
    {
        let size = end.checked_sub(&start)?;
        Some(Self { start, size })
    }
}

impl<T, S> Rect<T, S> {
    /// Builds the rectangle from the given inclusive range `start ..= end`
    /// (i.e. end included).
    ///
    /// # Examples
    /// ```rust
    /// use gardiz::{rect::Rect, coord::Vec2};
    ///
    /// # fn main() {
    /// let built_from_range = Rect::<u16>::from_range_incl(
    ///     Vec2 { x: 5, y: 3 },
    ///     Vec2 { x: 6, y: 8 },
    /// );
    /// let actual = Rect {
    ///     start: Vec2 { x: 5, y: 3 },
    ///     size: Vec2 { x: 2, y: 6 },
    /// };
    /// assert_eq!(built_from_range, actual);
    /// # }
    /// ```
    pub fn from_range_incl<Z>(start: Vec2<T>, end: Vec2<T>) -> Self
    where
        T: Sub<Output = Z> + Clone + Ord,
        Z: One + Add<Output = S>,
        S: Zero,
    {
        let size = if end < start {
            Vec2::<S>::zero()
        } else {
            end - start.clone() + Vec2::<Z>::one()
        };
        Self { start, size }
    }
}

impl<T> Rect<T> {
    /// Tries to make a rectangle from a given range (end included), and returns
    /// `None` if overflows.
    pub fn try_from_range_incl(start: Vec2<T>, end: Vec2<T>) -> Option<Self>
    where
        T: CheckedAdd + CheckedSub + One + Zero + Ord + Clone,
    {
        let size = if end < start {
            Vec2::<T>::zero()
        } else {
            let diff = end.checked_sub(&start.clone())?;
            diff.checked_add(&Vec2::<T>::one())?
        };
        Some(Self { start, size })
    }
}

impl<T, S> Rect<T, S> {
    /// Returns whether the rectangle is empty (i.e. size is zero).
    pub fn is_empty(&self) -> bool
    where
        S: Zero,
    {
        self.size.x.is_zero() || self.size.y.is_zero()
    }

    /// Returns coordinates one unit past the end (bottom-right) of the
    /// rectangle, i.e. the end excluded from the rectangle.
    ///
    /// # Examples
    /// ```rust
    /// use gardiz::{rect::Rect, coord::Vec2};
    ///
    /// # fn main() {
    /// let rectangle: Rect<u16> = Rect {
    ///     start: Vec2 { x: 5, y: 3 },
    ///     size: Vec2 { x: 2, y: 6 },
    /// };
    /// assert_eq!(rectangle.end(), Vec2 { x: 7, y: 9 });
    /// # }
    /// ```
    pub fn end(self) -> Vec2<T::Output>
    where
        T: Add<S>,
    {
        self.start + self.size
    }
}

impl<T> Rect<T> {
    /// Returns coordinates one unit past the end (bottom-right), wrapping
    /// around on overflow.
    pub fn wrapping_end(&self) -> Vec2<T>
    where
        T: WrappingAdd,
    {
        self.start.wrapping_add(&self.size)
    }

    /// Returns coordinates one unit past the end (bottom-right), saturating on
    /// overflow.
    pub fn saturating_end(&self) -> Vec2<T>
    where
        T: SaturatingAdd,
    {
        self.start.saturating_add(&self.size)
    }

    /// Returns coordinates one unit past the end (bottom-right), returning
    /// `None` on overflow.
    pub fn checked_end(&self) -> Option<Vec2<T>>
    where
        T: CheckedAdd,
    {
        self.start.checked_add(&self.size)
    }
}

impl<T, S> Rect<T, S> {
    /// Returns coordinates one unit past the end (bottom-right), but without
    /// taking the rectangle (by reference).
    pub fn end_ref<'this, U>(&'this self) -> Vec2<U>
    where
        &'this T: Add<&'this S, Output = U>,
    {
        &self.start + &self.size
    }

    /// Returns the last coordinates (bottom-right) of the rectangle, i.e.
    /// returns an included end.
    ///
    /// # Examples
    /// ```rust
    /// use gardiz::{rect::Rect, coord::Vec2};
    ///
    /// # fn main() {
    /// let rectangle: Rect<u16> = Rect {
    ///     start: Vec2 { x: 5, y: 3 },
    ///     size: Vec2 { x: 2, y: 6 },
    /// };
    /// assert_eq!(rectangle.end_inclusive(), Vec2 { x: 6, y: 8 });
    /// # }
    /// ```
    pub fn end_inclusive<U, V>(self) -> Vec2<V>
    where
        S: Sub<Output = U> + One + Zero,
        T: Add<U, Output = V> + Sub<Output = V> + One,
    {
        if self.is_empty() {
            self.start - Vec2::<T>::one()
        } else {
            self.start + (self.size - Vec2::<S>::one())
        }
    }
}

impl<T> Rect<T> {
    /// Returns the last coordinates (bottom-right) of the rectangle, wrapping
    /// around on overflow.
    pub fn wrapping_end_incl(&self) -> Vec2<T>
    where
        T: WrappingAdd + WrappingSub + One,
    {
        self.start.wrapping_add(&self.size).wrapping_sub(&Vec2::<T>::one())
    }

    /// Returns the last coordinates (bottom-right) of the rectangle, saturating
    /// on overflow.
    pub fn saturating_end_incl(&self) -> Vec2<T>
    where
        T: SaturatingAdd + SaturatingSub + One + Zero,
    {
        if self.is_empty() {
            self.start.saturating_sub(&Vec2::<T>::one())
        } else {
            let last_index = self.size.saturating_sub(&Vec2::<T>::one());
            self.start.saturating_add(&last_index)
        }
    }

    /// Returns the last coordinates (bottom-right) of the rectangle, returning
    /// `None` on overflow.
    pub fn checked_end_incl(&self) -> Option<Vec2<T>>
    where
        T: CheckedAdd + CheckedSub + One + Zero,
    {
        if self.is_empty() {
            self.start.checked_sub(&Vec2::<T>::one())
        } else {
            let last_index = self.size.checked_sub(&Vec2::<T>::one())?;
            self.start.checked_add(&&last_index)
        }
    }
}

impl<T, S> Rect<T, S> {
    /// Returns the last coordinates (bottom-right) of the rectangle, but
    /// without taking the rectangle (by reference).
    pub fn end_incl_ref<'this, U, V>(&'this self) -> Vec2<V>
    where
        &'this S: Sub<S, Output = U>,
        S: One + Zero,
        &'this T: Add<U, Output = V> + Sub<T, Output = V>,
        T: One,
    {
        if self.size.is_zero() {
            &self.start - Vec2::<T>::one()
        } else {
            &self.start + (&self.size - Vec2::<S>::one())
        }
    }

    /// Returns last included coordinates of the rectangle (bottom-right),
    /// but if the rectangle is empty, the output is `None`. With this, it is
    /// possible to extract the end of a "full rectangle".
    ///
    /// # Examples
    ///
    /// ## Actually Non-Empty
    /// ```rust
    /// use gardiz::{rect::Rect, coord::Vec2};
    ///
    /// # fn main() {
    /// let rectangle: Rect<u16> = Rect {
    ///     start: Vec2 { x: 5, y: 3 },
    ///     size: Vec2 { x: 2, y: 6 },
    /// };
    /// assert_eq!(rectangle.end_non_empty(), Some(Vec2 { x: 6, y: 8 }));
    /// # }
    /// ```
    ///
    /// ## Empty
    /// ```rust
    /// use gardiz::{rect::Rect, coord::Vec2};
    ///
    /// # fn main() {
    /// let rectangle: Rect<u16> = Rect {
    ///     start: Vec2 { x: 5, y: 3 },
    ///     size: Vec2 { x: 0, y: 6 },
    /// };
    /// assert_eq!(rectangle.end_non_empty(), None);
    /// # }
    /// ```
    pub fn end_non_empty<U, V>(self) -> Option<Vec2<V>>
    where
        S: Sub<Output = U> + One + Zero,
        T: Add<U, Output = V> + Sub<Output = V> + One,
    {
        if self.is_empty() {
            None
        } else {
            Some(self.start + (self.size - Vec2::<S>::one()))
        }
    }

    /// Returns last included coordinates of the rectangle (bottom-right), but
    /// if the rectangle is empty, the output is `None`, and without taking the
    /// rectangle, computing by reference instead.
    pub fn end_non_empty_ref<'this, U, V>(&'this self) -> Option<Vec2<V>>
    where
        &'this S: Sub<S, Output = U>,
        S: One + Zero,
        &'this T: Add<U, Output = V> + Sub<T, Output = V>,
        T: One,
    {
        if self.size.is_zero() {
            None
        } else {
            Some(&self.start + (&self.size - Vec2::<S>::one()))
        }
    }

    /// Tests whether a given point is inside the rectangle.
    pub fn has_point<'this, U>(&'this self, point: Vec2<T>) -> bool
    where
        &'this S: Sub<S, Output = U>,
        S: One + Zero,
        &'this T: Add<U, Output = T> + Sub<T, Output = T>,
        T: Sub<&'this T> + One + Ord,
    {
        let maybe_end = self.end_non_empty_ref();
        maybe_end.map_or(false, |end| {
            Axis::iter().all(|axis| {
                let this_less = self.start[axis] <= point[axis];
                let other_less = point[axis] <= end[axis];
                this_less && other_less
            })
        })
    }

    /// Tests whether two rectangles overlap in area.
    pub fn overlaps<'params, U>(&'params self, other: &'params Self) -> bool
    where
        &'params S: Sub<S, Output = U>,
        S: One + Zero,
        &'params T: Add<U, Output = T> + Sub<T, Output = T>,
        T: Sub<&'params T> + One + Ord,
    {
        let maybe_ends =
            self.end_non_empty_ref().zip(other.end_non_empty_ref());

        maybe_ends.map_or(false, |(this_end, other_end)| {
            Axis::iter().all(|axis| {
                let this_less = self.start[axis] <= other_end[axis];
                let other_less = other.start[axis] <= this_end[axis];
                this_less && other_less
            })
        })
    }

    /// Computes the overlapped area between two rectangles. An empty rectange
    /// is produced if both do not overlap.
    ///
    /// # Examples
    /// ```rust
    /// use gardiz::{rect::Rect, coord::Vec2};
    ///
    /// # fn main() {
    /// let left: Rect<u16> = Rect {
    ///     start: Vec2 { x: 5, y: 3 },
    ///     size: Vec2 { x: 10, y: 7 },
    /// };
    /// let right = Rect {
    ///     start: Vec2 { x: 9, y: 4 },
    ///     size: Vec2 { x: 9, y: 5 },
    /// };
    /// let overlapped = Rect {
    ///     start: Vec2 { x: 9, y: 4 },
    ///     size: Vec2 { x: 6, y: 5 },
    /// };
    /// assert_eq!(left.overlapped(&right), overlapped);
    /// # }
    /// ```
    pub fn overlapped<'params, U, Z>(
        &'params self,
        other: &'params Self,
    ) -> Rect<T, <Z as Add>::Output>
    where
        &'params S: Sub<S, Output = U>,
        S: One + Zero,
        &'params T: Add<U, Output = T> + Sub<T, Output = T>,
        T: Sub<&'params T, Output = Z> + One + Ord + Clone,
        Z: One + Add,
    {
        let start =
            self.start.as_ref().zip_with(other.start.as_ref(), Ord::max);
        let end = self.end_incl_ref().zip_with(other.end_incl_ref(), Ord::min);
        let size = end.zip_with(start, |end, start| end - start + Z::one());
        Rect { start: start.cloned(), size }
    }
}

impl<T> Rect<T> {
    /// Computes overlapped area between two rectangles, wrapping around on
    /// overflow.
    pub fn wrapping_overlapped<'params>(
        &'params self,
        other: &'params Self,
    ) -> Self
    where
        T: WrappingAdd + WrappingSub + Ord + Clone,
    {
        let start =
            self.start.as_ref().zip_with(other.start.as_ref(), Ord::max);
        let end = self.wrapping_end().zip(other.wrapping_end()).zip_with(
            start,
            |(this, other), start| {
                if this >= *start && other < *start {
                    this
                } else if other >= *start && this < *start {
                    other
                } else {
                    this.min(other)
                }
            },
        );
        let size = end.zip_with(start, |end, start| end.wrapping_sub(start));
        Rect { start: start.cloned(), size }
    }

    /// Computes overlapped area between two rectangles, saturating on overflow.
    pub fn saturating_overlapped<'params>(
        &'params self,
        other: &'params Self,
    ) -> Self
    where
        T: SaturatingAdd + SaturatingSub + One + Zero + Ord + Clone,
    {
        let start =
            self.start.as_ref().zip_with(other.start.as_ref(), Ord::max);
        let end = self
            .saturating_end_incl()
            .zip_with(other.saturating_end_incl(), Ord::min);
        let size = end.zip_with(start, |end, start| {
            end.saturating_sub(&start).saturating_add(&T::one())
        });
        Rect { start: start.cloned(), size }
    }

    /// Computes overlapped area between two rectangles, returning `None` on
    /// overflow.
    pub fn checked_overlapped<'params>(
        &'params self,
        other: &'params Self,
    ) -> Option<Self>
    where
        T: CheckedAdd + CheckedSub + One + Zero + Ord + Clone,
    {
        let start =
            self.start.as_ref().zip_with(other.start.as_ref(), Ord::max);
        let end = self
            .checked_end_incl()?
            .zip_with(other.checked_end_incl()?, Ord::min);
        let size = end
            .zip_with(start, |end, start| end.checked_sub(start))
            .transpose()?
            .map(|elem| elem.checked_add(&T::one()))
            .transpose()?;
        Some(Rect { start: start.cloned(), size })
    }
}

impl<T, S> Rect<T, S> {
    /// Iterator over all coordinates of this rectangle in the direction of
    /// columns.
    pub fn columns<'this>(&'this self) -> RectColumns<T>
    where
        &'this S: Sub<S, Output = T>,
        S: One + Zero,
        &'this T: Add<T, Output = T> + Sub<T, Output = T>,
        &'this T: Add<&'this S, Output = T>,
        T: One + AddAssign + Ord + Clone,
    {
        let inner = self.end_non_empty_ref().map(|end| RectColumnsInner {
            start: self.start.clone(),
            end: end.clone(),
            front: self.start.clone(),
            back: end,
        });
        RectColumns { inner }
    }

    /// Iterator over all coordinates of this rectangle in the direction of .
    pub fn rows<'this>(&'this self) -> RectRows<T>
    where
        &'this S: Sub<S, Output = T>,
        S: One + Zero,
        &'this T: Add<T, Output = T> + Sub<T, Output = T>,
        &'this T: Add<&'this S, Output = T>,
        T: One + AddAssign + Ord + Clone,
    {
        let inner = self.end_non_empty_ref().map(|end| RectRowsInner {
            start: self.start.clone(),
            end: end.clone(),
            front: self.start.clone(),
            back: end,
        });
        RectRows { inner }
    }

    /// Iterator over the inner borders of this rectangle.
    pub fn borders<'this>(&'this self) -> RectBorders<T>
    where
        &'this S: Sub<S, Output = T>,
        S: One + Zero,
        &'this T: Add<T, Output = T> + Sub<T, Output = T>,
        &'this T: Add<&'this S, Output = T>,
        T: AddAssign + One + Ord + Clone,
    {
        let inner = self.end_non_empty_ref().map(|end| RectBordersInner {
            start: self.start.clone(),
            fixed_axis: Axis::X,
            end,
            curr: self.start.clone(),
        });
        RectBorders { inner }
    }
}

/// Iterator over columns of the rectangle. See [`Rect::columns`].
#[derive(Debug)]
pub struct RectColumns<T> {
    inner: Option<RectColumnsInner<T>>,
}

impl<T> Iterator for RectColumns<T>
where
    T: One + AddAssign + Ord + Clone,
{
    type Item = Vec2<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let inner = self.inner.as_mut()?;
        let front = inner.front.clone();
        if inner.front.y >= inner.end.y {
            if inner.front.x < inner.back.x {
                inner.front.x += T::one();
                inner.front.y = inner.start.y.clone();
            } else {
                self.inner = None;
            }
        } else if inner.sides_crossed() {
            self.inner = None;
        } else {
            inner.front.y += T::one();
        }
        Some(front)
    }
}

impl<T> DoubleEndedIterator for RectColumns<T>
where
    T: One + AddAssign + SubAssign + Ord + Clone,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let inner = self.inner.as_mut()?;
        let back = inner.back.clone();
        if inner.back.y <= inner.start.y {
            if inner.front.x < inner.back.x {
                inner.back.x -= T::one();
                inner.back.y = inner.end.y.clone();
            } else {
                self.inner = None;
            }
        } else if inner.sides_crossed() {
            self.inner = None;
        } else {
            inner.back.y -= T::one();
        }
        Some(back)
    }
}

#[derive(Debug)]
struct RectColumnsInner<T> {
    start: Vec2<T>,
    end: Vec2<T>,
    front: Vec2<T>,
    back: Vec2<T>,
}

impl<T> RectColumnsInner<T>
where
    T: Ord,
{
    fn sides_crossed(&self) -> bool {
        self.front.x >= self.back.x && self.front.y >= self.back.y
    }
}

/// Iterator over rows of the rectangle. See [`Rect::rows`].
#[derive(Debug)]
pub struct RectRows<T> {
    inner: Option<RectRowsInner<T>>,
}

impl<T> Iterator for RectRows<T>
where
    T: One + AddAssign + Ord + Clone,
{
    type Item = Vec2<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let inner = self.inner.as_mut()?;
        let front = inner.front.clone();
        if inner.front.x >= inner.end.x {
            if inner.front.y < inner.back.y {
                inner.front.y += T::one();
                inner.front.x = inner.start.x.clone();
            } else {
                self.inner = None;
            }
        } else if inner.sides_crossed() {
            self.inner = None;
        } else {
            inner.front.x += T::one();
        }
        Some(front)
    }
}

impl<T> DoubleEndedIterator for RectRows<T>
where
    T: One + AddAssign + SubAssign + Ord + Clone,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let inner = self.inner.as_mut()?;
        let back = inner.back.clone();
        if inner.back.x <= inner.start.x {
            if inner.front.y < inner.back.y {
                inner.back.y -= T::one();
                inner.back.x = inner.end.x.clone();
            } else {
                self.inner = None;
            }
        } else if inner.sides_crossed() {
            self.inner = None;
        } else {
            inner.back.x -= T::one();
        }
        Some(back)
    }
}

#[derive(Debug)]
struct RectRowsInner<T> {
    start: Vec2<T>,
    end: Vec2<T>,
    front: Vec2<T>,
    back: Vec2<T>,
}

impl<T> RectRowsInner<T>
where
    T: Ord,
{
    fn sides_crossed(&self) -> bool {
        self.front.x >= self.back.x && self.front.y >= self.back.y
    }
}

/// Iterator over inner borders of the rectangle. See [`Rect::borders`].
#[derive(Debug)]
pub struct RectBorders<T> {
    inner: Option<RectBordersInner<T>>,
}

impl<T> Iterator for RectBorders<T>
where
    T: AddAssign + One + Ord + Clone,
{
    type Item = Vec2<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let inner = self.inner.as_mut()?;

        match inner.fixed_axis {
            Axis::X => {
                let curr = inner.curr.clone();
                if inner.curr.y >= inner.end.y {
                    if inner.curr.x >= inner.end.x {
                        if inner.start.x < inner.end.x {
                            inner.to_first_row();
                        } else {
                            self.inner = None;
                        }
                    } else {
                        inner.to_second_col();
                    }
                } else {
                    inner.iter_col();
                }
                Some(curr)
            },

            Axis::Y => {
                let curr = inner.curr.clone();
                if inner.curr.x >= inner.end.x {
                    if inner.curr.y < inner.end.y {
                        inner.to_second_row();
                    } else {
                        self.inner = None;
                    }
                } else {
                    inner.iter_row();
                }
                Some(curr)
            },
        }
    }
}

#[derive(Debug)]
struct RectBordersInner<T> {
    start: Vec2<T>,
    end: Vec2<T>,
    fixed_axis: Axis,
    curr: Vec2<T>,
}

impl<T> RectBordersInner<T>
where
    T: AddAssign + One + Clone,
{
    fn to_second_col(&mut self) {
        self.curr.x = self.end.x.clone();
        self.curr.y = self.start.y.clone();
    }

    fn to_first_row(&mut self) {
        self.curr.y = self.start.y.clone();
        self.curr.x = self.start.x.clone();
        self.curr.x += T::one();
        self.fixed_axis = Axis::Y;
    }

    fn to_second_row(&mut self) {
        self.curr.y = self.end.y.clone();
        self.curr.x = self.start.x.clone();
        self.curr.x += T::one();
    }

    fn iter_col(&mut self) {
        self.curr.y += T::one();
    }

    fn iter_row(&mut self) {
        self.curr.x += T::one();
    }
}
