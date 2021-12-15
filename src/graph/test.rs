use super::Graph;
use crate::{
    coord::Vec2,
    direc::{DirecMap, DirecVector, Direction},
};

#[test]
fn create() {
    let mut graph = Graph::<i32>::new();
    assert_eq!(graph.vertex_edges(Vec2 { x: 0, y: 0 }.as_ref()), None);
    assert!(graph.create_vertex(Vec2 { x: 0, y: 0 }));
    assert_eq!(
        graph.vertex_edges(Vec2 { x: 0, y: 0 }.as_ref()),
        Some(DirecMap::from_direcs(|_| false))
    );

    assert!(graph.create_vertex(Vec2 { x: 3, y: -1 }));
    assert_eq!(
        graph.vertex_edges(Vec2 { x: 3, y: -1 }.as_ref()),
        Some(DirecMap::from_direcs(|_| false))
    );

    assert!(graph.create_vertex(Vec2 { x: -9, y: 1400 }));
    assert_eq!(
        graph.vertex_edges(Vec2 { x: -9, y: 1400 }.as_ref()),
        Some(DirecMap::from_direcs(|_| false))
    );

    assert!(!graph.create_vertex(Vec2 { x: 0, y: 0 }));
    assert_eq!(
        graph.vertex_edges(Vec2 { x: 0, y: 0 }.as_ref()),
        Some(DirecMap::from_direcs(|_| false))
    );
    assert_eq!(
        graph.vertex_edges(Vec2 { x: 3, y: -1 }.as_ref()),
        Some(DirecMap::from_direcs(|_| false))
    );
    assert_eq!(
        graph.vertex_edges(Vec2 { x: -9, y: 1400 }.as_ref()),
        Some(DirecMap::from_direcs(|_| false))
    );
}

#[test]
fn from_vertices() {
    let graph = Graph::from_vertices(vec![
        Vec2 { x: 0, y: 0 },
        Vec2 { x: 3, y: -1 },
        Vec2 { x: -9, y: 1400 },
    ]);

    assert_eq!(
        graph.vertex_edges(Vec2 { x: 0, y: 0 }.as_ref()),
        Some(DirecMap::from_direcs(|_| false))
    );
    assert_eq!(
        graph.vertex_edges(Vec2 { x: 3, y: -1 }.as_ref()),
        Some(DirecMap::from_direcs(|_| false))
    );
    assert_eq!(
        graph.vertex_edges(Vec2 { x: -9, y: 1400 }.as_ref()),
        Some(DirecMap::from_direcs(|_| false))
    );
}

fn make_graph() -> Graph<i32> {
    let mut graph = Graph::from_vertices(vec![
        Vec2 { x: 0, y: 0 },
        Vec2 { x: 1, y: 0 },
        Vec2 { x: 0, y: -3 },
        Vec2 { x: 0, y: -8 },
        Vec2 { x: 3, y: -1 },
        Vec2 { x: 3, y: -9 },
        Vec2 { x: 3, y: -3 },
        Vec2 { x: 3, y: -17 },
        Vec2 { x: 1020, y: -3 },
        Vec2 { x: 1029, y: -3 },
        Vec2 { x: -9, y: 1401 },
        Vec2 { x: -9, y: 1400 },
        Vec2 { x: -9, y: 1399 },
    ]);

    assert!(graph
        .connect(Vec2 { x: 0, y: 0 }.as_ref(), Vec2 { x: 1, y: 0 }.as_ref()));
    assert!(graph
        .connect(Vec2 { x: 0, y: 0 }.as_ref(), Vec2 { x: 0, y: -3 }.as_ref()));
    assert!(graph
        .connect(Vec2 { x: 3, y: -3 }.as_ref(), Vec2 { x: 0, y: -3 }.as_ref()));
    graph.connect(Vec2 { x: 0, y: -8 }.as_ref(), Vec2 { x: 0, y: -3 }.as_ref());
    graph.connect(
        Vec2 { x: 3, y: -3 }.as_ref(),
        Vec2 { x: 1020, y: -3 }.as_ref(),
    );
    graph.connect(
        Vec2 { x: 1020, y: -3 }.as_ref(),
        Vec2 { x: 1029, y: -3 }.as_ref(),
    );
    graph.connect(Vec2 { x: 3, y: -3 }.as_ref(), Vec2 { x: 3, y: -1 }.as_ref());
    graph.connect(Vec2 { x: 3, y: -3 }.as_ref(), Vec2 { x: 3, y: -9 }.as_ref());
    graph
        .connect(Vec2 { x: 3, y: -17 }.as_ref(), Vec2 { x: 3, y: -9 }.as_ref());
    graph.connect(
        Vec2 { x: -9, y: 1401 }.as_ref(),
        Vec2 { x: -9, y: 1400 }.as_ref(),
    );

    graph
}

