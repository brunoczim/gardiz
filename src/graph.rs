use crate::{
    coord::{CoordPair, CoordRef},
    direc::{DirecMap, Direction},
    map::Map,
};
use std::collections::BTreeSet;
//use priority_queue::PriorityQueue;

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

    pub fn vertex_edges<C>(&self, vertex: C) -> Option<VertexEdges>
    where
        C: CoordRef<T>,
    {
        self.edges.get(&vertex.as_coord_ref()).copied()
    }

    pub fn connected<Ca, Cb>(&self, vertex_a: Ca, vertex_b: Cb) -> bool
    where
        Ca: CoordRef<T>,
        Cb: CoordRef<T>,
    {
        let vertex_a = vertex_a.as_coord_ref();
        let vertex_b = vertex_b.as_coord_ref();
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
            neighbour == Some(vertex_b)
        }
    }

    pub fn insert_vertex(&mut self, vertex: CoordPair<T>)
    where
        T: Clone,
    {
        let mut edges = DirecMap::from_direcs(|_| false);

        for direction in Direction::iter() {
            if let Some(neighbour) =
                self.edges.first_neighbour(&vertex, direction)
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
            self.edges.update(vertex_a, edges);
            let mut edges =
                self.vertex_edges(vertex_b).expect("Invalid vertex");
            edges[!direction] = true;
            self.edges.update(vertex_b, edges);
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
            self.edges.update(vertex_a, edges);
            let mut edges =
                self.vertex_edges(vertex_b).expect("Invalid vertex");
            edges[!direction] = false;
            self.edges.update(vertex_b, edges);
            true
        } else {
            false
        }
    }

    pub fn remove_vertex<C>(&mut self, vertex: C) -> bool
    where
        C: CoordRef<T>,
        T: Clone,
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
                    self.edges.update(&neighbour, neighbour_edges);
                }
            }
        }
        true
    }

    pub fn remove_with_edges<C>(&mut self, vertex: C) -> bool
    where
        C: CoordRef<T>,
        T: Clone,
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
                    self.edges.update(&neighbour, neighbour_edges);
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

    /*
    pub fn make_path(
        &mut self,
        start: CoordPair<T>,
        goal: CoordPair<T>,
        valid_points: &HashSet<CoordPair<T>>,
    ) -> Option<Vec<DirecVector<T>>> {
        let mut predecessors = HashMap::with_capacity(valid_points.len());

        let mut travelled = HashMap::with_capacity(valid_points.len());
        travelled.insert(start, AStarCost { distance: 0, turns: 0 });

        let mut points = PriorityQueue::with_capacity(valid_points.len());
        points.push(start, cmp::Reverse(AStarCost { distance: 0, turns: 0 }));

        while let Some((point, cmp::Reverse(cost))) = points.pop() {
            if point == goal {
                return Some(self.assemble_path(
                    start,
                    goal,
                    &predecessors,
                    cost,
                ));
            }
            self.eval_neighbours(
                goal,
                valid_points,
                point,
                &mut predecessors,
                &mut travelled,
                &mut points,
            );
        }

        None
    }

    fn assemble_path(
        &mut self,
        start: CoordPair<T>,
        goal: CoordPair<T>,
        predecessors: &HashMap<CoordPair<T>, CoordPair<T>>,
        cost: AStarCost,
    ) -> Vec<DirecVector<T>> {
        let mut steps = Vec::<DirecVector<T>>::with_capacity(
            cost.distance as usize - cost.turns as usize * 2,
        );
        let mut last_vertex = goal;
        let mut current = goal;

        while current != start {
            let prev = *predecessors.get(&current).unwrap();
            let direc = prev.direc_to(current).unwrap();

            match steps.last_mut() {
                Some(step) if step.direc == direc => step.magnitude += 1,
                _ => {
                    if last_vertex != current {
                        self.insert_vertex(current);
                        self.connect(last_vertex, current);
                        last_vertex = current;
                    }
                    steps.push(DirecVector { magnitude: 1, direc });
                },
            }

            if self.vertices().contains(prev) {
                self.connect(last_vertex, prev);
                last_vertex = prev;
            }

            current = prev;
        }

        steps.reverse();
        steps
    }

    fn eval_neighbours(
        &self,
        goal: CoordPair<T>,
        valid_points: &HashSet<CoordPair<T>>,
        point: CoordPair<T>,
        predecessors: &mut HashMap<CoordPair<T>, CoordPair<T>>,
        travelled: &mut HashMap<CoordPair<T>, AStarCost>,
        points: &mut PriorityQueue<CoordPair<T>, cmp::Reverse<AStarCost>>,
    ) {
        for direc in Direction::iter() {
            if let Some(neighbour) = point
                .move_by_direc(direc)
                .filter(|point| valid_points.contains(point))
            {
                let mut attempt = *travelled.get(&point).unwrap();
                attempt.distance += 1;

                let is_turning = predecessors.get(&point).map(|prev| {
                    prev.direc_to(point) != point.direc_to(neighbour)
                });
                if is_turning.unwrap_or(false) {
                    attempt.turns += 1;
                    // penalty
                    attempt.distance += 2;
                }

                if travelled
                    .get(&neighbour)
                    .map_or(true, |&cost| attempt < cost)
                {
                    predecessors.insert(neighbour, point);
                    travelled.insert(neighbour, attempt);
                    let heuristics =
                        neighbour.abs_distance(goal).fold(|a, b| a + b);
                    attempt.distance += heuristics;
                    points.push(neighbour, cmp::Reverse(attempt));
                }
            }
        }
    }
    */
}

/*
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
struct AStarCost<T> {
    distance: T,
    turns: T,
}
*/

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
