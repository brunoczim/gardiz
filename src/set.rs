//! A set of coordinates/vectors in a plane optimized for the plane (and related
//! utilites).

#[cfg(test)]
mod test;

use crate::{coord::Vec2, direc::Direction, map, map::Map};
use std::{borrow::Borrow, iter::FromIterator};

/// The set of coordinates/vectors in a plane, optimized for being in the plane.
/// Members of the set are `Vec2<T>`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "impl-serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct Set<T>
where
    T: Ord,
{
    #[cfg_attr(
        feature = "impl-serde",
        serde(bound(deserialize = "T: serde::Deserialize<'de> + Clone"))
    )]
    inner: Map<T, ()>,
}

impl<T> Default for Set<T>
where
    T: Ord,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Set<T>
where
    T: Ord,
{
    /// Creates a new empty set.
    pub fn new() -> Self {
        Set { inner: Map::new() }
    }

    /// Tests if the set is emtpy.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Returns the length of the set, i.e. how many [`Vec2`] are stored in this
    /// set.
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Tests if the set contains a given point.
    pub fn contains<U>(&self, point: Vec2<&U>) -> bool
    where
        T: Borrow<U>,
        U: Ord,
    {
        self.inner.contains(point)
    }

    /// Returns an iterator to the neighbours of a given point in a straight
    /// line in the given direction. The starting point is NOT included.
    pub fn neighbours<U>(
        &self,
        point: Vec2<&U>,
        direction: Direction,
    ) -> Neighbours<T>
    where
        T: Borrow<U>,
        U: Ord,
    {
        Neighbours { inner: self.inner.neighbours(point, direction) }
    }

    /// Returns an iterator to the neighbours of a given point in a straight
    /// line in the given direction. The starting point IS included.
    pub fn neighbours_incl<U>(
        &self,
        point: Vec2<&U>,
        direction: Direction,
    ) -> Neighbours<T>
    where
        T: Borrow<U>,
        U: Ord,
    {
        Neighbours { inner: self.inner.neighbours_incl(point, direction) }
    }

    /// Returns the nearest neighbour in a straight line of a given point in the
    /// the given direction.
    pub fn first_neighbour<U>(
        &self,
        point: Vec2<&U>,
        direction: Direction,
    ) -> Option<Vec2<&T>>
    where
        U: Ord,
        T: Borrow<U>,
    {
        self.inner.first_neighbour(point, direction)
    }

    /// Returns the furthest neighbour in a straight line of a given point in
    /// the given direction.
    pub fn last_neighbour<U>(
        &self,
        point: Vec2<&U>,
        direction: Direction,
    ) -> Option<Vec2<&T>>
    where
        U: Ord,
        T: Borrow<U>,
    {
        self.inner.last_neighbour(point, direction)
    }

    /// Inserts the given point in the set. Returns whether the insertion
    /// actually happened (i.e. the point was not already in the set).
    pub fn insert(&mut self, point: Vec2<T>) -> bool
    where
        T: Clone,
    {
        self.inner.insert(point, ()).is_none()
    }

    /// Removes a point from the set. Returns whether the removal actuall
    /// happened (i.e. the point was in the set).
    pub fn remove<U>(&mut self, point: Vec2<&U>) -> bool
    where
        U: Ord,
        T: Borrow<U>,
    {
        self.inner.remove(point).is_some()
    }

    /// Returns an iterator over all the points in the set, in the direction of
    /// rows (first point is the lowest), i.e. all `X` are yielded before going
    /// to the next `Y`.
    pub fn rows(&self) -> Rows<T> {
        Rows { inner: self.inner.rows() }
    }

    /// Returns an iterator over all the points in the set, in the direction of
    /// columns (first point is the lowest), i.e. all `Y` are yielded before
    /// going to the next `X`.
    pub fn columns(&self) -> Columns<T> {
        Columns { inner: self.inner.columns() }
    }
}

impl<T> Extend<Vec2<T>> for Set<T>
where
    T: Ord + Clone,
{
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = Vec2<T>>,
    {
        self.inner.extend(iter.into_iter().map(|key| (key, ())));
    }
}

impl<T> FromIterator<Vec2<T>> for Set<T>
where
    T: Ord + Clone,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Vec2<T>>,
    {
        Self { inner: iter.into_iter().map(|key| (key, ())).collect() }
    }
}

/// Iterator over the neighbours of a given point in a given direction (in a
/// set). See [`Set::neighbours`] and [`Set::neighbours_incl`].
#[derive(Debug, Clone)]
pub struct Neighbours<'set, T>
where
    T: Ord,
{
    inner: map::Neighbours<'set, T, ()>,
}

impl<'set, T> Iterator for Neighbours<'set, T>
where
    T: Ord,
{
    type Item = Vec2<&'set T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(key, _)| key)
    }
}

impl<'set, T> DoubleEndedIterator for Neighbours<'set, T>
where
    T: Ord,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back().map(|(key, _)| key)
    }
}

/// Iterator over the points of a set in the direction of rows (in a
/// set). See [`Set::rows`].
#[derive(Debug, Clone)]
pub struct Rows<'set, T>
where
    T: Ord,
{
    inner: map::Rows<'set, T, ()>,
}

impl<'set, T> Iterator for Rows<'set, T>
where
    T: Ord,
{
    type Item = Vec2<&'set T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(key, _)| key)
    }
}

impl<'set, T> DoubleEndedIterator for Rows<'set, T>
where
    T: Ord,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back().map(|(key, _)| key)
    }
}

/// Iterator over the points of a set in the direction of columns (in a
/// set). See [`Set::columns`].
#[derive(Debug, Clone)]
pub struct Columns<'set, T>
where
    T: Ord,
{
    inner: map::Columns<'set, T, ()>,
}

impl<'set, T> Iterator for Columns<'set, T>
where
    T: Ord,
{
    type Item = Vec2<&'set T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(key, _)| key)
    }
}

impl<'set, T> DoubleEndedIterator for Columns<'set, T>
where
    T: Ord,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back().map(|(key, _)| key)
    }
}
