use super::Map;
use crate::{coord::CoordPair, direc::Direction};

#[test]
fn insert() {
    let mut map = Map::<i32, &str>::new();
    assert!(map.get(CoordPair { x: 9, y: -12 }.as_ref()).is_none());
    assert!(map.get(CoordPair { x: 8, y: -17 }.as_ref()).is_none());
    assert!(map.get(CoordPair { x: 250, y: 120 }.as_ref()).is_none());

    assert!(map.insert(CoordPair { x: 9, y: -12 }, "avocado").is_none());
    assert_eq!(
        map.get(CoordPair { x: 9, y: -12 }.as_ref()).unwrap(),
        &"avocado"
    );

    assert!(map.insert(CoordPair { x: 8, y: -17 }, "banana").is_none());
    assert_eq!(
        map.get(CoordPair { x: 9, y: -12 }.as_ref()).unwrap(),
        &"avocado"
    );
    assert_eq!(
        map.get(CoordPair { x: 8, y: -17 }.as_ref()).unwrap(),
        &"banana"
    );
    assert!(map.get(CoordPair { x: 250, y: 120 }.as_ref()).is_none());

    assert_eq!(map.insert(CoordPair { x: 8, y: -17 }, "nihil"), Some("banana"));
    assert_eq!(
        map.get(CoordPair { x: 9, y: -12 }.as_ref()).unwrap(),
        &"avocado"
    );
    assert_eq!(map.get(CoordPair { x: 8, y: -17 }.as_ref()).unwrap(), &"nihil");
    assert!(map.get(CoordPair { x: 250, y: 120 }.as_ref()).is_none());
}

#[test]
fn create() {
    let mut map = Map::<i32, &str>::new();
    assert!(map.get(CoordPair { x: 9, y: -12 }.as_ref()).is_none());
    assert!(map.get(CoordPair { x: 8, y: -17 }.as_ref()).is_none());
    assert!(map.get(CoordPair { x: 250, y: 120 }.as_ref()).is_none());

    assert!(map.create(CoordPair { x: 9, y: -12 }, "avocado"));
    assert_eq!(
        map.get(CoordPair { x: 9, y: -12 }.as_ref()).unwrap(),
        &"avocado"
    );

    assert!(map.create(CoordPair { x: 8, y: -17 }, "banana"));
    assert_eq!(
        map.get(CoordPair { x: 9, y: -12 }.as_ref()).unwrap(),
        &"avocado"
    );
    assert_eq!(
        map.get(CoordPair { x: 8, y: -17 }.as_ref()).unwrap(),
        &"banana"
    );
    assert!(map.get(CoordPair { x: 250, y: 120 }.as_ref()).is_none());

    assert!(!map.create(CoordPair { x: 8, y: -17 }, "nihil"));
    assert_eq!(
        map.get(CoordPair { x: 9, y: -12 }.as_ref()).unwrap(),
        &"avocado"
    );
    assert_eq!(
        map.get(CoordPair { x: 8, y: -17 }.as_ref()).unwrap(),
        &"banana"
    );
    assert!(map.get(CoordPair { x: 250, y: 120 }.as_ref()).is_none());
}

#[test]
fn remove() {
    let mut map = Map::<i32, &str>::new();
    assert!(map.get(CoordPair { x: 9, y: -12 }.as_ref()).is_none());
    assert!(map.get(CoordPair { x: 8, y: -17 }.as_ref()).is_none());
    assert!(map.get(CoordPair { x: 250, y: 120 }.as_ref()).is_none());

    assert!(map.insert(CoordPair { x: 9, y: -12 }, "avocado").is_none());
    assert!(map.insert(CoordPair { x: 8, y: -17 }, "banana").is_none());
    assert_eq!(map.remove(CoordPair { x: 8, y: -17 }.as_ref()), Some("banana"));

    assert_eq!(
        map.get(CoordPair { x: 9, y: -12 }.as_ref()).unwrap(),
        &"avocado"
    );
    assert!(map.get(CoordPair { x: 8, y: -17 }.as_ref()).is_none());

    assert_eq!(
        map.remove(CoordPair { x: 9, y: -12 }.as_ref()),
        Some("avocado")
    );
    assert!(map.get(CoordPair { x: 8, y: -17 }.as_ref()).is_none());
    assert!(map.get(CoordPair { x: 9, y: -12 }.as_ref()).is_none());
}

