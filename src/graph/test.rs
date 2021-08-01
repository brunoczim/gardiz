use super::Graph;
use crate::{
    coord::CoordPair,
    direc::{DirecMap, Direction},
};

#[test]
fn create() {
    let mut graph = Graph::<i32>::new();
    assert_eq!(graph.vertex_edges(CoordPair { x: 0, y: 0 }.as_ref()), None);
    assert!(graph.create_vertex(CoordPair { x: 0, y: 0 }));
    assert_eq!(
        graph.vertex_edges(CoordPair { x: 0, y: 0 }.as_ref()),
        Some(DirecMap::from_direcs(|_| false))
    );

    assert!(graph.create_vertex(CoordPair { x: 3, y: -1 }));
    assert_eq!(
        graph.vertex_edges(CoordPair { x: 3, y: -1 }.as_ref()),
        Some(DirecMap::from_direcs(|_| false))
    );

    assert!(graph.create_vertex(CoordPair { x: -9, y: 1400 }));
    assert_eq!(
        graph.vertex_edges(CoordPair { x: -9, y: 1400 }.as_ref()),
        Some(DirecMap::from_direcs(|_| false))
    );

    assert!(!graph.create_vertex(CoordPair { x: 0, y: 0 }));
    assert_eq!(
        graph.vertex_edges(CoordPair { x: 0, y: 0 }.as_ref()),
        Some(DirecMap::from_direcs(|_| false))
    );
    assert_eq!(
        graph.vertex_edges(CoordPair { x: 3, y: -1 }.as_ref()),
        Some(DirecMap::from_direcs(|_| false))
    );
    assert_eq!(
        graph.vertex_edges(CoordPair { x: -9, y: 1400 }.as_ref()),
        Some(DirecMap::from_direcs(|_| false))
    );
}

#[test]
fn from_vertices() {
    let graph = Graph::from_vertices(vec![
        CoordPair { x: 0, y: 0 },
        CoordPair { x: 3, y: -1 },
        CoordPair { x: -9, y: 1400 },
    ]);

    assert_eq!(
        graph.vertex_edges(CoordPair { x: 0, y: 0 }.as_ref()),
        Some(DirecMap::from_direcs(|_| false))
    );
    assert_eq!(
        graph.vertex_edges(CoordPair { x: 3, y: -1 }.as_ref()),
        Some(DirecMap::from_direcs(|_| false))
    );
    assert_eq!(
        graph.vertex_edges(CoordPair { x: -9, y: 1400 }.as_ref()),
        Some(DirecMap::from_direcs(|_| false))
    );
}

fn make_graph() -> Graph<i32> {
    let mut graph = Graph::from_vertices(vec![
        CoordPair { x: 0, y: 0 },
        CoordPair { x: 1, y: 0 },
        CoordPair { x: 0, y: -3 },
        CoordPair { x: 0, y: -8 },
        CoordPair { x: 3, y: -1 },
        CoordPair { x: 3, y: -9 },
        CoordPair { x: 3, y: -3 },
        CoordPair { x: 3, y: -17 },
        CoordPair { x: 1020, y: -3 },
        CoordPair { x: 1029, y: -3 },
        CoordPair { x: -9, y: 1401 },
        CoordPair { x: -9, y: 1400 },
        CoordPair { x: -9, y: 1399 },
    ]);

    graph.connect(
        CoordPair { x: 0, y: 0 }.as_ref(),
        CoordPair { x: 1, y: 0 }.as_ref(),
    );
    graph.connect(
        CoordPair { x: 0, y: 0 }.as_ref(),
        CoordPair { x: 0, y: -3 }.as_ref(),
    );
    graph.connect(
        CoordPair { x: 3, y: -3 }.as_ref(),
        CoordPair { x: 0, y: -3 }.as_ref(),
    );
    graph.connect(
        CoordPair { x: 0, y: -8 }.as_ref(),
        CoordPair { x: 0, y: -3 }.as_ref(),
    );
    graph.connect(
        CoordPair { x: 3, y: -3 }.as_ref(),
        CoordPair { x: 1020, y: -3 }.as_ref(),
    );
    graph.connect(
        CoordPair { x: 1020, y: -3 }.as_ref(),
        CoordPair { x: 1029, y: -3 }.as_ref(),
    );
    graph.connect(
        CoordPair { x: 3, y: -3 }.as_ref(),
        CoordPair { x: 3, y: -1 }.as_ref(),
    );
    graph.connect(
        CoordPair { x: 3, y: -3 }.as_ref(),
        CoordPair { x: 3, y: -9 }.as_ref(),
    );
    graph.connect(
        CoordPair { x: 3, y: -17 }.as_ref(),
        CoordPair { x: 3, y: -9 }.as_ref(),
    );
    graph.connect(
        CoordPair { x: -9, y: 1401 }.as_ref(),
        CoordPair { x: -9, y: 1400 }.as_ref(),
    );

    graph
}