#[test]
fn are_connected() {
    let graph = make_graph();

    assert!(graph.are_connected(
        Vec2 { x: 0, y: 0 }.as_ref(),
        Vec2 { x: 1, y: 0 }.as_ref(),
    ));
    assert!(graph.are_connected(
        Vec2 { x: 1, y: 0 }.as_ref(),
        Vec2 { x: 0, y: 0 }.as_ref(),
    ));

    assert!(graph.are_connected(
        Vec2 { x: 0, y: 0 }.as_ref(),
        Vec2 { x: 0, y: -3 }.as_ref(),
    ));
    assert!(graph.are_connected(
        Vec2 { x: 0, y: -3 }.as_ref(),
        Vec2 { x: 0, y: 0 }.as_ref(),
    ));

    assert!(graph.are_connected(
        Vec2 { x: 3, y: -3 }.as_ref(),
        Vec2 { x: 0, y: -3 }.as_ref(),
    ));
    assert!(graph.are_connected(
        Vec2 { x: 0, y: -3 }.as_ref(),
        Vec2 { x: 3, y: -3 }.as_ref(),
    ));

    assert!(graph.are_connected(
        Vec2 { x: 3, y: -9 }.as_ref(),
        Vec2 { x: 3, y: -17 }.as_ref(),
    ));
    assert!(graph.are_connected(
        Vec2 { x: 3, y: -17 }.as_ref(),
        Vec2 { x: 3, y: -9 }.as_ref(),
    ));
}

#[test]
fn are_not_connected() {
    let graph = make_graph();

    assert!(!graph.are_connected(
        Vec2 { x: 3, y: -3 }.as_ref(),
        Vec2 { x: 1, y: 0 }.as_ref(),
    ));
    assert!(!graph.are_connected(
        Vec2 { x: 1, y: 0 }.as_ref(),
        Vec2 { x: 3, y: -3 }.as_ref(),
    ));

    assert!(!graph.are_connected(
        Vec2 { x: 3, y: -3 }.as_ref(),
        Vec2 { x: 1, y: 0 }.as_ref(),
    ));
    assert!(!graph.are_connected(
        Vec2 { x: 1, y: 0 }.as_ref(),
        Vec2 { x: 3, y: -3 }.as_ref(),
    ));
}

#[test]
fn connected_at() {
    let graph = make_graph();

    assert_eq!(
        graph.connected_at(Vec2 { x: 0, y: 0 }.as_ref(), Direction::Right),
        Some(Vec2 { x: 1, y: 0 }.as_ref()),
    );
    assert_eq!(
        graph.connected_at(Vec2 { x: 1, y: 0 }.as_ref(), Direction::Left),
        Some(Vec2 { x: 0, y: 0 }.as_ref()),
    );
    assert_eq!(
        graph.connected_at(Vec2 { x: 0, y: 0 }.as_ref(), Direction::Up),
        Some(Vec2 { x: 0, y: -3 }.as_ref()),
    );
    assert_eq!(
        graph.connected_at(Vec2 { x: 0, y: -3 }.as_ref(), Direction::Down),
        Some(Vec2 { x: 0, y: 0 }.as_ref()),
    );

    assert!(graph
        .connected_at(Vec2 { x: -9, y: 1399 }.as_ref(), Direction::Up)
        .is_none());
    assert!(graph
        .connected_at(Vec2 { x: -9, y: 1399 }.as_ref(), Direction::Left)
        .is_none());
    assert!(graph
        .connected_at(Vec2 { x: -9, y: 1399 }.as_ref(), Direction::Down)
        .is_none());
    assert!(graph
        .connected_at(Vec2 { x: -9, y: 1399 }.as_ref(), Direction::Right)
        .is_none());
}

