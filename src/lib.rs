//! Library for 2D geometric spaces (AKA "the plane"). This library focuses on
//! integer geometry, and is intended to be used in discrete 2D games.
//!
//! This crate supports 2D generic vectors (for representing points, sizes,
//! actual vectors, etc) with arithmetic operations on them (including dot
//! product and magnitude). It also defines "straight" basic directions in a
//! plane (i.e. up, left, down, right), using axis as indices and rectangles.
//!
//! It implements specialized maps and sets using points as keys. You could
//! think of maps as associating data with points in a plane, while sets could
//! be thought as specs of sub-planes. Both of them supports getting the first
//! or the last neighbour in a given direction.
//!
//! Finally, we have graphs whose vertices are points in the plane and they can
//! be connected (forming edges). It is useful to create planar graphs (i.e. no
//! edges cross), but the implementation won't prevent you from forming
//! non-planar graphs, although the graph still is in a 2D plane. The graph also
//! implements the "A Star" (or "A*") algorithm to make a path between two
//! points, using only a given region (creating vertices if necessary).
//!
//! As an example, here is the execution of A*:
//! ```rust
//! use gardiz::{
//!     coord::Vec2,
//!     graph::Graph,
//!     direc::{Direction, DirecVector},
//! };
//! use std::collections::HashSet;
//!
//! # fn main() {
//! // `i64` is the type of the coordinate of the points.
//! let mut graph = Graph::<i64>::new();
//! let start = Vec2 { x: -3, y: -3 };
//! let goal = Vec2 { x: 2, y: 4 };
//! graph.create_vertex(start);
//! graph.create_vertex(goal);
//!
//! let penalty = 2;
//!
//! let mut valid_points = HashSet::new();
//! for x in -3 .. 1 {
//!     for y in -3 .. 0 {
//!         valid_points.insert(Vec2 { x, y });
//!     }
//! }
//! for x in 0 .. 3 {
//!     for y in 0 .. 2 {
//!         valid_points.insert(Vec2 { x, y });
//!     }
//! }
//! for x in -1 .. 2 {
//!     for y in 2 .. 3 {
//!         valid_points.insert(Vec2 { x, y });
//!     }
//! }
//! for x in -2 .. 0 {
//!     for y in 3 .. 5 {
//!         valid_points.insert(Vec2 { x, y });
//!     }
//! }
//! for x in -3 .. 7 {
//!     for y in 5 .. 7 {
//!         valid_points.insert(Vec2 { x, y });
//!     }
//! }
//! for x in 1 .. 9 {
//!     for y in 4 .. 9 {
//!         valid_points.insert(Vec2 { x, y });
//!     }
//! }
//!
//! let mut expected = graph.clone();
//!
//! let directions = graph.make_path(
//!     &start,
//!     &goal,
//!     &penalty,
//!     |point| valid_points.contains(&point)
//! );
//!
//! assert_eq!(
//!     directions,
//!     Some(vec![
//!         // x = -3, y = -3
//!         DirecVector { direction: Direction::Right, magnitude: 3 },
//!         // x = 0, y = -3
//!         DirecVector { direction: Direction::Down, magnitude: 5 },
//!         // x = 0, y = 2
//!         DirecVector { direction: Direction::Left, magnitude: 1 },
//!         // x = -1, y = 2
//!         DirecVector { direction: Direction::Down, magnitude: 3 },
//!         // x = -1, y = 5
//!         DirecVector { direction: Direction::Right, magnitude: 3 },
//!         // x = 2, y = 5
//!         DirecVector { direction: Direction::Up, magnitude: 1 },
//!         // x = 2, y = 4
//!     ])
//! );
//!
//! expected.create_vertex(Vec2 { x: 0, y: -3 });
//! expected.create_vertex(Vec2 { x: 0, y: 2 });
//! expected.create_vertex(Vec2 { x: -1, y: 2 });
//! expected.create_vertex(Vec2 { x: -1, y: 5 });
//! expected.create_vertex(Vec2 { x: 2, y: 5 });
//! expected
//!     .connect(Vec2 { x: -3, y: -3 }.as_ref(), Vec2 { x: 0, y: -3 }.as_ref());
//! expected
//!     .connect(Vec2 { x: 0, y: 2 }.as_ref(), Vec2 { x: 0, y: -3 }.as_ref());
//! expected
//!     .connect(Vec2 { x: 0, y: 2 }.as_ref(), Vec2 { x: -1, y: 2 }.as_ref());
//! expected
//!     .connect(Vec2 { x: -1, y: 5 }.as_ref(), Vec2 { x: -1, y: 2 }.as_ref());
//! expected
//!     .connect(Vec2 { x: -1, y: 5 }.as_ref(), Vec2 { x: 2, y: 5 }.as_ref());
//! expected
//!     .connect(Vec2 { x: 2, y: 4 }.as_ref(), Vec2 { x: 2, y: 5 }.as_ref());
//! # }
//! ```

pub mod bits;
pub mod axis;
pub mod direc;
pub mod coord;
pub mod rect;
pub mod map;
pub mod set;
pub mod graph;