#[test]
fn are_connected() {
    let graph = make_graph();

    assert!(graph.are_connected(
        CoordPair { x: 0, y: 0 }.as_ref(),
        CoordPair { x: 1, y: 0 }.as_ref(),
    ));
    assert!(graph.are_connected(
        CoordPair { x: 1, y: 0 }.as_ref(),
        CoordPair { x: 0, y: 0 }.as_ref(),
    ));

    assert!(graph.are_connected(
        CoordPair { x: 0, y: 0 }.as_ref(),
        CoordPair { x: 0, y: -3 }.as_ref(),
    ));
    assert!(graph.are_connected(
        CoordPair { x: 0, y: -3 }.as_ref(),
        CoordPair { x: 0, y: 0 }.as_ref(),
    ));

    assert!(graph.are_connected(
        CoordPair { x: 3, y: -3 }.as_ref(),
        CoordPair { x: 0, y: -3 }.as_ref(),
    ));
    assert!(graph.are_connected(
        CoordPair { x: 0, y: -3 }.as_ref(),
        CoordPair { x: 3, y: -3 }.as_ref(),
    ));

    assert!(graph.are_connected(
        CoordPair { x: 3, y: -9 }.as_ref(),
        CoordPair { x: 3, y: -17 }.as_ref(),
    ));
    assert!(graph.are_connected(
        CoordPair { x: 3, y: -17 }.as_ref(),
        CoordPair { x: 3, y: -9 }.as_ref(),
    ));
}

#[test]
fn are_not_connected() {
    let graph = make_graph();

    assert!(!graph.are_connected(
        CoordPair { x: 3, y: -3 }.as_ref(),
        CoordPair { x: 1, y: 0 }.as_ref(),
    ));
    assert!(!graph.are_connected(
        CoordPair { x: 1, y: 0 }.as_ref(),
        CoordPair { x: 3, y: -3 }.as_ref(),
    ));

    assert!(!graph.are_connected(
        CoordPair { x: 3, y: -3 }.as_ref(),
        CoordPair { x: 1, y: 0 }.as_ref(),
    ));
    assert!(!graph.are_connected(
        CoordPair { x: 1, y: 0 }.as_ref(),
        CoordPair { x: 3, y: -3 }.as_ref(),
    ));
}

#[test]
fn connected_at() {
    let graph = make_graph();

    assert_eq!(
        graph.connected_at(CoordPair { x: 0, y: 0 }.as_ref(), Direction::Right),
        Some(CoordPair { x: 1, y: 0 }.as_ref()),
    );
    assert_eq!(
        graph.connected_at(CoordPair { x: 1, y: 0 }.as_ref(), Direction::Left),
        Some(CoordPair { x: 0, y: 0 }.as_ref()),
    );
    assert_eq!(
        graph.connected_at(CoordPair { x: 0, y: 0 }.as_ref(), Direction::Up),
        Some(CoordPair { x: 0, y: -3 }.as_ref()),
    );
    assert_eq!(
        graph.connected_at(CoordPair { x: 0, y: -3 }.as_ref(), Direction::Down),
        Some(CoordPair { x: 0, y: 0 }.as_ref()),
    );

    assert!(graph
        .connected_at(CoordPair { x: -9, y: 1399 }.as_ref(), Direction::Up)
        .is_none());
    assert!(graph
        .connected_at(CoordPair { x: -9, y: 1399 }.as_ref(), Direction::Left)
        .is_none());
    assert!(graph
        .connected_at(CoordPair { x: -9, y: 1399 }.as_ref(), Direction::Down)
        .is_none());
    assert!(graph
        .connected_at(CoordPair { x: -9, y: 1399 }.as_ref(), Direction::Right)
        .is_none());
}