#[test]
fn connections() {
    let graph = make_graph();
    let connections: Vec<_> = graph
        .connections()
        .map(|(va, vb)| (va.copied(), vb.copied()))
        .collect();

    assert_eq!(
        connections,
        &[
            (Vec2 { x: 3, y: -17 }, Vec2 { x: 3, y: -9 }),
            (Vec2 { x: 3, y: -9 }, Vec2 { x: 3, y: -3 }),
            (Vec2 { x: 0, y: -8 }, Vec2 { x: 0, y: -3 }),
            (Vec2 { x: 0, y: -3 }, Vec2 { x: 0, y: 0 }),
            (Vec2 { x: 0, y: -3 }, Vec2 { x: 3, y: -3 }),
            (Vec2 { x: 3, y: -3 }, Vec2 { x: 3, y: -1 }),
            (Vec2 { x: 3, y: -3 }, Vec2 { x: 1020, y: -3 }),
            (Vec2 { x: 1020, y: -3 }, Vec2 { x: 1029, y: -3 }),
            (Vec2 { x: 0, y: 0 }, Vec2 { x: 1, y: 0 }),
            (Vec2 { x: -9, y: 1400 }, Vec2 { x: -9, y: 1401 })
        ]
    );
}

#[test]
fn connect_twice() {
    let mut graph = make_graph();

    assert!(!graph
        .connect(Vec2 { x: 0, y: 0 }.as_ref(), Vec2 { x: 1, y: 0 }.as_ref(),));
    assert!(!graph
        .connect(Vec2 { x: 0, y: 0 }.as_ref(), Vec2 { x: 0, y: -3 }.as_ref(),));
    assert!(
        !graph.connect(
            Vec2 { x: 3, y: -3 }.as_ref(),
            Vec2 { x: 0, y: -3 }.as_ref(),
        )
    );

    assert!(graph.are_connected(
        Vec2 { x: 0, y: 0 }.as_ref(),
        Vec2 { x: 1, y: 0 }.as_ref(),
    ));
    assert!(graph.are_connected(
        Vec2 { x: 1, y: 0 }.as_ref(),
        Vec2 { x: 0, y: 0 }.as_ref(),
    ));

    assert!(graph.are_connected(
        Vec2 { x: 0, y: 0 }.as_ref(),
        Vec2 { x: 0, y: -3 }.as_ref(),
    ));
    assert!(graph.are_connected(
        Vec2 { x: 0, y: -3 }.as_ref(),
        Vec2 { x: 0, y: 0 }.as_ref(),
    ));

    assert!(graph.are_connected(
        Vec2 { x: 3, y: -3 }.as_ref(),
        Vec2 { x: 0, y: -3 }.as_ref(),
    ));
    assert!(graph.are_connected(
        Vec2 { x: 0, y: -3 }.as_ref(),
        Vec2 { x: 3, y: -3 }.as_ref(),
    ));

    assert!(!graph.are_connected(
        Vec2 { x: 3, y: -3 }.as_ref(),
        Vec2 { x: 1, y: 0 }.as_ref(),
    ));
    assert!(!graph.are_connected(
        Vec2 { x: 1, y: 0 }.as_ref(),
        Vec2 { x: 3, y: -3 }.as_ref(),
    ));

    assert!(graph.connect(
        Vec2 { x: -9, y: 1399 }.as_ref(),
        Vec2 { x: -9, y: 1400 }.as_ref(),
    ));

    assert!(graph.are_connected(
        Vec2 { x: -9, y: 1399 }.as_ref(),
        Vec2 { x: -9, y: 1400 }.as_ref(),
    ));
    assert!(graph.are_connected(
        Vec2 { x: -9, y: 1400 }.as_ref(),
        Vec2 { x: -9, y: 1399 }.as_ref(),
    ));
}