#[test]
fn update() {
    let mut map = Map::<i32, &str>::new();
    assert!(map.get(CoordPair { x: 9, y: -12 }.as_ref()).is_none());
    assert!(map.get(CoordPair { x: 8, y: -17 }.as_ref()).is_none());
    assert!(map.get(CoordPair { x: 250, y: 120 }.as_ref()).is_none());

    assert!(map.insert(CoordPair { x: 9, y: -12 }, "avocado").is_none());
    assert!(map.insert(CoordPair { x: 8, y: -17 }, "banana").is_none());

    assert_eq!(
        map.update(CoordPair { x: 8, y: -17 }.as_ref(), "nihil"),
        Ok("banana")
    );
    assert_eq!(
        map.get(CoordPair { x: 9, y: -12 }.as_ref()).unwrap(),
        &"avocado"
    );
    assert_eq!(map.get(CoordPair { x: 8, y: -17 }.as_ref()).unwrap(), &"nihil");

    assert!(map.get(CoordPair { x: 250, y: 120 }.as_ref()).is_none());

    assert_eq!(
        map.update(CoordPair { x: 800, y: 700 }.as_ref(), "nothing"),
        Err("nothing")
    );
}

#[test]
fn is_empty() {
    let mut map = Map::<i32, &str>::new();
    assert!(map.is_empty());
    map.insert(CoordPair { x: 5, y: 2 }, "testing is fun");
    assert!(!map.is_empty());
    map.insert(CoordPair { x: 2, y: -2 }, "hope it's still fun");
    assert!(!map.is_empty());
}

#[test]
fn contains() {
    let mut map = Map::<i32, &str>::new();
    assert!(!map.contains(CoordPair { x: 9, y: -12 }.as_ref()));
    assert!(!map.contains(CoordPair { x: 8, y: -17 }.as_ref()));
    assert!(!map.contains(CoordPair { x: 2, y: -47 }.as_ref()));

    map.insert(CoordPair { x: 9, y: -12 }, "avocado");
    assert!(map.contains(CoordPair { x: 9, y: -12 }.as_ref()));
    assert!(!map.contains(CoordPair { x: 8, y: -17 }.as_ref()));
    assert!(!map.contains(CoordPair { x: 2, y: -47 }.as_ref()));

    map.insert(CoordPair { x: 8, y: -17 }, "banana");
    assert!(map.contains(CoordPair { x: 9, y: -12 }.as_ref()));
    assert!(map.contains(CoordPair { x: 8, y: -17 }.as_ref()));
    assert!(!map.contains(CoordPair { x: 2, y: -47 }.as_ref()));
}

fn make_map() -> Map<i32, &'static str> {
    let mut map = Map::new();
    map.insert(CoordPair { x: 0, y: 2 }, "have");
    map.insert(CoordPair { x: 0, y: 5 }, "a");
    map.insert(CoordPair { x: 0, y: -2 }, "very");
    map.insert(CoordPair { x: 0, y: 569 }, "(really)");
    map.insert(CoordPair { x: 9, y: -2 }, "nice");
    map.insert(CoordPair { x: 100, y: -2 }, "day");
    map.insert(CoordPair { x: 1, y: -2 }, "and");
    map.insert(CoordPair { x: -1, y: -2 }, "(eh)");
    map.insert(CoordPair { x: 9, y: 3 }, "stand");
    map.insert(CoordPair { x: -51, y: 5 }, "still");
    map.insert(CoordPair { x: 2099, y: 4 }, "please");
    map
}