#[test]
fn disconnect() {
    let mut graph = make_graph();

    graph.disconnect(
        CoordPair { x: 0, y: 0 }.as_ref(),
        CoordPair { x: 1, y: 0 }.as_ref(),
    );
    graph.disconnect(
        CoordPair { x: 0, y: 0 }.as_ref(),
        CoordPair { x: 0, y: -3 }.as_ref(),
    );
    graph.disconnect(
        CoordPair { x: 3, y: -3 }.as_ref(),
        CoordPair { x: 0, y: -3 }.as_ref(),
    );

    assert!(!graph.are_connected(
        CoordPair { x: 0, y: 0 }.as_ref(),
        CoordPair { x: 1, y: 0 }.as_ref(),
    ));
    assert!(!graph.are_connected(
        CoordPair { x: 1, y: 0 }.as_ref(),
        CoordPair { x: 0, y: 0 }.as_ref(),
    ));

    assert!(!graph.are_connected(
        CoordPair { x: 0, y: 0 }.as_ref(),
        CoordPair { x: 0, y: -3 }.as_ref(),
    ));
    assert!(!graph.are_connected(
        CoordPair { x: 0, y: -3 }.as_ref(),
        CoordPair { x: 0, y: 0 }.as_ref(),
    ));

    assert!(!graph.are_connected(
        CoordPair { x: 3, y: -3 }.as_ref(),
        CoordPair { x: 0, y: -3 }.as_ref(),
    ));
    assert!(!graph.are_connected(
        CoordPair { x: 0, y: -3 }.as_ref(),
        CoordPair { x: 3, y: -3 }.as_ref(),
    ));

    assert!(!graph.are_connected(
        CoordPair { x: 3, y: -3 }.as_ref(),
        CoordPair { x: 1, y: 0 }.as_ref(),
    ));
    assert!(!graph.are_connected(
        CoordPair { x: 1, y: 0 }.as_ref(),
        CoordPair { x: 3, y: -3 }.as_ref(),
    ));

    assert!(graph.are_connected(
        CoordPair { x: 3, y: -9 }.as_ref(),
        CoordPair { x: 3, y: -17 }.as_ref(),
    ));
    assert!(graph.are_connected(
        CoordPair { x: 3, y: -17 }.as_ref(),
        CoordPair { x: 3, y: -9 }.as_ref(),
    ));
}

#[test]
fn remove_vertex() {
    let mut graph = make_graph();
    graph.remove_vertex(CoordPair { x: 3, y: -3 }.as_ref());

    assert!(graph.are_connected(
        CoordPair { x: 0, y: 0 }.as_ref(),
        CoordPair { x: 1, y: 0 }.as_ref(),
    ));
    assert!(graph.are_connected(
        CoordPair { x: 1, y: 0 }.as_ref(),
        CoordPair { x: 0, y: 0 }.as_ref(),
    ));

    assert!(graph.are_connected(
        CoordPair { x: 0, y: 0 }.as_ref(),
        CoordPair { x: 0, y: -3 }.as_ref(),
    ));
    assert!(graph.are_connected(
        CoordPair { x: 0, y: -3 }.as_ref(),
        CoordPair { x: 0, y: 0 }.as_ref(),
    ));

    assert!(!graph.are_connected(
        CoordPair { x: 3, y: -3 }.as_ref(),
        CoordPair { x: 0, y: -3 }.as_ref(),
    ));
    assert!(!graph.are_connected(
        CoordPair { x: 0, y: -3 }.as_ref(),
        CoordPair { x: 3, y: -3 }.as_ref(),
    ));

    assert!(graph.are_connected(
        CoordPair { x: 0, y: -3 }.as_ref(),
        CoordPair { x: 1020, y: -3 }.as_ref(),
    ));
    assert!(graph.are_connected(
        CoordPair { x: 0, y: -3 }.as_ref(),
        CoordPair { x: 1020, y: -3 }.as_ref(),
    ));

    assert!(graph.are_connected(
        CoordPair { x: 3, y: -9 }.as_ref(),
        CoordPair { x: 3, y: -1 }.as_ref(),
    ));
    assert!(graph.are_connected(
        CoordPair { x: 3, y: -1 }.as_ref(),
        CoordPair { x: 3, y: -9 }.as_ref(),
    ));

    graph.remove_vertex(CoordPair { x: 0, y: -3 }.as_ref());

    assert!(graph.are_connected(
        CoordPair { x: 0, y: -8 }.as_ref(),
        CoordPair { x: 0, y: 0 }.as_ref(),
    ));
    assert!(graph.are_connected(
        CoordPair { x: 0, y: 0 }.as_ref(),
        CoordPair { x: 0, y: -8 }.as_ref(),
    ));

    let edges =
        graph.edges().get(CoordPair { x: 1020, y: -3 }.as_ref()).unwrap();
    assert!(!edges[Direction::Up]);
}

