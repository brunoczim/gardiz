//! A map from coordinates/vectors to arbitrary data, optimized for the plane
//! (and related items).

#[cfg(test)]
mod test;

use crate::{coord::Vec2, direc::Direction};
use std::{
    borrow::Borrow,
    collections::{btree_map, BTreeMap},
    iter::FromIterator,
    mem,
};

/// Map of coordinates in a plane to arbitrary data. Optimized given the fact
/// the coordinates/vectors are in the plane. Keys of the map are `Vec2<K>`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Map<K, V>
where
    K: Ord,
{
    neighbours: Vec2<BTreeMap<K, BTreeMap<K, V>>>,
}

impl<K, V> Default for Map<K, V>
where
    K: Ord,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> Map<K, V>
where
    K: Ord,
{
    /// Creates a new empty coordinate map.
    pub fn new() -> Self {
        Map { neighbours: Vec2::from_axes(|_| BTreeMap::new()) }
    }

    /// Returns whether the map is empty.
    pub fn is_empty(&self) -> bool {
        self.neighbours.x.is_empty()
    }

    /// Attempts to get the data associated with the given point.
    pub fn get<Q>(&self, point: Vec2<&Q>) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        self.neighbours.x.get(point.x).and_then(|ys| ys.get(&point.y))
    }

    /// Tests if the map contains the given point.
    pub fn contains<Q>(&self, point: Vec2<&Q>) -> bool
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        self.neighbours
            .x
            .get(point.x)
            .map_or(false, |ys| ys.contains_key(&point.y))
    }

    /// Returns an iterator to the neighbours of a given point in a straight
    /// line in the given direction. The starting point is NOT included. For
    /// every point yielded by the iterator, the associated data is also
    /// returned.
    pub fn neighbours<Q>(
        &self,
        point: Vec2<&Q>,
        direction: Direction,
    ) -> Neighbours<K, V>
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        let mut iterator = self.neighbours_incl(point, direction);
        iterator.next();
        iterator
    }

    /// Returns an iterator to the neighbours of a given point in a straight
    /// line in the given direction. The starting point IS included. For
    /// every point yielded by the iterator, the associated data is also
    /// returned.
    pub fn neighbours_incl<Q>(
        &self,
        point: Vec2<&Q>,
        direction: Direction,
    ) -> Neighbours<K, V>
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        Neighbours { inner: NeighboursInner::new(self, point, direction) }
    }

    /// Returns the nearest neighbour in a straight line of a given point in the
    /// the given direction WITHOUT the associated data.
    pub fn first_neighbour<Q>(
        &self,
        point: Vec2<&Q>,
        direction: Direction,
    ) -> Option<Vec2<&K>>
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        self.first_neighbour_data(point, direction).map(|(key, _)| key)
    }

    /// Returns the nearest neighbour in a straight line of a given point in the
    /// given direction WITH the associated data.
    pub fn first_neighbour_data<Q>(
        &self,
        point: Vec2<&Q>,
        direction: Direction,
    ) -> Option<(Vec2<&K>, &V)>
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        self.neighbours(point, direction).next()
    }

    /// Returns the furthest neighbour in a straight line of a given point in
    /// the given direction WITHOUT the associated data.
    pub fn last_neighbour<Q>(
        &self,
        point: Vec2<&Q>,
        direction: Direction,
    ) -> Option<Vec2<&K>>
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        self.last_neighbour_data(point, direction).map(|(key, _)| key)
    }

    /// Returns the furthest neighbour in a straight line of a given point in
    /// the given direction WITH the associated data.
    pub fn last_neighbour_data<Q>(
        &self,
        point: Vec2<&Q>,
        direction: Direction,
    ) -> Option<(Vec2<&K>, &V)>
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        self.neighbours(point, direction).next_back()
    }

    /// Inserts the given point with its associated data. A possible previous
    /// value is returned.
    pub fn insert(&mut self, point: Vec2<K>, value: V) -> Option<V>
    where
        K: Clone,
        V: Clone,
    {
        let inner_tables = match self
            .neighbours
            .as_mut()
            .zip_with(point.as_ref(), |table, key| table.get_mut(key))
            .transpose()
        {
            Some(entries) => entries,
            None => self
                .neighbours
                .as_mut()
                .zip_with(point.clone(), |table, key| {
                    table.entry(key).or_default()
                }),
        };

        let values = Vec2 { x: value.clone(), y: value };
        let old = (!point)
            .zip(values)
            .zip_with(inner_tables, |(key, value), table| {
                table.insert(key, value)
            });
        old.x
    }

    /// Creates an entry at the given point with its associated data. Fails if
    /// there already is an entry for that point. Returns whether it succeeded.
    pub fn create(&mut self, point: Vec2<K>, value: V) -> bool
    where
        K: Clone,
        V: Clone,
    {
        let values = Vec2 { x: value.clone(), y: value };
        let entries = (!point.clone()).zip(values);
        let result = self
            .neighbours
            .as_mut()
            .zip_with(point, |table, key| table.entry(key))
            .zip_with(entries, |table_entry, (key, value)| match table_entry {
                btree_map::Entry::Vacant(table_entry) => {
                    let mut inner_table = BTreeMap::new();
                    inner_table.insert(key, value);
                    table_entry.insert(inner_table);
                    true
                },

                btree_map::Entry::Occupied(table_entry) => {
                    match table_entry.into_mut().entry(key) {
                        btree_map::Entry::Vacant(inner_entry) => {
                            inner_entry.insert(value);
                            true
                        },
                        btree_map::Entry::Occupied(_) => false,
                    }
                },
            });
        result.x
    }

    /// Updates an existing point's entry with the given value. The entry must
    /// exist. Returns `Ok(old_value)` if successful, but `Err(attempted_value)`
    /// if failing.
    pub fn update<Q>(&mut self, point: Vec2<&Q>, value: V) -> Result<V, V>
    where
        K: Borrow<Q>,
        Q: Ord,
        V: Clone,
    {
        let inner_tables = match self
            .neighbours
            .as_mut()
            .zip_with(point, |table, key| table.get_mut(key))
            .transpose()
        {
            Some(inner_tables) => inner_tables,
            None => return Err(value),
        };

        let values = Vec2 { x: value.clone(), y: value };
        let old = (!point).zip(values).zip_with(
            inner_tables,
            |(key, value), table| match table.get_mut(key) {
                Some(entry) => Ok(mem::replace(entry, value)),
                None => Err(value),
            },
        );
        old.x
    }

    /// Removes the given point entry from the from the map. Returns the data
    /// associated with the point, if successful.
    pub fn remove<Q>(&mut self, point: Vec2<&Q>) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        let table_pairs = point.zip(!point);
        let removed = self.neighbours.as_mut().zip_with(
            table_pairs,
            |table, (key, inner_key)| {
                let inner_table = table.get_mut(key)?;

                let removed = inner_table.remove(inner_key);
                if table.is_empty() {
                    table.remove(&key);
                }
                removed
            },
        );
        removed.x
    }

    /// Returns an iterator over all entries, by running through the "rows" of
    /// the map, (with the first entries having the lowest keys), i.e. all `X`
    /// are yielded before going to the next `Y`.
    pub fn rows(&self) -> Rows<K, V> {
        Rows { outer: self.neighbours.y.iter(), front: None, back: None }
    }

    /// Returns an iterator over all entries, by running through the "columns"
    /// of the map, (with the first entries having the lowest keys), i.e. all
    /// `Y` are yielded before going to the next `X`.
    pub fn columns(&self) -> Columns<K, V> {
        Columns {
            transposed: Rows {
                outer: self.neighbours.x.iter(),
                front: None,
                back: None,
            },
        }
    }
}