#[test]
fn neighbours() {
    let map = make_map();
    let collect = |key, direc| {
        map.neighbours(key, direc)
            .map(|(key, val)| (key.copied(), *val))
            .collect::<Vec<_>>()
    };

    assert_eq!(
        collect(CoordPair { x: 0, y: 2 }.as_ref(), Direction::Down),
        &[
            (CoordPair { x: 0, y: 5 }, "a"),
            (CoordPair { x: 0, y: 569 }, "(really)"),
        ],
    );

    assert_eq!(
        collect(CoordPair { x: 0, y: 2 }.as_ref(), Direction::Up),
        &[(CoordPair { x: 0, y: -2 }, "very")],
    );

    assert_eq!(
        collect(CoordPair { x: 0, y: 2 }.as_ref(), Direction::Left),
        &[],
    );
    assert_eq!(
        collect(CoordPair { x: 0, y: 2 }.as_ref(), Direction::Right),
        &[],
    );

    assert_eq!(
        collect(CoordPair { x: 0, y: -2 }.as_ref(), Direction::Down),
        &[
            (CoordPair { x: 0, y: 2 }, "have"),
            (CoordPair { x: 0, y: 5 }, "a"),
            (CoordPair { x: 0, y: 569 }, "(really)"),
        ],
    );

    assert_eq!(
        collect(CoordPair { x: 0, y: -2 }.as_ref(), Direction::Right),
        &[
            (CoordPair { x: 1, y: -2 }, "and"),
            (CoordPair { x: 9, y: -2 }, "nice"),
            (CoordPair { x: 100, y: -2 }, "day")
        ],
    );
    assert_eq!(
        collect(CoordPair { x: 0, y: -2 }.as_ref(), Direction::Left),
        &[(CoordPair { x: -1, y: -2 }, "(eh)")],
    );

    assert_eq!(
        collect(CoordPair { x: -51, y: 5 }.as_ref(), Direction::Right),
        &[(CoordPair { x: 0, y: 5 }, "a")],
    );

    assert_eq!(
        collect(CoordPair { x: 9, y: 3 }.as_ref(), Direction::Up),
        &[(CoordPair { x: 9, y: -2 }, "nice")],
    );

    assert_eq!(
        collect(CoordPair { x: 2099, y: 4 }.as_ref(), Direction::Up),
        &[],
    );
    assert_eq!(
        collect(CoordPair { x: 2099, y: 4 }.as_ref(), Direction::Down),
        &[],
    );
    assert_eq!(
        collect(CoordPair { x: 2099, y: 4 }.as_ref(), Direction::Left),
        &[],
    );
    assert_eq!(
        collect(CoordPair { x: 2099, y: 4 }.as_ref(), Direction::Right),
        &[],
    );
}

#[test]
fn first_neighbour() {
    let map = make_map();
    assert_eq!(
        map.first_neighbour(CoordPair { x: 0, y: 5 }.as_ref(), Direction::Up),
        Some(CoordPair { x: 0, y: 2 }.as_ref())
    );

    assert_eq!(
        map.first_neighbour(CoordPair { x: 0, y: 2 }.as_ref(), Direction::Down),
        Some(CoordPair { x: 0, y: 5 }.as_ref())
    );

    assert_eq!(
        map.first_neighbour(CoordPair { x: 0, y: 2 }.as_ref(), Direction::Left),
        None
    );

    assert_eq!(
        map.first_neighbour(
            CoordPair { x: 0, y: 2 }.as_ref(),
            Direction::Right
        ),
        None
    );

    assert_eq!(
        map.first_neighbour(
            CoordPair { x: 0, y: -2 }.as_ref(),
            Direction::Right
        ),
        Some(CoordPair { x: 1, y: -2 }.as_ref()),
    );
    assert_eq!(
        map.first_neighbour(
            CoordPair { x: 1, y: -2 }.as_ref(),
            Direction::Left
        ),
        Some(CoordPair { x: 0, y: -2 }.as_ref()),
    );
}