#[test]
fn disconnect() {
    let mut graph = make_graph();

    graph
        .disconnect(Vec2 { x: 0, y: 0 }.as_ref(), Vec2 { x: 1, y: 0 }.as_ref());
    graph.disconnect(
        Vec2 { x: 0, y: 0 }.as_ref(),
        Vec2 { x: 0, y: -3 }.as_ref(),
    );
    graph.disconnect(
        Vec2 { x: 3, y: -3 }.as_ref(),
        Vec2 { x: 0, y: -3 }.as_ref(),
    );

    assert!(!graph.are_connected(
        Vec2 { x: 0, y: 0 }.as_ref(),
        Vec2 { x: 1, y: 0 }.as_ref(),
    ));
    assert!(!graph.are_connected(
        Vec2 { x: 1, y: 0 }.as_ref(),
        Vec2 { x: 0, y: 0 }.as_ref(),
    ));

    assert!(!graph.are_connected(
        Vec2 { x: 0, y: 0 }.as_ref(),
        Vec2 { x: 0, y: -3 }.as_ref(),
    ));
    assert!(!graph.are_connected(
        Vec2 { x: 0, y: -3 }.as_ref(),
        Vec2 { x: 0, y: 0 }.as_ref(),
    ));

    assert!(!graph.are_connected(
        Vec2 { x: 3, y: -3 }.as_ref(),
        Vec2 { x: 0, y: -3 }.as_ref(),
    ));
    assert!(!graph.are_connected(
        Vec2 { x: 0, y: -3 }.as_ref(),
        Vec2 { x: 3, y: -3 }.as_ref(),
    ));

    assert!(!graph.are_connected(
        Vec2 { x: 3, y: -3 }.as_ref(),
        Vec2 { x: 1, y: 0 }.as_ref(),
    ));
    assert!(!graph.are_connected(
        Vec2 { x: 1, y: 0 }.as_ref(),
        Vec2 { x: 3, y: -3 }.as_ref(),
    ));

    assert!(graph.are_connected(
        Vec2 { x: 3, y: -9 }.as_ref(),
        Vec2 { x: 3, y: -17 }.as_ref(),
    ));
    assert!(graph.are_connected(
        Vec2 { x: 3, y: -17 }.as_ref(),
        Vec2 { x: 3, y: -9 }.as_ref(),
    ));
}

#[test]
fn extend_vertices() {
    let mut graph = make_graph();
    graph.extend_vertices(vec![
        Vec2 { x: -9, y: 1395 },
        Vec2 { x: 1, y: 9 },
        Vec2 { x: 8, y: 9 },
        Vec2 { x: 8, y: 7 },
        Vec2 { x: 8, y: 5 },
    ]);

    assert!(graph.vertices_edges().contains(Vec2 { x: -9, y: 1395 }.as_ref()));
    assert!(graph.vertices_edges().contains(Vec2 { x: 1, y: 9 }.as_ref()));
    assert!(graph.vertices_edges().contains(Vec2 { x: 8, y: 5 }.as_ref()));
    assert!(!graph.vertices_edges().contains(Vec2 { x: -8123, y: 0 }.as_ref()));
}

#[test]
fn extend_edges() {
    let mut graph = make_graph();
    graph.create_vertex(Vec2 { x: -9, y: 1395 });
    graph.create_vertex(Vec2 { x: 1, y: 9 });
    graph.create_vertex(Vec2 { x: 8, y: 9 });
    graph.create_vertex(Vec2 { x: 8, y: 7 });
    graph.create_vertex(Vec2 { x: 8, y: 5 });

    graph.extend_edges(vec![
        (Vec2 { x: 1, y: 9 }.as_ref(), Vec2 { x: 8, y: 9 }.as_ref()),
        (Vec2 { x: 8, y: 7 }.as_ref(), Vec2 { x: 8, y: 9 }.as_ref()),
        (Vec2 { x: -9, y: 1399 }.as_ref(), Vec2 { x: -9, y: 1400 }.as_ref()),
    ]);

    assert!(graph.are_connected(
        Vec2 { x: 1, y: 9 }.as_ref(),
        Vec2 { x: 8, y: 9 }.as_ref()
    ));

    assert!(graph.are_connected(
        Vec2 { x: 8, y: 9 }.as_ref(),
        Vec2 { x: 8, y: 7 }.as_ref()
    ));

    assert!(graph.are_connected(
        Vec2 { x: -9, y: 1399 }.as_ref(),
        Vec2 { x: -9, y: 1400 }.as_ref()
    ));

    assert!(!graph.are_connected(
        Vec2 { x: 8, y: 7 }.as_ref(),
        Vec2 { x: 7, y: 5 }.as_ref()
    ));
}

