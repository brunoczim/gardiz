use crate::{axis::Axis, coord::CoordPair};
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Rect<T, S = T> {
    pub start: CoordPair<T>,
    pub size: CoordPair<S>,
}

impl<T, S> Rect<T, S> {
    pub fn from_range<U>(start: CoordPair<T>, end: CoordPair<U>) -> Self
    where
        T: Clone,
        U: Sub<T, Output = S>,
    {
        let size = end - start.clone();
        Self { start, size }
    }
}

impl<T> Rect<T> {
    pub fn try_from_range<U>(
        start: CoordPair<T>,
        end: CoordPair<T>,
    ) -> Option<Self>
    where
        T: Clone + CheckedSub,
    {
        let size = end.checked_sub(&start)?;
        Some(Self { start, size })
    }
}

impl<T, S> Rect<T, S> {
    pub fn is_empty(&self) -> bool
    where
        S: Zero,
    {
        self.size.is_zero()
    }

    pub fn end(self) -> CoordPair<T::Output>
    where
        T: Add<S>,
    {
        self.start + self.size
    }
}

impl<T> Rect<T> {
    pub fn wrapping_end(&self) -> CoordPair<T>
    where
        T: WrappingAdd,
    {
        self.start.wrapping_add(&self.size)
    }

    pub fn saturating_end(&self) -> CoordPair<T>
    where
        T: SaturatingAdd,
    {
        self.start.saturating_add(&self.size)
    }

    pub fn checked_end(&self) -> Option<CoordPair<T>>
    where
        T: CheckedAdd,
    {
        self.start.checked_add(&self.size)
    }
}

impl<T, S> Rect<T, S> {
    pub fn end_ref<'this, U>(&'this self) -> CoordPair<U>
    where
        &'this T: Add<&'this S, Output = U>,
    {
        &self.start + &self.size
    }

    pub fn end_inclusive<U, V>(self) -> CoordPair<V>
    where
        S: Sub<Output = U> + One + Zero,
        T: Add<U, Output = V> + Sub<Output = V> + One,
    {
        if self.is_empty() {
            self.start - CoordPair::<T>::one()
        } else {
            self.start + (self.size - CoordPair::<S>::one())
        }
    }
}

impl<T> Rect<T> {
    pub fn wrapping_end_incl(&self) -> CoordPair<T>
    where
        T: WrappingAdd + WrappingSub + One,
    {
        self.start.wrapping_add(&self.size).wrapping_sub(&CoordPair::<T>::one())
    }

    pub fn saturating_end_incl(&self) -> CoordPair<T>
    where
        T: SaturatingAdd + SaturatingSub + One + Zero,
    {
        if self.is_empty() {
            self.start.saturating_sub(&CoordPair::<T>::one())
        } else {
            let last_index = self.size.saturating_sub(&CoordPair::<T>::one());
            self.start.saturating_add(&last_index)
        }
    }

    pub fn checked_end_incl(&self) -> Option<CoordPair<T>>
    where
        T: CheckedAdd + CheckedSub + One + Zero,
    {
        if self.is_empty() {
            self.start.checked_sub(&CoordPair::<T>::one())
        } else {
            let last_index = self.size.checked_sub(&CoordPair::<T>::one())?;
            self.start.checked_add(&&last_index)
        }
    }
}

impl<T, S> Rect<T, S> {
    pub fn end_incl_ref<'this, U, V>(&'this self) -> CoordPair<V>
    where
        &'this S: Sub<S, Output = U>,
        S: One + Zero,
        &'this T: Add<U, Output = V> + Sub<T, Output = V>,
        T: One,
    {
        if self.size.is_zero() {
            &self.start - CoordPair::<T>::one()
        } else {
            &self.start + (&self.size - CoordPair::<S>::one())
        }
    }

    pub fn end_non_empty<U, V>(self) -> Option<CoordPair<V>>
    where
        S: Sub<Output = U> + One + Zero,
        T: Add<U, Output = V> + Sub<Output = V> + One,
    {
        if self.is_empty() {
            None
        } else {
            Some(self.start + (self.size - CoordPair::<S>::one()))
        }
    }

    pub fn end_non_empty_ref<'this, U, V>(&'this self) -> Option<CoordPair<V>>
    where
        &'this S: Sub<S, Output = U>,
        S: One + Zero,
        &'this T: Add<U, Output = V> + Sub<T, Output = V>,
        T: One,
    {
        if self.size.is_zero() {
            None
        } else {
            Some(&self.start + (&self.size - CoordPair::<S>::one()))
        }
    }

    pub fn has_point<'this, U>(&'this self, point: CoordPair<T>) -> bool
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
    pub fn wrapping_overlapped<'params>(
        &'params self,
        other: &'params Self,
    ) -> Self
    where
        T: WrappingAdd + WrappingSub + Ord + Clone,
    {
        let start =
            self.start.as_ref().zip_with(other.start.as_ref(), Ord::max);
        let end = self.wrapping_end().zip_with(other.wrapping_end(), Ord::min);
        let size = end.zip_with(start, |end, start| end.wrapping_sub(start));
        Rect { start: start.cloned(), size }
    }

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
        let size = end.zip_with(start, |end, start| end.saturating_sub(start));
        Rect { start: start.cloned(), size }
    }

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
            .transpose()?;
        Some(Rect { start: start.cloned(), size })
    }
}

