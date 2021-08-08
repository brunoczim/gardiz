//! A graph of points in a plane.

#[cfg(test)]
mod test;

use crate::{
    axis::{self, Axis},
    bits::Distance,
    coord::Vec2,
    direc::{DirecMap, DirecVector, Direction},
    map::{Map, Rows},
};
use num::{CheckedAdd, CheckedSub, One, Zero};
use priority_queue::PriorityQueue;
use std::{
    borrow::Borrow,
    cmp,
    collections::{BTreeSet, HashMap},
    hash::Hash,
    iter::Peekable,
    ops::AddAssign,
};

/// The vertices_edges of a vertex. More specifically, at which direction the
/// vertex is connected?
pub type VertexEdges = DirecMap<bool>;

/// A graph of points in a plane. Two points can only be connected once with
/// each other or not connected at all (with each other), no pair of points can
/// be connected with each other more than once. Also, graphs might not be
/// necessarily planar, although they can (this means two edges can overlap).
/// Points can only be connected in "straight" 2D directions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Graph<T>
where
    T: Ord,
{
    vertices_edges: Map<T, VertexEdges>,
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
        Self { vertices_edges: Map::new() }
    }

    /// Creates the graph from a list of vertices (and no vertices_edges!).
    pub fn from_vertices<I>(vertices: I) -> Self
    where
        I: IntoIterator<Item = Vec2<T>>,
        T: Clone,
    {
        Self {
            vertices_edges: vertices
                .into_iter()
                .map(|vertex| (vertex, DirecMap::from_direcs(|_| false)))
                .collect(),
        }
    }

    /// Creates the graph from a list of vertices (and list of vertices-pair
    /// connected in vertices_edges).
    pub fn from_verts_and_edges<'vertex, U, I, J>(
        vertices: I,
        vertices_edges: J,
    ) -> Self
    where
        I: IntoIterator<Item = Vec2<T>>,
        T: Borrow<U> + Clone,
        U: 'vertex + Ord,
        J: IntoIterator<Item = (Vec2<&'vertex U>, Vec2<&'vertex U>)>,
    {
        let mut this = Self::from_vertices(vertices);
        this.extend_edges(vertices_edges);
        this
    }

    /// Extend the set of vertices from a given list of vertices, creating
    /// vertices when not existing already. The created vertices have no edges.
    pub fn extend_vertices<I>(&mut self, vertices: I)
    where
        I: IntoIterator<Item = Vec2<T>>,
        T: Clone,
    {
        self.vertices_edges.extend(
            vertices
                .into_iter()
                .map(|vertex| (vertex, DirecMap::from_direcs(|_| false))),
        );
    }

    /// Extends the graph edge list from a list of vertices-pair connected in
    /// vertices_edges.
    pub fn extend_edges<'vertex, U, I>(&mut self, vertices_edges: I)
    where
        U: 'vertex + Ord,
        T: Borrow<U>,
        I: IntoIterator<Item = (Vec2<&'vertex U>, Vec2<&'vertex U>)>,
    {
        for (vertex_a, vertex_b) in vertices_edges {
            self.connect(vertex_a, vertex_b);
        }
    }

    /// Returns the underlying map of vertices to edge flags.
    pub fn vertices_edges(&self) -> &Map<T, VertexEdges> {
        &self.vertices_edges
    }

    /// Gets the edge flags of the given vertex, the vertex is in the graph in
    /// the first place.
    pub fn vertex_edges<U>(&self, vertex: Vec2<&U>) -> Option<VertexEdges>
    where
        U: Ord,
        T: Borrow<U>,
    {
        self.vertices_edges.get(vertex).copied()
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
        let vertices_edges = match self.vertex_edges(vertex_a) {
            Some(vertices_edges) => vertices_edges,
            None => return false,
        };

        vertices_edges[direction] && {
            let neighbour =
                self.vertices_edges.first_neighbour(vertex_a, direction);
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
            self.vertices_edges.first_neighbour(vertex, direction)
        } else {
            None
        }
    }

    /// Creates a new vertex in the graph (without creating vertices_edges!).
    /// Returns if the vertex was really created (i.e. vertex not already
    /// there).
    pub fn create_vertex(&mut self, vertex: Vec2<T>) -> bool
    where
        T: Clone,
    {
        let mut vertices_edges = DirecMap::from_direcs(|_| false);

        for direction in Direction::iter() {
            if let Some(neighbour) =
                self.vertices_edges.first_neighbour(vertex.as_ref(), direction)
            {
                let neighbour_edges =
                    self.vertex_edges(neighbour).expect("Inconsistent graph");
                if neighbour_edges[!direction] {
                    vertices_edges[direction] = true;
                }
            }
        }

        self.vertices_edges.create(vertex.clone(), vertices_edges)
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
            .vertices_edges
            .first_neighbour(vertex_a, direction)
            .map(|neighbour| neighbour.map(Borrow::borrow));

        if first_neighbour != Some(vertex_b) {
            panic!("Vertices are not neighbours")
        }

        let mut vertices_edges =
            self.vertex_edges(vertex_a).expect("Invalid vertex");
        if vertices_edges[direction] {
            false
        } else {
            vertices_edges[direction] = true;
            let _ = self.vertices_edges.update(vertex_a, vertices_edges);
            let mut vertices_edges =
                self.vertex_edges(vertex_b).expect("Invalid vertex");
            vertices_edges[!direction] = true;
            let _ = self.vertices_edges.update(vertex_b, vertices_edges);
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
            .vertices_edges
            .first_neighbour(vertex_a, direction)
            .map(|neighbour| neighbour.map(Borrow::borrow));

        if first_neighbour != Some(vertex_b) {
            panic!("Vertices are not neighbours")
        }

        let mut vertices_edges =
            self.vertex_edges(vertex_a).expect("Invalid vertex");
        if vertices_edges[direction] {
            vertices_edges[direction] = false;
            let _ = self.vertices_edges.update(vertex_a, vertices_edges);
            let mut vertices_edges =
                self.vertex_edges(vertex_b).expect("Invalid vertex");
            vertices_edges[!direction] = false;
            let _ = self.vertices_edges.update(vertex_b, vertices_edges);
            true
        } else {
            false
        }
    }

    /// Iterator over the connections of this graph: pairs of vertices in an
    /// edge. Note that two vertices cannot be connected twice.
    pub fn connections(&self) -> Connections<T> {
        Connections {
            graph: self,
            vertices_edges: self.vertices_edges.rows().peekable(),
            axes: Axis::iter(),
        }
    }

    /// Removes a vertex but attempts to connect vertices_edges between its
    /// neighbours, if the target vertex had vertices_edges in both
    /// directions. Returns if the vertex was really removed (i.e. it was in
    /// the graph).
    pub fn remove_vertex<U>(&mut self, vertex: Vec2<&U>) -> bool
    where
        U: Ord,
        T: Borrow<U> + Clone,
    {
        let vertices_edges = match self.vertices_edges.get(vertex).copied() {
            Some(vertices_edges) => vertices_edges,
            None => return false,
        };

        for direction in Direction::iter() {
            if let Some((neighbour, neighbour_edges)) = self
                .vertices_edges
                .first_neighbour_data(vertex, direction)
                .clone()
            {
                let neighbour = neighbour.cloned();
                let mut neighbour_edges = *neighbour_edges;
                if !vertices_edges[!direction] {
                    neighbour_edges[!direction] = false;
                    let _ = self
                        .vertices_edges
                        .update::<T>(neighbour.as_ref(), neighbour_edges);
                }
            }
        }

        self.vertices_edges.remove(vertex);
        true
    }

    /// Removes a vertex and all its vertices_edges. Returns if the vertex was
    /// really removed (i.e. it was in the graph).
    pub fn remove_with_edges<U>(&mut self, vertex: Vec2<&U>) -> bool
    where
        U: Ord,
        T: Borrow<U> + Clone + std::fmt::Debug,
    {
        let vertices_edges = match self.vertices_edges.get(vertex).copied() {
            Some(vertices_edges) => vertices_edges,
            None => return false,
        };

        for direction in Direction::iter() {
            if let Some((neighbour, neighbour_edges)) = self
                .vertices_edges
                .first_neighbour_data(vertex, direction)
                .clone()
            {
                let neighbour = neighbour.cloned();
                let mut neighbour_edges = *neighbour_edges;
                if vertices_edges[direction] {
                    neighbour_edges[!direction] = false;
                    let _ = self
                        .vertices_edges
                        .update::<T>(neighbour.as_ref(), neighbour_edges);
                }
            }
        }

        self.vertices_edges.remove(vertex);
        true
    }

    /// Creates iterator over connected components of the graph. E.g. each
    /// "island" in the graph makes a new subgraph yielded by the iterator.
    pub fn components(&self) -> Components<T> {
        Components {
            graph: self,
            unvisited: self.vertices_edges.rows().map(|(key, _)| key).collect(),
        }
    }

    /// Makes a path from the given starting point till the "goal" point and
    /// creates intermediate points in the graph. The algorithm chooses the
    /// smallest path between the two points. It is also possible to specify
    /// a "penalty" added to the cost of paths when they turn. Recomended values
    /// for "penalty" are `0`, `1` or `2`. For minimizing turns, `2` is
    /// strongly recommended. The only points actually used are the ones
    /// validated by the given function `valid_points`.
    pub fn make_path<'points, F>(
        &mut self,
        start: &'points Vec2<T>,
        goal: &'points Vec2<T>,
        penalty: &'points T,
        valid_points: F,
    ) -> Option<Vec<DirecVector<T>>>
    where
        T: Clone + Hash,
        T: Zero + One + AddAssign + CheckedAdd + CheckedSub,
        T: AddAssign<&'points T>,
        F: FnMut(&Vec2<T>) -> bool,
    {
        PathMakerBuf::new().make_path(self, start, goal, penalty, valid_points)
    }
}

