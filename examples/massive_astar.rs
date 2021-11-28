use gardiz::{
    coord::Vec2,
    graph::{Graph, PathMakerBuf},
    set::Set,
};
use rand::{
    rngs::StdRng,
    seq::{IteratorRandom, SliceRandom},
    Rng,
    SeedableRng,
};

const VERTICES_COUNT: usize = 20;
const EDGES_COUNT: usize = 124;

fn main() {
    let mut graph = Graph::new();
    let mut area = Set::new();
    for y in 100 .. 400 {
        for x in 500 .. 800 {
            area.insert(Vec2 { x, y });
        }
    }
    let mut rng = StdRng::seed_from_u64(0);
    generate_graph(&mut graph, &area, VERTICES_COUNT, EDGES_COUNT, &mut rng);
}

fn generate_graph<R>(
    target: &mut Graph<u16>,
    area: &Set<u16>,
    max_vertices: usize,
    max_edges: usize,
    rng: &mut R,
) where
    R: Rng + ?Sized,
{
    generate_vertices(target, area, max_vertices, rng);
    generate_edges(target, area, max_edges, rng);
}

fn generate_vertices<R>(
    target: &mut Graph<u16>,
    area: &Set<u16>,
    max_vertices: usize,
    rng: &mut R,
) where
    R: Rng + ?Sized,
{
    let points = area.rows().map(Vec2::copied).collect::<Vec<_>>();
    let amount = points.len().min(max_vertices);
    for &point in points.iter().choose_multiple(&mut *rng, amount) {
        target.create_vertex(point);
    }
}

fn generate_edges<R>(
    target: &mut Graph<u16>,
    area: &Set<u16>,
    max_edges: usize,
    rng: &mut R,
) where
    R: Rng + ?Sized,
{
    let mut path_maker_buf = PathMakerBuf::new();

    let mut vertices = target
        .vertices_edges()
        .rows()
        .map(|(point, _)| point.copied())
        .collect::<Vec<_>>();
    vertices.shuffle(&mut *rng);

    if let Some((&first, rest)) = vertices.split_first() {
        let mut prev = first;
        for &curr in rest {
            path_maker_buf.make_path(target, &prev, &curr, &2, |point| {
                area.contains(point.as_ref())
            });
            prev = curr;
        }
    }

    if vertices.len() >= 2 {
        for _ in 0 .. max_edges {
            let mut iter = vertices.choose_multiple(&mut *rng, 2);
            let first = *iter.next().unwrap();
            let second = *iter.next().unwrap();
            path_maker_buf.make_path(target, &first, &second, &2, |point| {
                area.contains(point.as_ref())
            });
        }
    }
}