impl<T, S> Rect<T, S> {
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

#[derive(Debug)]
pub struct RectColumns<T> {
    inner: Option<RectColumnsInner<T>>,
}

impl<T> Iterator for RectColumns<T>
where
    T: One + AddAssign + Ord + Clone,
{
    type Item = CoordPair<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let inner = self.inner.as_mut()?;
        if inner.front.y == inner.end.y {
            inner.front.x += T::one();
            if inner.front.x >= inner.back.x {
                return None;
            }
            inner.front.y = inner.start.y.clone();
        } else if inner.sides_crossed() {
            self.inner = None;
            return None;
        }
        let front = inner.front.clone();
        inner.front.y += T::one();
        Some(front)
    }
}

impl<T> DoubleEndedIterator for RectColumns<T>
where
    T: One + AddAssign + SubAssign + Ord + Clone,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let inner = self.inner.as_mut()?;
        if inner.back.y == inner.start.y {
            if inner.back.x <= inner.front.x {
                None?
            }
            inner.back.x -= T::one();
            inner.back.y = inner.end.y.clone();
        } else if inner.sides_crossed() {
            self.inner = None;
            return None;
        }
        inner.back.y -= T::one();
        Some(inner.back.clone())
    }
}

#[derive(Debug)]
struct RectColumnsInner<T> {
    start: CoordPair<T>,
    end: CoordPair<T>,
    front: CoordPair<T>,
    back: CoordPair<T>,
}

impl<T> RectColumnsInner<T>
where
    T: Ord,
{
    fn sides_crossed(&self) -> bool {
        self.front.x >= self.back.x && self.front.y >= self.back.y
    }
}

#[derive(Debug)]
pub struct RectRows<T> {
    inner: Option<RectRowsInner<T>>,
}

impl<T> Iterator for RectRows<T>
where
    T: One + AddAssign + Ord + Clone,
{
    type Item = CoordPair<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let inner = self.inner.as_mut()?;
        if inner.front.x == inner.end.x {
            inner.front.y += T::one();
            if inner.front.y >= inner.back.y {
                return None;
            }
            inner.front.x = inner.start.x.clone();
        } else if inner.sides_crossed() {
            self.inner = None;
            return None;
        }
        let curr = inner.front.clone();
        inner.front.x += T::one();
        Some(curr)
    }
}

impl<T> DoubleEndedIterator for RectRows<T>
where
    T: One + AddAssign + SubAssign + Ord + Clone,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let inner = self.inner.as_mut()?;
        if inner.back.x == inner.start.x {
            if inner.back.y <= inner.front.y {
                None?
            }
            inner.back.y -= T::one();
            inner.back.x = inner.end.x.clone();
        } else if inner.sides_crossed() {
            self.inner = None;
            return None;
        }
        inner.back.x -= T::one();
        Some(inner.back.clone())
    }
}

#[derive(Debug)]
struct RectRowsInner<T> {
    start: CoordPair<T>,
    end: CoordPair<T>,
    front: CoordPair<T>,
    back: CoordPair<T>,
}

impl<T> RectRowsInner<T>
where
    T: Ord,
{
    fn sides_crossed(&self) -> bool {
        self.front.x >= self.back.x && self.front.y >= self.back.y
    }
}

#[derive(Debug)]
pub struct RectBorders<T> {
    inner: Option<RectBordersInner<T>>,
}

impl<T> Iterator for RectBorders<T>
where
    T: AddAssign + One + Ord + Clone,
{
    type Item = CoordPair<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let inner = self.inner.as_mut()?;

        match inner.fixed_axis {
            Axis::X => {
                let ret = inner.curr.clone();
                if inner.curr.y >= inner.end.y {
                    if inner.curr.x >= inner.end.x {
                        if inner.start.x == inner.end.x {
                            self.inner = None;
                        } else {
                            inner.to_first_row();
                        }
                    } else {
                        inner.to_second_col();
                    }
                } else {
                    inner.iter_col();
                }
                Some(ret)
            },

            Axis::Y => {
                let ret = inner.curr.clone();
                if inner.curr.x >= inner.end.x {
                    if inner.curr.y >= inner.end.y {
                        self.inner = None;
                        return None;
                    }
                    inner.to_second_row();
                } else {
                    inner.iter_row();
                }
                Some(ret)
            },
        }
    }
}

#[derive(Debug)]
struct RectBordersInner<T> {
    start: CoordPair<T>,
    end: CoordPair<T>,
    fixed_axis: Axis,
    curr: CoordPair<T>,
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
        self.curr.x = self.end.y.clone();
        self.curr.x += T::one();
    }

    fn iter_col(&mut self) {
        self.curr.y += T::one();
    }

    fn iter_row(&mut self) {
        self.curr.x += T::one();
    }
}