#[test]
fn last_neighbour() {
    let map = make_map();
    assert_eq!(
        map.last_neighbour(CoordPair { x: 0, y: 5 }.as_ref(), Direction::Up),
        Some(CoordPair { x: 0, y: -2 }.as_ref())
    );

    assert_eq!(
        map.last_neighbour(CoordPair { x: 0, y: 2 }.as_ref(), Direction::Down),
        Some(CoordPair { x: 0, y: 569 }.as_ref())
    );

    assert_eq!(
        map.last_neighbour(CoordPair { x: 0, y: 2 }.as_ref(), Direction::Left),
        None
    );

    assert_eq!(
        map.last_neighbour(CoordPair { x: 0, y: 2 }.as_ref(), Direction::Right),
        None
    );

    assert_eq!(
        map.last_neighbour(
            CoordPair { x: 0, y: -2 }.as_ref(),
            Direction::Right
        ),
        Some(CoordPair { x: 100, y: -2 }.as_ref()),
    );
    assert_eq!(
        map.last_neighbour(CoordPair { x: 1, y: -2 }.as_ref(), Direction::Left),
        Some(CoordPair { x: -1, y: -2 }.as_ref()),
    );
}

#[test]
fn rows() {
    let map = make_map();
    let rows = map
        .rows()
        .map(|(key, value)| (key.copied(), *value))
        .collect::<Vec<_>>();
    assert_eq!(
        rows,
        &[
            (CoordPair { x: -1, y: -2 }, "(eh)"),
            (CoordPair { x: 0, y: -2 }, "very"),
            (CoordPair { x: 1, y: -2 }, "and"),
            (CoordPair { x: 9, y: -2 }, "nice"),
            (CoordPair { x: 100, y: -2 }, "day"),
            (CoordPair { x: 0, y: 2 }, "have"),
            (CoordPair { x: 9, y: 3 }, "stand"),
            (CoordPair { x: 2099, y: 4 }, "please"),
            (CoordPair { x: -51, y: 5 }, "still"),
            (CoordPair { x: 0, y: 5 }, "a"),
            (CoordPair { x: 0, y: 569 }, "(really)"),
        ]
    );
}

#[test]
fn rows_empty() {
    let map = Map::<i16, f64>::new();
    let rows = map
        .rows()
        .map(|(key, value)| (key.copied(), *value))
        .collect::<Vec<_>>();
    assert_eq!(rows, &[]);
}

#[test]
fn columns() {
    let map = make_map();
    let columns = map
        .columns()
        .map(|(key, value)| (key.copied(), *value))
        .collect::<Vec<_>>();
    assert_eq!(
        columns,
        &[
            (CoordPair { x: -51, y: 5 }, "still"),
            (CoordPair { x: -1, y: -2 }, "(eh)"),
            (CoordPair { x: 0, y: -2 }, "very"),
            (CoordPair { x: 0, y: 2 }, "have"),
            (CoordPair { x: 0, y: 5 }, "a"),
            (CoordPair { x: 0, y: 569 }, "(really)"),
            (CoordPair { x: 1, y: -2 }, "and"),
            (CoordPair { x: 9, y: -2 }, "nice"),
            (CoordPair { x: 9, y: 3 }, "stand"),
            (CoordPair { x: 100, y: -2 }, "day"),
            (CoordPair { x: 2099, y: 4 }, "please"),
        ]
    );
}

#[test]
fn columns_empty() {
    let map = Map::<i16, f64>::new();
    let columns = map
        .columns()
        .map(|(key, value)| (key.copied(), *value))
        .collect::<Vec<_>>();
    assert_eq!(columns, &[]);
}