#[test]
fn from_verts_and_edges() {
    let graph = Graph::<u128>::from_verts_and_edges(
        vec![
            Vec2 { x: 5, y: 7 },
            Vec2 { x: 8, y: 6 },
            Vec2 { x: 0, y: 1 },
            Vec2 { x: 5, y: 6 },
        ],
        vec![
            (Vec2 { x: 5, y: 6 }.as_ref(), Vec2 { x: 8, y: 6 }.as_ref()),
            (Vec2 { x: 5, y: 6 }.as_ref(), Vec2 { x: 5, y: 7 }.as_ref()),
        ],
    );

    assert!(graph.are_connected(
        Vec2 { x: 5, y: 6 }.as_ref(),
        Vec2 { x: 8, y: 6 }.as_ref()
    ));

    assert!(graph.are_connected(
        Vec2 { x: 5, y: 6 }.as_ref(),
        Vec2 { x: 5, y: 7 }.as_ref()
    ));

    assert!(!graph.are_connected(
        Vec2 { x: 5, y: 6 }.as_ref(),
        Vec2 { x: 0, y: 1 }.as_ref()
    ));
}

#[test]
fn remove_vertex() {
    let mut graph = make_graph();
    graph.remove_vertex(Vec2 { x: 3, y: -3 }.as_ref());

    assert!(graph.are_connected(
        Vec2 { x: 0, y: 0 }.as_ref(),
        Vec2 { x: 1, y: 0 }.as_ref(),
    ));
    assert!(graph.are_connected(
        Vec2 { x: 1, y: 0 }.as_ref(),
        Vec2 { x: 0, y: 0 }.as_ref(),
    ));

    assert!(graph.are_connected(
        Vec2 { x: 0, y: 0 }.as_ref(),
        Vec2 { x: 0, y: -3 }.as_ref(),
    ));
    assert!(graph.are_connected(
        Vec2 { x: 0, y: -3 }.as_ref(),
        Vec2 { x: 0, y: 0 }.as_ref(),
    ));

    assert!(!graph.are_connected(
        Vec2 { x: 3, y: -3 }.as_ref(),
        Vec2 { x: 0, y: -3 }.as_ref(),
    ));
    assert!(!graph.are_connected(
        Vec2 { x: 0, y: -3 }.as_ref(),
        Vec2 { x: 3, y: -3 }.as_ref(),
    ));

    assert!(graph.are_connected(
        Vec2 { x: 0, y: -3 }.as_ref(),
        Vec2 { x: 1020, y: -3 }.as_ref(),
    ));
    assert!(graph.are_connected(
        Vec2 { x: 0, y: -3 }.as_ref(),
        Vec2 { x: 1020, y: -3 }.as_ref(),
    ));

    assert!(graph.are_connected(
        Vec2 { x: 3, y: -9 }.as_ref(),
        Vec2 { x: 3, y: -1 }.as_ref(),
    ));
    assert!(graph.are_connected(
        Vec2 { x: 3, y: -1 }.as_ref(),
        Vec2 { x: 3, y: -9 }.as_ref(),
    ));

    graph.remove_vertex(Vec2 { x: 0, y: -3 }.as_ref());

    assert!(graph.are_connected(
        Vec2 { x: 0, y: -8 }.as_ref(),
        Vec2 { x: 0, y: 0 }.as_ref(),
    ));
    assert!(graph.are_connected(
        Vec2 { x: 0, y: 0 }.as_ref(),
        Vec2 { x: 0, y: -8 }.as_ref(),
    ));

    let edges =
        graph.vertices_edges().get(Vec2 { x: 1020, y: -3 }.as_ref()).unwrap();
    assert!(!edges[Direction::Up]);
}

