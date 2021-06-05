use crate::{
    coord::{CoordPair, CoordRef},
    direc::{DirecMap, Direction},
    map::Map,
    set::Set,
};
//use priority_queue::PriorityQueue;

pub type VertexEdges = DirecMap<bool>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Graph<T>
where
    T: Ord,
{
    edges: Map<CoordPair<T>, VertexEdges>,
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

    pub fn vertex_edges<C>(&self, vertex: C) -> Option<VertexEdges>
    where
        C: CoordRef<T>,
    {
        self.edges.get(&vertex.as_coord_ref()).copied()
    }

    pub fn connects(
        &self,
        vertex_a: &CoordPair<T>,
        vertex_b: &CoordPair<T>,
    ) -> bool {
        let direc = match vertex_a.direction_to(&vertex_b) {
            Some(direc) => direc,
            None => return false,
        };
        let edges = match self.vertex_edges(vertex_a) {
            Some(edges) => edges,
            None => return false,
        };

        edges[direc]
            && self.vertices().neighbour(vertex_a, direc)
                == Some(vertex_b.as_ref())
    }

    pub fn insert_vertex(&mut self, vertex: CoordPair<T>)
    where
        T: Clone,
    {
        self.vertices.insert(vertex.clone());

        let mut edges =
            DirecMap { up: false, left: false, down: false, right: false };

        for direction in Direction::iter() {
            if let Some(neighbour) =
                self.vertices().neighbour(&vertex, direction)
            {
                let neighbour_edges =
                    self.vertex_edges(neighbour).expect("Inconsistent graph");
                if neighbour_edges[!direc] {
                    edges[direc] = true;
                }
            }
        }

        self.edges.insert(vertex, edges);
    }

    pub fn connect(
        &mut self,
        vertex_a: CoordPair<T>,
        vertex_b: CoordPair<T>,
    ) -> bool {
        let direc = vertex_a.direc_to(vertex_b).expect("no straight direction");

        if self.vertices().neighbour(vertex_a, direc) != Some(vertex_b) {
            panic!("Vertices are not neighbours")
        }

        let edges = self.edges.get_mut(&vertex_a).expect("Invalid vertex");
        if edges[direc] {
            false
        } else {
            edges[direc] = true;
            let edges = self.edges.get_mut(&vertex_b).expect("Invalid vertex");
            edges[!direc] = true;
            true
        }
    }

    pub fn disconnect(
        &mut self,
        vertex_a: CoordPair<T>,
        vertex_b: CoordPair<T>,
    ) -> bool {
        let direc = vertex_a.direc_to(vertex_b).expect("no straight direction");

        if self.vertices().neighbour(vertex_a, direc) != Some(vertex_b) {
            panic!("Vertices are not neighbours")
        }

        let edges = self.edges.get_mut(&vertex_a).expect("Invalid vertex");
        if !edges[direc] {
            false
        } else {
            edges[direc] = false;
            let edges = self.edges.get_mut(&vertex_b).expect("Invalid vertex");
            edges[!direc] = false;
            true
        }
    }

    pub fn remove_vertex(&mut self, vertex: CoordPair<T>) -> bool {
        let edges = match self.edges.remove(&vertex) {
            Some(edges) => edges,
            None => return false,
        };
        for direc in Direction::iter() {
            if let Some(neighbour) = self.vertices().neighbour(vertex, direc) {
                if !edges[direc] || !edges[!direc] {
                    let neighbour_edges = self
                        .edges
                        .get_mut(&neighbour)
                        .expect("Inconsistent graph");
                    neighbour_edges[direc] = false;
                }
            }
        }

        self.vertices.remove(vertex);
        true
    }

    pub fn remove_vertex_with_edges(&mut self, vertex: CoordPair<T>) -> bool {
        let edges = match self.edges.remove(&vertex) {
            Some(edges) => edges,
            None => return false,
        };
        for direc in Direction::iter() {
            if let Some(neighbour) = self.vertices().neighbour(vertex, direc) {
                if edges[direc] {
                    let neighbour_edges = self
                        .edges
                        .get_mut(&neighbour)
                        .expect("Inconsistent graph");
                    neighbour_edges[direc] = false;
                }
            }
        }

        self.vertices.remove(vertex);
        true
    }

    pub fn edges(&self) -> Edges<T> {
        Edges { graph: self, inner: self.edges.iter(), right: None, down: None }
    }

    pub fn components(&self) -> Components<T> {
        Components { graph: self, unvisited: self.vertices().rows().collect() }
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
pub struct Edges<'graph, T>
where
    T: Ord,
{
    graph: &'graph Graph<T>,
    inner: hash_map::Iter<'graph, CoordPair<T>, VertexEdges>,
    right: Option<CoordPair<T>>,
    down: Option<CoordPair<T>>,
}

impl<'graph, T> Iterator for Edges<'graph, T>
where
    T: Ord,
{
    type Item = (CoordPair<T>, CoordPair<T>);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(right) = self.right.take() {
                break Some((
                    right,
                    self.graph
                        .vertices()
                        .neighbour(right, Direction::Right)
                        .unwrap(),
                ));
            }
            if let Some(down) = self.down.take() {
                break Some((
                    down,
                    self.graph
                        .vertices()
                        .neighbour(down, Direction::Down)
                        .unwrap(),
                ));
            }
            let (&coord, &map) = self.inner.next()?;
            if map.right {
                self.right = Some(coord);
            }
            if map.down {
                self.down = Some(coord);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Components<'graph, T>
where
    T: Ord,
{
    graph: &'graph Graph<T>,
    unvisited: BTreeSet<CoordPair<T>>,
}

impl<'graph, T> Iterator for Components<'graph, T>
where
    T: Ord,
{
    type Item = Graph<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let start = *self.unvisited.range(..).next()?;
        let mut stack = vec![start];
        let mut graph = Graph::new();

        graph.insert_vertex(start);
        while let Some(node) = stack.pop() {
            if self.unvisited.remove(&node) {
                for direc in Direction::iter() {
                    if let Some(neighbour) =
                        self.graph.vertices().neighbour(node, direc)
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
