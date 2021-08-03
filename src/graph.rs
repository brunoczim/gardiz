//! A graph of points in a plane.

// TODO: Make connections not overlap

#[cfg(test)]
mod test;

use crate::{
    coord::Vec2,
    direc::{DirecMap, Direction},
    map::Map,
};
use std::{borrow::Borrow, collections::BTreeSet};

/// The edges of a vertex. More specifically, at which direction the vertex is
/// connected?
pub type VertexEdges = DirecMap<bool>;

/// A graph of points in a plane.
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
    /// Creates a new empty graph.
    pub fn new() -> Self {
        Self { edges: Map::new() }
    }

    /// Creates the graph from a list of vertices (and no edges!).
    pub fn from_vertices<I>(vertices: I) -> Self
    where
        I: IntoIterator<Item = Vec2<T>>,
        T: Clone,
    {
        Self {
            edges: vertices
                .into_iter()
                .map(|vertex| (vertex, DirecMap::from_direcs(|_| false)))
                .collect(),
        }
    }

    /// Returns the underlying map of vertices to edge flags.
    pub fn edges(&self) -> &Map<T, VertexEdges> {
        &self.edges
    }

    /// Gets the edge flags of the given vertex, the vertex is in the graph in
    /// the first place.
    pub fn vertex_edges<U>(&self, vertex: Vec2<&U>) -> Option<VertexEdges>
    where
        U: Ord,
        T: Borrow<U>,
    {
        self.edges.get(vertex).copied()
    }

    /// Tests if the given two vertices are connected.
    pub fn are_connected<U>(
        &self,
        vertex_a: Vec2<&U>,
        vertex_b: Vec2<&U>,
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
            neighbour.map(Vec2::into_borrow) == Some(vertex_b)
        }
    }

    /// Gets the vertex connected with the given vertex in the given direction,
    /// if there is an edge in this direction.
    pub fn connected_at<U>(
        &self,
        vertex: Vec2<&U>,
        direction: Direction,
    ) -> Option<Vec2<&T>>
    where
        T: Borrow<U>,
        U: Ord,
    {
        if self.vertex_edges(vertex)?[direction] {
            self.edges.first_neighbour(vertex, direction)
        } else {
            None
        }
    }

    /// Creates a new vertex in the graph (without creating edges!). Returns if
    /// the vertex was really created (i.e. vertex not already there).
    pub fn create_vertex(&mut self, vertex: Vec2<T>) -> bool
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

        self.edges.create(vertex.clone(), edges)
    }

    /// Connects the given two vertices and returns if they were really
    /// connected (i.e. they were previously disconnected).
    pub fn connect<U>(&mut self, vertex_a: Vec2<&U>, vertex_b: Vec2<&U>) -> bool
    where
        U: Ord,
        T: Borrow<U>,
    {
        let direction =
            vertex_a.direction_to(&vertex_b).expect("no straight direction");

        let first_neighbour = self
            .edges
            .first_neighbour(vertex_a, direction)
            .map(|neighbour| neighbour.map(Borrow::borrow));

        if first_neighbour != Some(vertex_b) {
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

    /// Disconnects the given two vertices and returns if they were really
    /// disconnected (i.e. they were previously connected).
    pub fn disconnect<U>(
        &mut self,
        vertex_a: Vec2<&U>,
        vertex_b: Vec2<&U>,
    ) -> bool
    where
        U: Ord,
        T: Borrow<U>,
    {
        let direction =
            vertex_a.direction_to(&vertex_b).expect("no straight direction");

        let first_neighbour = self
            .edges
            .first_neighbour(vertex_a, direction)
            .map(|neighbour| neighbour.map(Borrow::borrow));

        if first_neighbour != Some(vertex_b) {
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

    /// Removes a vertex but attempts to connect edges between its neighbours,
    /// if the target vertex had edges in both directions. Returns if the vertex
    /// was really removed (i.e. it was in the graph).
    pub fn remove_vertex<U>(&mut self, vertex: Vec2<&U>) -> bool
    where
        U: Ord,
        T: Borrow<U> + Clone,
    {
        let edges = match self.edges.get(vertex).copied() {
            Some(edges) => edges,
            None => return false,
        };

        for direction in Direction::iter() {
            if let Some((neighbour, neighbour_edges)) =
                self.edges.first_neighbour_data(vertex, direction).clone()
            {
                let neighbour = neighbour.cloned();
                let mut neighbour_edges = *neighbour_edges;
                if !edges[!direction] {
                    neighbour_edges[!direction] = false;
                    let _ = self
                        .edges
                        .update::<T>(neighbour.as_ref(), neighbour_edges);
                }
            }
        }

        self.edges.remove(vertex);
        true
    }

    /// Removes a vertex and all its edges. Returns if the vertex was really
    /// removed (i.e. it was in the graph).
    pub fn remove_with_edges<U>(&mut self, vertex: Vec2<&U>) -> bool
    where
        U: Ord,
        T: Borrow<U> + Clone + std::fmt::Debug,
    {
        let edges = match self.edges.get(vertex).copied() {
            Some(edges) => edges,
            None => return false,
        };

        for direction in Direction::iter() {
            if let Some((neighbour, neighbour_edges)) =
                self.edges.first_neighbour_data(vertex, direction).clone()
            {
                let neighbour = neighbour.cloned();
                let mut neighbour_edges = *neighbour_edges;
                if edges[direction] {
                    neighbour_edges[!direction] = false;
                    let _ = self
                        .edges
                        .update::<T>(neighbour.as_ref(), neighbour_edges);
                }
            }
        }

        self.edges.remove(vertex);
        true
    }

    /// Creates iterator over connected components of the graph. E.g. each
    /// "island" in the graph makes a new subgraph (and an item of the
    /// iterator).
    pub fn components(&self) -> Components<T> {
        Components {
            graph: self,
            unvisited: self.edges.rows().map(|(key, _)| key).collect(),
        }
    }
}

/// Iterator over connected components of the graph. See [`Graph::components`].
#[derive(Debug, Clone)]
pub struct Components<'graph, T>
where
    T: Ord,
{
    graph: &'graph Graph<T>,
    unvisited: BTreeSet<Vec2<&'graph T>>,
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

        graph.create_vertex(start);
        while let Some(node) = stack.pop() {
            if self.unvisited.remove(&node) {
                for direction in Direction::iter() {
                    if let Some(neighbour) =
                        self.graph.connected_at(node, direction)
                    {
                        graph.create_vertex(neighbour);
                        graph.connect(node, neighbour);
                        stack.push(neighbour);
                    }
                }
            }
        }

        Some(graph)
    }
}