#[test]
fn remove_with_edges() {
    let mut graph = make_graph();
    graph.remove_with_edges(Vec2 { x: 3, y: -3 }.as_ref());

    assert!(graph.are_connected(
        Vec2 { x: 0, y: 0 }.as_ref(),
        Vec2 { x: 1, y: 0 }.as_ref(),
    ));
    assert!(graph.are_connected(
        Vec2 { x: 1, y: 0 }.as_ref(),
        Vec2 { x: 0, y: 0 }.as_ref(),
    ));

    assert!(graph.are_connected(
        Vec2 { x: 0, y: 0 }.as_ref(),
        Vec2 { x: 0, y: -3 }.as_ref(),
    ));
    assert!(graph.are_connected(
        Vec2 { x: 0, y: -3 }.as_ref(),
        Vec2 { x: 0, y: 0 }.as_ref(),
    ));

    assert!(!graph.are_connected(
        Vec2 { x: 3, y: -3 }.as_ref(),
        Vec2 { x: 0, y: -3 }.as_ref(),
    ));
    assert!(!graph.are_connected(
        Vec2 { x: 0, y: -3 }.as_ref(),
        Vec2 { x: 3, y: -3 }.as_ref(),
    ));

    assert!(!graph.are_connected(
        Vec2 { x: 0, y: -3 }.as_ref(),
        Vec2 { x: 1020, y: -3 }.as_ref(),
    ));
    assert!(!graph.are_connected(
        Vec2 { x: 1020, y: -3 }.as_ref(),
        Vec2 { x: 0, y: -3 }.as_ref(),
    ));

    assert!(!graph.are_connected(
        Vec2 { x: 3, y: -9 }.as_ref(),
        Vec2 { x: 3, y: -1 }.as_ref(),
    ));
    assert!(!graph.are_connected(
        Vec2 { x: 3, y: -1 }.as_ref(),
        Vec2 { x: 3, y: -9 }.as_ref(),
    ));

    graph.remove_with_edges(Vec2 { x: 0, y: -3 }.as_ref());

    assert!(!graph.are_connected(
        Vec2 { x: 0, y: -8 }.as_ref(),
        Vec2 { x: 0, y: 0 }.as_ref(),
    ));
    assert!(!graph.are_connected(
        Vec2 { x: 0, y: 0 }.as_ref(),
        Vec2 { x: 0, y: -8 }.as_ref(),
    ));

    let edges =
        graph.vertices_edges().get(Vec2 { x: 1020, y: -3 }.as_ref()).unwrap();
    assert!(!edges[Direction::Up]);
}

#[test]
fn components() {
    let graph = make_graph();
    let components = graph.components().collect::<Vec<_>>();

    let mut component1 = Graph::<&i32>::from_vertices(vec![
        Vec2 { x: 0, y: 0 }.as_ref(),
        Vec2 { x: 1, y: 0 }.as_ref(),
        Vec2 { x: 0, y: -3 }.as_ref(),
        Vec2 { x: 0, y: -8 }.as_ref(),
        Vec2 { x: 3, y: -1 }.as_ref(),
        Vec2 { x: 3, y: -9 }.as_ref(),
        Vec2 { x: 3, y: -3 }.as_ref(),
        Vec2 { x: 3, y: -17 }.as_ref(),
        Vec2 { x: 1020, y: -3 }.as_ref(),
        Vec2 { x: 1029, y: -3 }.as_ref(),
    ]);
    component1
        .connect(Vec2 { x: 0, y: 0 }.as_ref(), Vec2 { x: 1, y: 0 }.as_ref());
    component1
        .connect(Vec2 { x: 0, y: 0 }.as_ref(), Vec2 { x: 0, y: -3 }.as_ref());
    component1
        .connect(Vec2 { x: 3, y: -3 }.as_ref(), Vec2 { x: 0, y: -3 }.as_ref());
    component1
        .connect(Vec2 { x: 0, y: -8 }.as_ref(), Vec2 { x: 0, y: -3 }.as_ref());
    component1.connect(
        Vec2 { x: 3, y: -3 }.as_ref(),
        Vec2 { x: 1020, y: -3 }.as_ref(),
    );
    component1.connect(
        Vec2 { x: 1020, y: -3 }.as_ref(),
        Vec2 { x: 1029, y: -3 }.as_ref(),
    );
    component1
        .connect(Vec2 { x: 3, y: -3 }.as_ref(), Vec2 { x: 3, y: -1 }.as_ref());
    component1
        .connect(Vec2 { x: 3, y: -3 }.as_ref(), Vec2 { x: 3, y: -9 }.as_ref());
    component1
        .connect(Vec2 { x: 3, y: -17 }.as_ref(), Vec2 { x: 3, y: -9 }.as_ref());

    let component2 =
        Graph::<&i32>::from_vertices(vec![Vec2 { x: -9, y: 1399 }.as_ref()]);

    let mut component3 = Graph::<&i32>::from_vertices(vec![
        Vec2 { x: -9, y: 1401 }.as_ref(),
        Vec2 { x: -9, y: 1400 }.as_ref(),
    ]);
    component3.connect(
        Vec2 { x: -9, y: 1401 }.as_ref(),
        Vec2 { x: -9, y: 1400 }.as_ref(),
    );

    assert_eq!(components, &[component1, component2, component3]);
}