impl<K, V> Extend<(Vec2<K>, V)> for Map<K, V>
where
    K: Ord + Clone,
    V: Clone,
{
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = (Vec2<K>, V)>,
    {
        for (point, value) in iter {
            self.insert(point, value);
        }
    }
}

impl<K, V> FromIterator<(Vec2<K>, V)> for Map<K, V>
where
    K: Ord + Clone,
    V: Clone,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (Vec2<K>, V)>,
    {
        let mut map = Map::new();
        map.extend(iter);
        map
    }
}

/// Iterator over the neighbours of a point in a map. See [`Map::neighbours`]
/// and [`Map::neighbours_incl`].
#[derive(Debug)]
pub struct Neighbours<'map, K, V>
where
    K: Ord,
{
    inner: Option<NeighboursInner<'map, K, V>>,
}

impl<'map, K, V> Iterator for Neighbours<'map, K, V>
where
    K: Ord,
{
    type Item = (Vec2<&'map K>, &'map V);

    fn next(&mut self) -> Option<Self::Item> {
        let inner = self.inner.as_mut()?;
        match inner.direction {
            Direction::Up => {
                let (inner_key, value) = inner.range.next_back()?;
                Some((Vec2 { x: inner.key, y: inner_key }, value))
            },
            Direction::Down => {
                let (inner_key, value) = inner.range.next()?;
                Some((Vec2 { x: inner.key, y: inner_key }, value))
            },
            Direction::Left => {
                let (inner_key, value) = inner.range.next_back()?;
                Some((Vec2 { y: inner.key, x: inner_key }, value))
            },
            Direction::Right => {
                let (inner_key, value) = inner.range.next()?;
                Some((Vec2 { y: inner.key, x: inner_key }, value))
            },
        }
    }
}

