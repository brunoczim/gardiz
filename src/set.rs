use crate::{coord::CoordPair, direc::Direction};
use std::{
    collections::{btree_map, btree_set, BTreeMap, BTreeSet},
    hash::Hash,
    ops::Bound,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Set<T>
where
    T: Ord + Hash,
{
    neighbours: CoordPair<BTreeMap<T, BTreeSet<T>>>,
}

impl<T> Default for Set<T>
where
    T: Ord + Hash,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Set<T>
where
    T: Ord + Hash,
{
    pub fn new() -> Self {
        Set { neighbours: CoordPair::from_axes(|_| BTreeMap::new()) }
    }

    pub fn len(&self) -> usize {
        self.neighbours.x.len()
    }

    pub fn contains(&self, point: &CoordPair<T>) -> bool {
        self.neighbours
            .x
            .get(&point.x)
            .map_or(false, |ys| ys.contains(&point.y))
    }

    pub fn neighbour<'found>(
        &'found self,
        point: &'found CoordPair<T>,
        direction: Direction,
    ) -> Option<CoordPair<&'found T>> {
        match direction {
            Direction::Up => self
                .neighbours
                .x
                .get(&point.x)?
                .range(.. &point.y)
                .next_back()
                .map(|coord_y| CoordPair { x: &point.x, y: coord_y }),

            Direction::Down => self
                .neighbours
                .x
                .get(&point.x)?
                .range((Bound::Excluded(&point.y), Bound::Unbounded))
                .next()
                .map(|coord_y| CoordPair { x: &point.x, y: coord_y }),

            Direction::Left => self
                .neighbours
                .y
                .get(&point.y)?
                .range(.. &point.x)
                .next_back()
                .map(|coord_x| CoordPair { y: &point.y, x: coord_x }),

            Direction::Right => self
                .neighbours
                .y
                .get(&point.y)?
                .range((Bound::Excluded(&point.x), Bound::Unbounded))
                .next()
                .map(|coord_x| CoordPair { y: &point.y, x: coord_x }),
        }
    }

    pub fn last_neighbour<'found>(
        &'found self,
        point: &'found CoordPair<T>,
        direction: Direction,
    ) -> Option<CoordPair<&'found T>> {
        match direction {
            Direction::Up => self
                .neighbours
                .x
                .get(&point.x)?
                .range(.. &point.y)
                .next()
                .map(|coord_y| CoordPair { x: &point.x, y: coord_y }),

            Direction::Down => self
                .neighbours
                .x
                .get(&point.x)?
                .range((Bound::Excluded(&point.y), Bound::Unbounded))
                .next_back()
                .map(|coord_y| CoordPair { x: &point.x, y: coord_y }),

            Direction::Left => self
                .neighbours
                .y
                .get(&point.y)?
                .range(.. &point.x)
                .next()
                .map(|coord_x| CoordPair { y: &point.y, x: coord_x }),

            Direction::Right => self
                .neighbours
                .y
                .get(&point.y)?
                .range((Bound::Excluded(&point.x), Bound::Unbounded))
                .next_back()
                .map(|coord_x| CoordPair { y: &point.y, x: coord_x }),
        }
    }

    pub fn insert(&mut self, point: CoordPair<T>)
    where
        T: Clone,
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

        inner_tables.zip_with(!point, |table, value| table.insert(value));
    }

    pub fn remove(&mut self, point: &CoordPair<T>) -> bool {
        let table_pairs = point.as_ref().zip(!point.as_ref());
        let removed = self.neighbours.as_mut().zip_with(
            table_pairs,
            |table, (key, value)| {
                let inner_table = match table.get_mut(key) {
                    Some(table) => table,
                    None => return false,
                };

                let removed = inner_table.remove(value);
                if removed {
                    table.remove(&key);
                }
                removed
            },
        );

        removed.x && removed.y
    }

    pub fn rows(&self) -> Rows<T> {
        Rows { outer: self.neighbours.y.iter(), front: None, back: None }
    }

    pub fn columns(&self) -> Columns<T> {
        Columns { outer: self.neighbours.x.iter(), front: None, back: None }
    }
}

#[derive(Debug)]
pub struct Rows<'set, T>
where
    T: Ord + Hash,
{
    outer: btree_map::Iter<'set, T, BTreeSet<T>>,
    front: Option<(&'set T, btree_set::Iter<'set, T>)>,
    back: Option<(&'set T, btree_set::Iter<'set, T>)>,
}

impl<'set, T> Iterator for Rows<'set, T>
where
    T: Ord + Hash,
{
    type Item = CoordPair<&'set T>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some((y, inner)) = &mut self.front {
                match inner.next() {
                    Some(x) => break Some(CoordPair { x, y }),
                    None => self.front = None,
                }
            }
            match self.outer.next() {
                Some((y, inner)) => self.front = Some((y, inner.iter())),
                None => {
                    let (y, inner) = self.back.as_mut()?;
                    match inner.next() {
                        Some(x) => break Some(CoordPair { x, y }),
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

impl<'set, T> DoubleEndedIterator for Rows<'set, T>
where
    T: Ord + Hash,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            if let Some((y, inner)) = &mut self.back {
                match inner.next() {
                    Some(x) => break Some(CoordPair { x, y }),
                    None => self.back = None,
                }
            }
            match self.outer.next() {
                Some((y, inner)) => self.back = Some((y, inner.iter())),
                None => {
                    let (y, inner) = self.front.as_mut()?;
                    match inner.next() {
                        Some(x) => break Some(CoordPair { x, y }),
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
pub struct Columns<'set, T>
where
    T: Ord + Hash,
{
    outer: btree_map::Iter<'set, T, BTreeSet<T>>,
    front: Option<(&'set T, btree_set::Iter<'set, T>)>,
    back: Option<(&'set T, btree_set::Iter<'set, T>)>,
}

impl<'set, T> Iterator for Columns<'set, T>
where
    T: Ord + Hash,
{
    type Item = CoordPair<&'set T>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some((x, inner)) = &mut self.front {
                match inner.next() {
                    Some(y) => break Some(CoordPair { x, y }),
                    None => self.front = None,
                }
            }
            match self.outer.next() {
                Some((x, inner)) => self.front = Some((x, inner.iter())),
                None => {
                    let (x, inner) = self.back.as_mut()?;
                    match inner.next() {
                        Some(y) => break Some(CoordPair { x, y }),
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

impl<'set, T> DoubleEndedIterator for Columns<'set, T>
where
    T: Ord + Hash,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            if let Some((x, inner)) = &mut self.back {
                match inner.next() {
                    Some(y) => break Some(CoordPair { x, y }),
                    None => self.back = None,
                }
            }
            match self.outer.next() {
                Some((x, inner)) => self.back = Some((x, inner.iter())),
                None => {
                    let (x, inner) = self.front.as_mut()?;
                    match inner.next() {
                        Some(y) => break Some(CoordPair { x, y }),
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