/// A buffer for an A* search algorithm useful for saving a few deallocations
/// and allocations when performing lots of searches. See [`Graph::make_path`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathMakerBuf<T>
where
    T: Clone + Hash + Ord,
    T: Zero + One + AddAssign + CheckedAdd + CheckedSub,
{
    predecessors: HashMap<Vec2<T>, Vec2<T>>,
    travelled: HashMap<Vec2<T>, Cost<T>>,
    points: PriorityQueue<Vec2<T>, cmp::Reverse<Cost<T>>>,
}

impl<T> Default for PathMakerBuf<T>
where
    T: Clone + Hash + Ord,
    T: Zero + One + AddAssign + CheckedAdd + CheckedSub,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> PathMakerBuf<T>
where
    T: Clone + Hash + Ord,
    T: Zero + One + AddAssign + CheckedAdd + CheckedSub,
{
    /// Creates a new empty path maker buffer.
    pub fn new() -> Self {
        Self {
            predecessors: HashMap::new(),
            travelled: HashMap::new(),
            points: PriorityQueue::new(),
        }
    }

    /// Performs the A* search algorithm using this buffer. See
    /// [`Graph::make_path`].
    pub fn make_path<'graph, 'points, F>(
        &mut self,
        graph: &'graph mut Graph<T>,
        start: &'points Vec2<T>,
        goal: &'points Vec2<T>,
        penalty: &'points T,
        valid_points: F,
    ) -> Option<Vec<DirecVector<T>>>
    where
        T: Clone,
        T: Zero + One + AddAssign + AddAssign<&'points T>,
        F: FnMut(&Vec2<T>) -> bool,
    {
        let mut call =
            PathMakerCall::new(self, graph, start, goal, penalty, valid_points);
        let path = call.run();
        self.travelled.clear();
        self.predecessors.clear();
        self.points.clear();
        path
    }
}

