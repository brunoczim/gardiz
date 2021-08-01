#[cfg(test)]
mod test;

use crate::{
    coord::{CoordPair, CoordRef},
    direc::{DirecMap, Direction},
    map::Map,
};
use std::{borrow::Borrow, collections::BTreeSet};

pub type VertexEdges = DirecMap<bool>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Graph<T>
where
    T: Ord,
{
    edges: Map<T, VertexEdges>,
}

impl<T> Default for Graph<T>
where
    T: Ord,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Graph<T>
where
    T: Ord,
{
    pub fn new() -> Self {
        Self { edges: Map::new() }
    }

    pub fn from_vertices<I>(vertices: I) -> Self
    where
        I: IntoIterator<Item = CoordPair<T>>,
        T: Clone,
    {
        Self {
            edges: vertices
                .into_iter()
                .map(|vertex| (vertex, DirecMap::from_direcs(|_| false)))
                .collect(),
        }
    }

    pub fn edges(&self) -> &Map<T, DirecMap<bool>> {
        &self.edges
    }

    pub fn vertex_edges<U>(&self, vertex: CoordPair<&U>) -> Option<VertexEdges>
    where
        U: Ord,
        T: Borrow<U>,
    {
        self.edges.get(vertex).copied()
    }

    pub fn connected<U>(
        &self,
        vertex_a: CoordPair<&U>,
        vertex_b: CoordPair<&U>,
    ) -> bool
    where
        U: Ord,
        T: Borrow<U>,
    {
        let direction = match vertex_a.direction_to(&vertex_b) {
            Some(direction) => direction,
            None => return false,
        };
        let edges = match self.vertex_edges(vertex_a) {
            Some(edges) => edges,
            None => return false,
        };

        edges[direction] && {
            let neighbour = self.edges.first_neighbour(vertex_a, direction);
            neighbour.map(|found| found.map(|coord| coord.borrow()))
                == Some(vertex_b)
        }
    }

    pub fn insert_vertex(&mut self, vertex: CoordPair<T>)
    where
        T: Clone,
    {
        let mut edges = DirecMap::from_direcs(|_| false);

        for direction in Direction::iter() {
            if let Some(neighbour) =
                self.edges.first_neighbour(vertex.as_ref(), direction)
            {
                let neighbour_edges =
                    self.vertex_edges(neighbour).expect("Inconsistent graph");
                if neighbour_edges[!direction] {
                    edges[direction] = true;
                }
            }
        }

        self.edges.insert(vertex.clone(), edges);
    }

    pub fn connect<Ca, Cb>(&mut self, vertex_a: Ca, vertex_b: Cb) -> bool
    where
        Ca: CoordRef<T>,
        Cb: CoordRef<T>,
    {
        let vertex_a = vertex_a.as_coord_ref();
        let vertex_b = vertex_b.as_coord_ref();
        let direction =
            vertex_a.direction_to(&vertex_b).expect("no straight direction");

        if self.edges.first_neighbour(vertex_a, direction) != Some(vertex_b) {
            panic!("Vertices are not neighbours")
        }

        let mut edges = self.vertex_edges(vertex_a).expect("Invalid vertex");
        if edges[direction] {
            false
        } else {
            edges[direction] = true;
            let _ = self.edges.update(vertex_a, edges);
            let mut edges =
                self.vertex_edges(vertex_b).expect("Invalid vertex");
            edges[!direction] = true;
            let _ = self.edges.update(vertex_b, edges);
            true
        }
    }

    pub fn disconnect<Ca, Cb>(&mut self, vertex_a: Ca, vertex_b: Cb) -> bool
    where
        Ca: CoordRef<T>,
        Cb: CoordRef<T>,
    {
        let vertex_a = vertex_a.as_coord_ref();
        let vertex_b = vertex_b.as_coord_ref();
        let direction =
            vertex_a.direction_to(&vertex_b).expect("no straight direction");

        if self.edges.first_neighbour(vertex_a, direction) != Some(vertex_b) {
            panic!("Vertices are not neighbours")
        }

        let mut edges = self.vertex_edges(vertex_a).expect("Invalid vertex");
        if edges[direction] {
            edges[direction] = false;
            let _ = self.edges.update(vertex_a, edges);
            let mut edges =
                self.vertex_edges(vertex_b).expect("Invalid vertex");
            edges[!direction] = false;
            let _ = self.edges.update(vertex_b, edges);
            true
        } else {
            false
        }
    }

    pub fn remove_vertex<U>(&mut self, vertex: CoordPair<&U>) -> bool
    where
        U: Ord,
        T: Borrow<U> + Clone,
    {
        let vertex = vertex.as_coord_ref();
        let edges = match self.edges.remove(vertex) {
            Some(edges) => edges,
            None => return false,
        };
        for direction in Direction::iter() {
            if let Some((neighbour, neighbour_edges)) =
                self.edges.neighbours(vertex, direction).next().clone()
            {
                let neighbour = neighbour.cloned();
                let mut neighbour_edges = *neighbour_edges;
                if !edges[direction] || !edges[!direction] {
                    neighbour_edges[direction] = false;
                    let _ = self
                        .edges
                        .update::<T>(neighbour.as_ref(), neighbour_edges);
                }
            }
        }
        true
    }

    pub fn remove_with_edges<U>(&mut self, vertex: CoordPair<&U>) -> bool
    where
        U: Ord,
        T: Borrow<U> + Clone,
    {
        let vertex = vertex.as_coord_ref();
        let edges = match self.edges.remove(vertex) {
            Some(edges) => edges,
            None => return false,
        };
        for direction in Direction::iter() {
            if let Some((neighbour, neighbour_edges)) =
                self.edges.neighbours(vertex, direction).next().clone()
            {
                let neighbour = neighbour.cloned();
                let mut neighbour_edges = *neighbour_edges;
                if edges[direction] {
                    neighbour_edges[direction] = false;
                    let _ = self
                        .edges
                        .update::<T>(neighbour.as_ref(), neighbour_edges);
                }
            }
        }
        true
    }

    pub fn components(&self) -> Components<T> {
        Components {
            graph: self,
            unvisited: self.edges.rows().map(|(key, _)| key).collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Components<'graph, T>
where
    T: Ord,
{
    graph: &'graph Graph<T>,
    unvisited: BTreeSet<CoordPair<&'graph T>>,
}

impl<'graph, T> Iterator for Components<'graph, T>
where
    T: Ord + Clone,
{
    type Item = Graph<&'graph T>;

    fn next(&mut self) -> Option<Self::Item> {
        let start = *self.unvisited.iter().next()?;
        let mut stack = vec![start];
        let mut graph = Graph::new();

        graph.insert_vertex(start);
        while let Some(node) = stack.pop() {
            if self.unvisited.remove(&node) {
                for direction in Direction::iter() {
                    if let Some(neighbour) =
                        self.graph.edges.first_neighbour(node, direction)
                    {
                        graph.insert_vertex(neighbour);
                        graph.connect(node, neighbour);
                        stack.push(neighbour);
                    }
                }
            }
        }

        Some(graph)
    }
}