impl<'map, K, V> DoubleEndedIterator for Neighbours<'map, K, V>
where
    K: Ord,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let inner = self.inner.as_mut()?;
        match inner.direction {
            Direction::Up => {
                let (inner_key, value) = inner.range.next()?;
                Some((Vec2 { x: inner.key, y: inner_key }, value))
            },
            Direction::Down => {
                let (inner_key, value) = inner.range.next_back()?;
                Some((Vec2 { x: inner.key, y: inner_key }, value))
            },
            Direction::Left => {
                let (inner_key, value) = inner.range.next()?;
                Some((Vec2 { y: inner.key, x: inner_key }, value))
            },
            Direction::Right => {
                let (inner_key, value) = inner.range.next_back()?;
                Some((Vec2 { y: inner.key, x: inner_key }, value))
            },
        }
    }
}

#[derive(Debug)]
struct NeighboursInner<'map, K, V>
where
    K: Ord,
{
    direction: Direction,
    key: &'map K,
    range: btree_map::Range<'map, K, V>,
}

impl<'map, K, V> NeighboursInner<'map, K, V>
where
    K: Ord,
{
    fn new<'param, Q>(
        map: &'map Map<K, V>,
        point: Vec2<&'param Q>,
        direction: Direction,
    ) -> Option<Self>
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        match direction {
            Direction::Up => {
                let (key, table) = map.neighbours.x.get_key_value(point.x)?;
                if table.contains_key(point.y) {
                    let range = table.range(..= point.y);
                    Some(Self { key, direction, range })
                } else {
                    None
                }
            },

            Direction::Down => {
                let (key, table) = map.neighbours.x.get_key_value(&point.x)?;
                if table.contains_key(point.y) {
                    let range = table.range(point.y ..);
                    Some(Self { key, direction, range })
                } else {
                    None
                }
            },

            Direction::Left => {
                let (key, table) = map.neighbours.y.get_key_value(&point.y)?;
                if table.contains_key(point.x) {
                    let range = table.range(..= point.x);
                    Some(Self { key, direction, range })
                } else {
                    None
                }
            },

            Direction::Right => {
                let (key, table) = map.neighbours.y.get_key_value(&point.y)?;
                if table.contains_key(point.x) {
                    let range = table.range(point.x ..);
                    Some(Self { key, direction, range })
                } else {
                    None
                }
            },
        }
    }
}

/// Iterator over entries of a map in the direction of rows. See [`Map::rows`].
#[derive(Debug)]
pub struct Rows<'map, K, V>
where
    K: Ord,
{
    outer: btree_map::Iter<'map, K, BTreeMap<K, V>>,
    front: Option<(&'map K, btree_map::Iter<'map, K, V>)>,
    back: Option<(&'map K, btree_map::Iter<'map, K, V>)>,
}

impl<'map, K, V> Iterator for Rows<'map, K, V>
where
    K: Ord,
{
    type Item = (Vec2<&'map K>, &'map V);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some((y, inner)) = &mut self.front {
                match inner.next() {
                    Some((x, value)) => break Some((Vec2 { x, y }, value)),
                    None => self.front = None,
                }
            }
            match self.outer.next() {
                Some((y, inner)) => self.front = Some((y, inner.iter())),
                None => {
                    let (y, inner) = self.back.as_mut()?;
                    match inner.next() {
                        Some((x, value)) => break Some((Vec2 { x, y }, value)),
                        None => {
                            self.back = None;
                            break None;
                        },
                    }
                },
            }
        }
    }
}

impl<'map, K, V> DoubleEndedIterator for Rows<'map, K, V>
where
    K: Ord,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            if let Some((y, inner)) = &mut self.back {
                match inner.next() {
                    Some((x, value)) => break Some((Vec2 { x, y }, value)),
                    None => self.back = None,
                }
            }
            match self.outer.next() {
                Some((y, inner)) => self.back = Some((y, inner.iter())),
                None => {
                    let (y, inner) = self.front.as_mut()?;
                    match inner.next() {
                        Some((x, value)) => break Some((Vec2 { x, y }, value)),
                        None => {
                            self.front = None;
                            break None;
                        },
                    }
                },
            }
        }
    }
}

/// Iterator over entries of a map in the direction of columns. See
/// [`Map::columns`].
#[derive(Debug)]
pub struct Columns<'map, K, V>
where
    K: Ord,
{
    transposed: Rows<'map, K, V>,
}

impl<'map, K, V> Iterator for Columns<'map, K, V>
where
    K: Ord,
{
    type Item = (Vec2<&'map K>, &'map V);

    fn next(&mut self) -> Option<Self::Item> {
        self.transposed.next().map(|(coord, value)| (!coord, value))
    }
}

impl<'map, K, V> DoubleEndedIterator for Columns<'map, K, V>
where
    K: Ord,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.transposed.next_back().map(|(coord, value)| (!coord, value))
    }
}