#[test]
fn remove_with_edges() {
    let mut graph = make_graph();
    graph.remove_with_edges(CoordPair { x: 3, y: -3 }.as_ref());

    assert!(graph.are_connected(
        CoordPair { x: 0, y: 0 }.as_ref(),
        CoordPair { x: 1, y: 0 }.as_ref(),
    ));
    assert!(graph.are_connected(
        CoordPair { x: 1, y: 0 }.as_ref(),
        CoordPair { x: 0, y: 0 }.as_ref(),
    ));

    assert!(graph.are_connected(
        CoordPair { x: 0, y: 0 }.as_ref(),
        CoordPair { x: 0, y: -3 }.as_ref(),
    ));
    assert!(graph.are_connected(
        CoordPair { x: 0, y: -3 }.as_ref(),
        CoordPair { x: 0, y: 0 }.as_ref(),
    ));

    assert!(!graph.are_connected(
        CoordPair { x: 3, y: -3 }.as_ref(),
        CoordPair { x: 0, y: -3 }.as_ref(),
    ));
    assert!(!graph.are_connected(
        CoordPair { x: 0, y: -3 }.as_ref(),
        CoordPair { x: 3, y: -3 }.as_ref(),
    ));

    assert!(!graph.are_connected(
        CoordPair { x: 0, y: -3 }.as_ref(),
        CoordPair { x: 1020, y: -3 }.as_ref(),
    ));
    assert!(!graph.are_connected(
        CoordPair { x: 0, y: -3 }.as_ref(),
        CoordPair { x: 1020, y: -3 }.as_ref(),
    ));

    assert!(!graph.are_connected(
        CoordPair { x: 3, y: -9 }.as_ref(),
        CoordPair { x: 3, y: -1 }.as_ref(),
    ));
    assert!(!graph.are_connected(
        CoordPair { x: 3, y: -1 }.as_ref(),
        CoordPair { x: 3, y: -9 }.as_ref(),
    ));

    graph.remove_with_edges(CoordPair { x: 0, y: -3 }.as_ref());

    assert!(!graph.are_connected(
        CoordPair { x: 0, y: -8 }.as_ref(),
        CoordPair { x: 0, y: 0 }.as_ref(),
    ));
    assert!(!graph.are_connected(
        CoordPair { x: 0, y: 0 }.as_ref(),
        CoordPair { x: 0, y: -8 }.as_ref(),
    ));

    let edges =
        graph.edges().get(CoordPair { x: 1020, y: -3 }.as_ref()).unwrap();
    assert!(!edges[Direction::Up]);
}

#[test]
fn components() {
    let graph = make_graph();
    let components = graph.components().collect::<Vec<_>>();

    let mut component1 = Graph::<&i32>::from_vertices(vec![
        CoordPair { x: 0, y: 0 }.as_ref(),
        CoordPair { x: 1, y: 0 }.as_ref(),
        CoordPair { x: 0, y: -3 }.as_ref(),
        CoordPair { x: 0, y: -8 }.as_ref(),
        CoordPair { x: 3, y: -1 }.as_ref(),
        CoordPair { x: 3, y: -9 }.as_ref(),
        CoordPair { x: 3, y: -3 }.as_ref(),
        CoordPair { x: 3, y: -17 }.as_ref(),
        CoordPair { x: 1020, y: -3 }.as_ref(),
        CoordPair { x: 1029, y: -3 }.as_ref(),
    ]);
    component1.connect(
        CoordPair { x: 0, y: 0 }.as_ref(),
        CoordPair { x: 1, y: 0 }.as_ref(),
    );
    component1.connect(
        CoordPair { x: 0, y: 0 }.as_ref(),
        CoordPair { x: 0, y: -3 }.as_ref(),
    );
    component1.connect(
        CoordPair { x: 3, y: -3 }.as_ref(),
        CoordPair { x: 0, y: -3 }.as_ref(),
    );
    component1.connect(
        CoordPair { x: 0, y: -8 }.as_ref(),
        CoordPair { x: 0, y: -3 }.as_ref(),
    );
    component1.connect(
        CoordPair { x: 3, y: -3 }.as_ref(),
        CoordPair { x: 1020, y: -3 }.as_ref(),
    );
    component1.connect(
        CoordPair { x: 1020, y: -3 }.as_ref(),
        CoordPair { x: 1029, y: -3 }.as_ref(),
    );
    component1.connect(
        CoordPair { x: 3, y: -3 }.as_ref(),
        CoordPair { x: 3, y: -1 }.as_ref(),
    );
    component1.connect(
        CoordPair { x: 3, y: -3 }.as_ref(),
        CoordPair { x: 3, y: -9 }.as_ref(),
    );
    component1.connect(
        CoordPair { x: 3, y: -17 }.as_ref(),
        CoordPair { x: 3, y: -9 }.as_ref(),
    );

    let component2 =
        Graph::<&i32>::from_vertices(vec![
            CoordPair { x: -9, y: 1399 }.as_ref()
        ]);

    let mut component3 = Graph::<&i32>::from_vertices(vec![
        CoordPair { x: -9, y: 1401 }.as_ref(),
        CoordPair { x: -9, y: 1400 }.as_ref(),
    ]);
    component3.connect(
        CoordPair { x: -9, y: 1401 }.as_ref(),
        CoordPair { x: -9, y: 1400 }.as_ref(),
    );

    assert_eq!(components, &[component1, component2, component3]);
}