#[derive(Debug, PartialEq, Eq)]
struct PathMakerCall<'maker, 'graph, 'points, T, F>
where
    T: Clone + Hash + Ord,
    T: Zero + One,
    T: AddAssign + CheckedAdd + CheckedSub + AddAssign<&'points T>,
    F: FnMut(&Vec2<T>) -> bool,
    'graph: 'maker,
{
    buf: &'maker mut PathMakerBuf<T>,
    graph: &'graph mut Graph<T>,
    start: &'points Vec2<T>,
    goal: &'points Vec2<T>,
    penalty: &'points T,
    valid_points: F,
}

impl<'maker, 'graph, 'points, T, F> PathMakerCall<'maker, 'graph, 'points, T, F>
where
    T: Clone + Hash + Ord,
    T: Zero + One,
    T: AddAssign + CheckedAdd + CheckedSub + AddAssign<&'points T>,
    F: FnMut(&Vec2<T>) -> bool,
{
    fn new(
        buf: &'maker mut PathMakerBuf<T>,
        graph: &'graph mut Graph<T>,
        start: &'points Vec2<T>,
        goal: &'points Vec2<T>,
        penalty: &'points T,
        valid_points: F,
    ) -> Self {
        let this = Self { buf, graph, start, goal, penalty, valid_points };
        this.buf.travelled.insert(this.start.clone(), Cost::new());
        this.buf.points.push(this.start.clone(), cmp::Reverse(Cost::new()));
        this
    }

    fn run(&mut self) -> Option<Vec<DirecVector<T>>> {
        loop {
            let (current, cmp::Reverse(cost)) = self.buf.points.pop()?;

            if current == *self.goal {
                break Some(self.assemble_path(cost));
            }

            self.eval_neighbours(current);
        }
    }

    fn assemble_path(&mut self, _cost: Cost<T>) -> Vec<DirecVector<T>> {
        let mut steps = Vec::<DirecVector<_>>::new();
        let mut last_vertex = self.goal;
        let mut current = self.goal;

        while current != self.start {
            let prev = self.buf.predecessors.get(current).unwrap();
            let direction = prev.direction_to(current).unwrap();

            match steps.last_mut() {
                Some(step) if step.direction == direction => {
                    step.magnitude += T::one()
                },
                _ => {
                    if last_vertex != current {
                        self.graph.create_vertex((*current).clone());
                        self.graph.connect::<T>(
                            last_vertex.as_ref(),
                            current.as_ref(),
                        );
                        last_vertex = current;
                    }
                    steps.push(DirecVector { magnitude: T::one(), direction });
                },
            }

            if self.graph.vertices_edges().contains::<T>(prev.as_ref()) {
                self.graph.connect::<T>(last_vertex.as_ref(), prev.as_ref());
                last_vertex = prev;
            }

            current = prev;
        }

        steps.reverse();
        steps
    }

    fn eval_neighbours(&mut self, current: Vec2<T>) {
        for direction in Direction::iter() {
            if let Some(neighbour) = current
                .clone()
                .checked_move(direction)
                .filter(|point| (self.valid_points)(point))
            {
                let mut attempt =
                    self.buf.travelled.get(&current).unwrap().clone();
                attempt.distance += T::one();

                let is_turning = self
                    .buf
                    .predecessors
                    .get(&current)
                    .map(|prev| prev.direction_to(&current) != Some(direction))
                    .unwrap_or(false);

                if is_turning {
                    attempt.turns += T::one();
                    attempt.distance += self.penalty;
                }

                if self
                    .buf
                    .travelled
                    .get(&neighbour)
                    .map_or(true, |cost| attempt < *cost)
                {
                    self.buf
                        .predecessors
                        .insert(neighbour.clone(), current.clone());
                    self.buf
                        .travelled
                        .insert(neighbour.clone(), attempt.clone());
                    let heuristics = neighbour
                        .clone()
                        .zip_with(self.goal.clone(), Distance::distance)
                        .fold(T::zero(), |coord_a, coord_b| coord_a + coord_b);
                    attempt.distance += heuristics;
                    self.buf.points.push(neighbour, cmp::Reverse(attempt));
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
struct Cost<T> {
    distance: T,
    turns: T,
}

impl<T> Cost<T> {
    fn new() -> Self
    where
        T: Zero,
    {
        Self { distance: T::zero(), turns: T::zero() }
    }
}

/// Iterator over the connections of this graph pairs of vertices in an edge.
/// See [`Graph::connections`].
#[derive(Debug, Clone)]
pub struct Connections<'graph, T>
where
    T: Ord,
{
    graph: &'graph Graph<T>,
    vertices_edges: Peekable<Rows<'graph, T, VertexEdges>>,
    axes: axis::Iter,
}

impl<'graph, T> Iterator for Connections<'graph, T>
where
    T: Ord,
{
    type Item = (Vec2<&'graph T>, Vec2<&'graph T>);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (vertex, _) = self.vertices_edges.peek().copied()?;
            match self.axes.next().map(Direction::from_axis_pos) {
                Some(direction) => {
                    if let Some(neighbour) =
                        self.graph.connected_at(vertex, direction)
                    {
                        break Some((vertex, neighbour));
                    }
                },
                None => {
                    self.vertices_edges.next()?;
                    self.axes = Axis::iter();
                },
            }
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