fn make_a_star_graph() -> Graph<u16> {
    let mut graph = Graph::new();
    graph.create_vertex(Vec2 { x: 0, y: 0 });
    graph.create_vertex(Vec2 { x: 5, y: 2 });
    graph.create_vertex(Vec2 { x: 5, y: 7 });
    graph.create_vertex(Vec2 { x: 2, y: 5 });
    graph
}

#[test]
fn a_star() {
    let mut graph = make_a_star_graph();
    let start = Vec2 { x: 0, y: 0 };
    let goal = Vec2 { x: 5, y: 7 };
    let penalty = 100;
    let valid_points = |point: &Vec2<u16>| point.x < 10 && point.y < 10;

    let directions =
        graph.make_path(&start, &goal, &penalty, valid_points).unwrap();

    assert_eq!(
        directions,
        vec![
            DirecVector { direction: Direction::Down, magnitude: 7 },
            DirecVector { direction: Direction::Right, magnitude: 5 },
        ]
    );

    let mut expected = make_a_star_graph();
    expected.create_vertex(Vec2 { x: 0, y: 7 });
    expected
        .connect(Vec2 { x: 0, y: 0 }.as_ref(), Vec2 { x: 0, y: 7 }.as_ref());
    expected
        .connect(Vec2 { x: 5, y: 7 }.as_ref(), Vec2 { x: 0, y: 7 }.as_ref());

    assert_eq!(graph, expected);
}

#[test]
fn a_star_irregular() {
    let mut graph = make_a_star_graph();
    let start = Vec2 { x: 0, y: 0 };
    let goal = Vec2 { x: 5, y: 7 };
    let penalty = 10;
    let valid_points = |point: &Vec2<u16>| {
        point.x < 4 && point.y < 3
            || point.x >= 3 && point.y >= 3 && point.x < 6 && point.y < 5
            || point.x >= 2 && point.y >= 5 && point.x < 5 && point.y < 6
            || point.x >= 1 && point.y >= 6 && point.x < 3 && point.y < 8
            || point.y >= 8 && point.x < 10 && point.y < 10
            || point.x >= 4 && point.y >= 7 && point.x < 12 && point.y < 12
    };

    let directions =
        graph.make_path(&start, &goal, &penalty, valid_points).unwrap();

    assert_eq!(
        directions,
        vec![
            // x = 0, y = 0
            DirecVector { direction: Direction::Right, magnitude: 3 },
            // x = 3, y = 0
            DirecVector { direction: Direction::Down, magnitude: 5 },
            // x = 3, y = 5
            DirecVector { direction: Direction::Left, magnitude: 1 },
            // x = 2, y = 5
            DirecVector { direction: Direction::Down, magnitude: 3 },
            // x = 2, y = 8
            DirecVector { direction: Direction::Right, magnitude: 3 },
            // x = 5, y = 8
            DirecVector { direction: Direction::Up, magnitude: 1 },
            // x = 5, y = 7
        ]
    );

    let mut expected = make_a_star_graph();
    expected.create_vertex(Vec2 { x: 3, y: 0 });
    expected.create_vertex(Vec2 { x: 3, y: 5 });
    expected.create_vertex(Vec2 { x: 2, y: 8 });
    expected.create_vertex(Vec2 { x: 5, y: 8 });
    expected
        .connect(Vec2 { x: 0, y: 0 }.as_ref(), Vec2 { x: 3, y: 0 }.as_ref());
    expected
        .connect(Vec2 { x: 3, y: 5 }.as_ref(), Vec2 { x: 3, y: 0 }.as_ref());
    expected
        .connect(Vec2 { x: 3, y: 5 }.as_ref(), Vec2 { x: 2, y: 5 }.as_ref());
    expected
        .connect(Vec2 { x: 2, y: 8 }.as_ref(), Vec2 { x: 2, y: 5 }.as_ref());
    expected
        .connect(Vec2 { x: 2, y: 8 }.as_ref(), Vec2 { x: 5, y: 8 }.as_ref());
    expected
        .connect(Vec2 { x: 5, y: 7 }.as_ref(), Vec2 { x: 5, y: 8 }.as_ref());

    assert_eq!(graph, expected);
}
