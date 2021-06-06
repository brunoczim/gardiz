use crate::{
    coord::{CoordPair, CoordRef},
    direc::Direction,
};
use std::{
    collections::{btree_map, BTreeMap},
    iter::FromIterator,
    mem,
    ops::Bound,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Map<K, V>
where
    K: Ord,
{
    neighbours: CoordPair<BTreeMap<K, BTreeMap<K, V>>>,
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
    pub fn new() -> Self {
        Map { neighbours: CoordPair::from_axes(|_| BTreeMap::new()) }
    }

    pub fn is_empty(&self) -> bool {
        self.neighbours.x.is_empty()
    }

    pub fn get<C>(&self, point: C) -> Option<&V>
    where
        C: CoordRef<K>,
    {
        self.neighbours
            .x
            .get(point.as_coord_ref().x)
            .and_then(|ys| ys.get(point.as_coord_ref().y))
    }

    pub fn contains<C>(&self, point: C) -> bool
    where
        C: CoordRef<K>,
    {
        self.neighbours
            .x
            .get(point.as_coord_ref().x)
            .map_or(false, |ys| ys.contains_key(point.as_coord_ref().y))
    }

    pub fn neighbours<C>(
        &self,
        point: C,
        direction: Direction,
    ) -> Neighbours<K, V>
    where
        C: CoordRef<K>,
    {
        Neighbours {
            inner: NeighboursInner::new(self, point.as_coord_ref(), direction),
        }
    }

    pub fn first_neighbour<C>(
        &self,
        point: C,
        direction: Direction,
    ) -> Option<CoordPair<&K>>
    where
        C: CoordRef<K>,
    {
        self.neighbours(point, direction).next().map(|(key, _)| key)
    }

    pub fn last_neighbour<C>(
        &self,
        point: C,
        direction: Direction,
    ) -> Option<CoordPair<&K>>
    where
        C: CoordRef<K>,
    {
        self.neighbours(point, direction).next_back().map(|(key, _)| key)
    }

    pub fn insert(&mut self, point: CoordPair<K>, value: V) -> Option<V>
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

        let values = CoordPair { x: value.clone(), y: value };
        let old = (!point)
            .zip(values)
            .zip_with(inner_tables, |(key, value), table| {
                table.insert(key, value)
            });
        old.x
    }

    pub fn create(&mut self, point: CoordPair<K>, value: V) -> Result<(), &V>
    where
        K: Clone,
        V: Clone,
    {
        let values = CoordPair { x: value.clone(), y: value };
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
                    Ok(())
                },

                btree_map::Entry::Occupied(table_entry) => {
                    match table_entry.get_mut().entry(key) {
                        btree_map::Entry::Vacant(inner_entry) => {
                            inner_entry.insert(value);
                            Ok(())
                        },
                        btree_map::Entry::Occupied(inner_entry) => {
                            Err(inner_entry.get())
                        },
                    }
                },
            });
        result.x
    }

    pub fn update<C>(&mut self, point: C, value: V) -> Result<V, V>
    where
        C: CoordRef<K>,
        V: Clone,
    {
        let point = point.as_coord_ref();
        let inner_tables = self
            .neighbours
            .as_mut()
            .zip_with(point, |table, key| table.get_mut(key))
            .transpose()
            .ok_or(value)?;

        let values = CoordPair { x: value.clone(), y: value };
        let old = (!point).zip(values).zip_with(
            inner_tables,
            |(key, value), table| match table.get_mut(key) {
                Some(entry) => Ok(mem::replace(entry, value)),
                None => Err(value),
            },
        );
        old.x
    }

    pub fn remove<C>(&mut self, point: C) -> Option<V>
    where
        C: CoordRef<K>,
    {
        let table_pairs = point.as_coord_ref().zip(!point.as_coord_ref());
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

    pub fn rows(&self) -> Rows<K, V> {
        Rows { outer: self.neighbours.y.iter(), front: None, back: None }
    }

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

impl<K, V> Extend<(CoordPair<K>, V)> for Map<K, V>
where
    K: Ord + Clone,
    V: Clone,
{
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = (CoordPair<K>, V)>,
    {
        for (point, value) in iter {
            self.insert(point, value);
        }
    }
}

impl<K, V> FromIterator<(CoordPair<K>, V)> for Map<K, V>
where
    K: Ord + Clone,
    V: Clone,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (CoordPair<K>, V)>,
    {
        let mut map = Map::new();
        map.extend(iter);
        map
    }
}

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
    type Item = (CoordPair<&'map K>, &'map V);

    fn next(&mut self) -> Option<Self::Item> {
        let inner = self.inner.as_mut()?;
        match inner.direction {
            Direction::Up => {
                let (inner_key, value) = inner.range.next_back()?;
                Some((CoordPair { x: inner.key, y: inner_key }, value))
            },
            Direction::Down => {
                let (inner_key, value) = inner.range.next()?;
                Some((CoordPair { x: inner.key, y: inner_key }, value))
            },
            Direction::Left => {
                let (inner_key, value) = inner.range.next_back()?;
                Some((CoordPair { y: inner.key, x: inner_key }, value))
            },
            Direction::Right => {
                let (inner_key, value) = inner.range.next()?;
                Some((CoordPair { y: inner.key, x: inner_key }, value))
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
                Some((CoordPair { x: inner.key, y: inner_key }, value))
            },
            Direction::Down => {
                let (inner_key, value) = inner.range.next_back()?;
                Some((CoordPair { x: inner.key, y: inner_key }, value))
            },
            Direction::Left => {
                let (inner_key, value) = inner.range.next()?;
                Some((CoordPair { y: inner.key, x: inner_key }, value))
            },
            Direction::Right => {
                let (inner_key, value) = inner.range.next_back()?;
                Some((CoordPair { y: inner.key, x: inner_key }, value))
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
    fn new<'param>(
        map: &'map Map<K, V>,
        point: CoordPair<&'param K>,
        direction: Direction,
    ) -> Option<Self> {
        match direction {
            Direction::Up => {
                let (key, table) = map.neighbours.x.get_key_value(point.x)?;
                let range = table.range(.. point.y);
                Some(Self { key, direction, range })
            },

            Direction::Down => {
                let (key, table) = map.neighbours.x.get_key_value(point.x)?;
                let range_spec = (Bound::Excluded(point.y), Bound::Unbounded);
                let range = table.range(range_spec);
                Some(Self { key, direction, range })
            },

            Direction::Left => {
                let (key, table) = map.neighbours.y.get_key_value(point.y)?;
                let range = table.range(.. point.x);
                Some(Self { key, direction, range })
            },

            Direction::Right => {
                let (key, table) = map.neighbours.y.get_key_value(point.y)?;
                let range_spec = (Bound::Excluded(point.x), Bound::Unbounded);
                let range = table.range(range_spec);
                Some(Self { key, direction, range })
            },
        }
    }
}

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
    type Item = (CoordPair<&'map K>, &'map V);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some((y, inner)) = &mut self.front {
                match inner.next() {
                    Some((x, value)) => {
                        break Some((CoordPair { x, y }, value))
                    },
                    None => self.front = None,
                }
            }
            match self.outer.next() {
                Some((y, inner)) => self.front = Some((y, inner.iter())),
                None => {
                    let (y, inner) = self.back.as_mut()?;
                    match inner.next() {
                        Some((x, value)) => {
                            break Some((CoordPair { x, y }, value))
                        },
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
                    Some((x, value)) => {
                        break Some((CoordPair { x, y }, value))
                    },
                    None => self.back = None,
                }
            }
            match self.outer.next() {
                Some((y, inner)) => self.back = Some((y, inner.iter())),
                None => {
                    let (y, inner) = self.front.as_mut()?;
                    match inner.next() {
                        Some((x, value)) => {
                            break Some((CoordPair { x, y }, value))
                        },
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
    type Item = (CoordPair<&'map K>, &'map V);

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
