use super::Set;
use crate::{coord::Vec2, direc::Direction};

#[test]
fn insert() {
    let mut set = Set::<i32>::new();
    assert!(!set.contains(Vec2 { x: 9, y: -12 }.as_ref()));
    assert!(!set.contains(Vec2 { x: 8, y: -17 }.as_ref()));
    assert!(!set.contains(Vec2 { x: 250, y: 120 }.as_ref()));

    assert!(set.insert(Vec2 { x: 9, y: -12 }));
    assert!(set.contains(Vec2 { x: 9, y: -12 }.as_ref()));
    assert!(!set.contains(Vec2 { x: 8, y: -17 }.as_ref()));
    assert!(!set.contains(Vec2 { x: 250, y: 120 }.as_ref()));

    assert!(set.insert(Vec2 { x: 8, y: -17 }));
    assert!(set.contains(Vec2 { x: 9, y: -12 }.as_ref()),);
    assert!(set.contains(Vec2 { x: 8, y: -17 }.as_ref()));
    assert!(!set.contains(Vec2 { x: 250, y: 120 }.as_ref()));

    assert!(!set.insert(Vec2 { x: 8, y: -17 }));
    assert!(set.contains(Vec2 { x: 9, y: -12 }.as_ref()));
    assert!(set.contains(Vec2 { x: 8, y: -17 }.as_ref()));
    assert!(!set.contains(Vec2 { x: 250, y: 120 }.as_ref()));
}

#[test]
fn remove() {
    let mut set = Set::<i32>::new();
    assert!(!set.contains(Vec2 { x: 9, y: -12 }.as_ref()));
    assert!(!set.contains(Vec2 { x: 8, y: -17 }.as_ref()));
    assert!(!set.contains(Vec2 { x: 250, y: 120 }.as_ref()));

    assert!(set.insert(Vec2 { x: 9, y: -12 }));
    assert!(set.insert(Vec2 { x: 8, y: -17 }));
    assert!(set.remove(Vec2 { x: 8, y: -17 }.as_ref()));

    assert!(set.contains(Vec2 { x: 9, y: -12 }.as_ref()));
    assert!(!set.contains(Vec2 { x: 8, y: -17 }.as_ref()));

    assert!(set.remove(Vec2 { x: 9, y: -12 }.as_ref()));
    assert!(set.insert(Vec2 { x: 8, y: -17 }));
    assert!(set.insert(Vec2 { x: 9, y: -12 }));
}

#[test]
fn is_empty() {
    let mut set = Set::<i32>::new();
    assert!(set.is_empty());
    set.insert(Vec2 { x: 5, y: 2 });
    assert!(!set.is_empty());
    set.insert(Vec2 { x: 2, y: -2 });
    assert!(!set.is_empty());
}

#[test]
fn contains() {
    let mut set = Set::<i32>::new();
    assert!(!set.contains(Vec2 { x: 9, y: -12 }.as_ref()));
    assert!(!set.contains(Vec2 { x: 8, y: -17 }.as_ref()));
    assert!(!set.contains(Vec2 { x: 2, y: -47 }.as_ref()));

    set.insert(Vec2 { x: 9, y: -12 });
    assert!(set.contains(Vec2 { x: 9, y: -12 }.as_ref()));
    assert!(!set.contains(Vec2 { x: 8, y: -17 }.as_ref()));
    assert!(!set.contains(Vec2 { x: 2, y: -47 }.as_ref()));

    set.insert(Vec2 { x: 8, y: -17 });
    assert!(set.contains(Vec2 { x: 9, y: -12 }.as_ref()));
    assert!(set.contains(Vec2 { x: 8, y: -17 }.as_ref()));
    assert!(!set.contains(Vec2 { x: 2, y: -47 }.as_ref()));
}

fn make_set() -> Set<i32> {
    let mut set = Set::new();
    set.insert(Vec2 { x: 0, y: 2 });
    set.insert(Vec2 { x: 0, y: 5 });
    set.insert(Vec2 { x: 0, y: -2 });
    set.insert(Vec2 { x: 0, y: 569 });
    set.insert(Vec2 { x: 9, y: -2 });
    set.insert(Vec2 { x: 100, y: -2 });
    set.insert(Vec2 { x: 1, y: -2 });
    set.insert(Vec2 { x: -1, y: -2 });
    set.insert(Vec2 { x: 9, y: 3 });
    set.insert(Vec2 { x: -51, y: 5 });
    set.insert(Vec2 { x: 2099, y: 4 });
    set
}

#[test]
fn neighbours() {
    let set = make_set();
    let collect = |elem, direc| {
        set.neighbours(elem, direc)
            .map(|elem| elem.copied())
            .collect::<Vec<_>>()
    };

    assert_eq!(
        collect(Vec2 { x: 0, y: 2 }.as_ref(), Direction::Down),
        &[Vec2 { x: 0, y: 5 }, Vec2 { x: 0, y: 569 },],
    );

    assert_eq!(
        collect(Vec2 { x: 0, y: 2 }.as_ref(), Direction::Up),
        &[Vec2 { x: 0, y: -2 }],
    );

    assert_eq!(collect(Vec2 { x: 0, y: 2 }.as_ref(), Direction::Left), &[],);
    assert_eq!(collect(Vec2 { x: 0, y: 2 }.as_ref(), Direction::Right), &[],);

    assert_eq!(
        collect(Vec2 { x: 0, y: -2 }.as_ref(), Direction::Down),
        &[Vec2 { x: 0, y: 2 }, Vec2 { x: 0, y: 5 }, Vec2 { x: 0, y: 569 },],
    );

    assert_eq!(
        collect(Vec2 { x: 0, y: -2 }.as_ref(), Direction::Right),
        &[Vec2 { x: 1, y: -2 }, Vec2 { x: 9, y: -2 }, Vec2 { x: 100, y: -2 },],
    );
    assert_eq!(
        collect(Vec2 { x: 0, y: -2 }.as_ref(), Direction::Left),
        &[Vec2 { x: -1, y: -2 }],
    );

    assert_eq!(
        collect(Vec2 { x: -51, y: 5 }.as_ref(), Direction::Right),
        &[Vec2 { x: 0, y: 5 }],
    );

    assert_eq!(
        collect(Vec2 { x: 9, y: 3 }.as_ref(), Direction::Up),
        &[Vec2 { x: 9, y: -2 }],
    );

    assert_eq!(collect(Vec2 { x: 2099, y: 4 }.as_ref(), Direction::Up), &[]);
    assert_eq!(collect(Vec2 { x: 2099, y: 4 }.as_ref(), Direction::Down), &[]);
    assert_eq!(collect(Vec2 { x: 2099, y: 4 }.as_ref(), Direction::Left), &[]);
    assert_eq!(collect(Vec2 { x: 2099, y: 4 }.as_ref(), Direction::Right), &[]);
}

#[test]
fn neighbours_incl() {
    let set = make_set();
    let collect = |elem, direc| {
        set.neighbours_incl(elem, direc)
            .map(|elem| elem.copied())
            .collect::<Vec<_>>()
    };

    assert_eq!(
        collect(Vec2 { x: 0, y: 2 }.as_ref(), Direction::Down),
        &[Vec2 { x: 0, y: 2 }, Vec2 { x: 0, y: 5 }, Vec2 { x: 0, y: 569 },],
    );

    assert_eq!(
        collect(Vec2 { x: 0, y: 2 }.as_ref(), Direction::Up),
        &[Vec2 { x: 0, y: 2 }, Vec2 { x: 0, y: -2 }],
    );

    assert_eq!(
        collect(Vec2 { x: 0, y: 2 }.as_ref(), Direction::Left),
        &[Vec2 { x: 0, y: 2 }]
    );
    assert_eq!(
        collect(Vec2 { x: 0, y: 2 }.as_ref(), Direction::Right),
        &[Vec2 { x: 0, y: 2 }]
    );

    assert_eq!(
        collect(Vec2 { x: 0, y: -2 }.as_ref(), Direction::Down),
        &[
            Vec2 { x: 0, y: -2 },
            Vec2 { x: 0, y: 2 },
            Vec2 { x: 0, y: 5 },
            Vec2 { x: 0, y: 569 },
        ],
    );

    assert_eq!(
        collect(Vec2 { x: 0, y: -2 }.as_ref(), Direction::Right),
        &[
            Vec2 { x: 0, y: -2 },
            Vec2 { x: 1, y: -2 },
            Vec2 { x: 9, y: -2 },
            Vec2 { x: 100, y: -2 },
        ],
    );
    assert_eq!(
        collect(Vec2 { x: 0, y: -2 }.as_ref(), Direction::Left),
        &[Vec2 { x: 0, y: -2 }, Vec2 { x: -1, y: -2 }],
    );

    assert_eq!(
        collect(Vec2 { x: -51, y: 5 }.as_ref(), Direction::Right),
        &[Vec2 { x: -51, y: 5 }, Vec2 { x: 0, y: 5 }],
    );

    assert_eq!(
        collect(Vec2 { x: 9, y: 3 }.as_ref(), Direction::Up),
        &[Vec2 { x: 9, y: 3 }, Vec2 { x: 9, y: -2 }],
    );

    assert_eq!(
        collect(Vec2 { x: 2099, y: 4 }.as_ref(), Direction::Up),
        &[Vec2 { x: 2099, y: 4 }]
    );
    assert_eq!(
        collect(Vec2 { x: 2099, y: 4 }.as_ref(), Direction::Down),
        &[Vec2 { x: 2099, y: 4 }]
    );
    assert_eq!(
        collect(Vec2 { x: 2099, y: 4 }.as_ref(), Direction::Left),
        &[Vec2 { x: 2099, y: 4 }]
    );
    assert_eq!(
        collect(Vec2 { x: 2099, y: 4 }.as_ref(), Direction::Right),
        &[Vec2 { x: 2099, y: 4 }]
    );
}

#[test]
fn first_neighbour() {
    let set = make_set();
    assert_eq!(
        set.first_neighbour(Vec2 { x: 0, y: 5 }.as_ref(), Direction::Up),
        Some(Vec2 { x: 0, y: 2 }.as_ref())
    );

    assert_eq!(
        set.first_neighbour(Vec2 { x: 0, y: 2 }.as_ref(), Direction::Down),
        Some(Vec2 { x: 0, y: 5 }.as_ref())
    );

    assert_eq!(
        set.first_neighbour(Vec2 { x: 0, y: 2 }.as_ref(), Direction::Left),
        None
    );

    assert_eq!(
        set.first_neighbour(Vec2 { x: 0, y: 2 }.as_ref(), Direction::Right),
        None
    );

    assert_eq!(
        set.first_neighbour(Vec2 { x: 0, y: -2 }.as_ref(), Direction::Right),
        Some(Vec2 { x: 1, y: -2 }.as_ref()),
    );
    assert_eq!(
        set.first_neighbour(Vec2 { x: 1, y: -2 }.as_ref(), Direction::Left),
        Some(Vec2 { x: 0, y: -2 }.as_ref()),
    );
}

#[test]
fn last_neighbour() {
    let set = make_set();
    assert_eq!(
        set.last_neighbour(Vec2 { x: 0, y: 5 }.as_ref(), Direction::Up),
        Some(Vec2 { x: 0, y: -2 }.as_ref())
    );

    assert_eq!(
        set.last_neighbour(Vec2 { x: 0, y: 2 }.as_ref(), Direction::Down),
        Some(Vec2 { x: 0, y: 569 }.as_ref())
    );

    assert_eq!(
        set.last_neighbour(Vec2 { x: 0, y: 2 }.as_ref(), Direction::Left),
        None
    );

    assert_eq!(
        set.last_neighbour(Vec2 { x: 0, y: 2 }.as_ref(), Direction::Right),
        None
    );

    assert_eq!(
        set.last_neighbour(Vec2 { x: 0, y: -2 }.as_ref(), Direction::Right),
        Some(Vec2 { x: 100, y: -2 }.as_ref()),
    );
    assert_eq!(
        set.last_neighbour(Vec2 { x: 1, y: -2 }.as_ref(), Direction::Left),
        Some(Vec2 { x: -1, y: -2 }.as_ref()),
    );
}

#[test]
fn rows() {
    let set = make_set();
    let rows = set.rows().map(|elem| elem.copied()).collect::<Vec<_>>();
    assert_eq!(
        rows,
        &[
            Vec2 { x: -1, y: -2 },
            Vec2 { x: 0, y: -2 },
            Vec2 { x: 1, y: -2 },
            Vec2 { x: 9, y: -2 },
            Vec2 { x: 100, y: -2 },
            Vec2 { x: 0, y: 2 },
            Vec2 { x: 9, y: 3 },
            Vec2 { x: 2099, y: 4 },
            Vec2 { x: -51, y: 5 },
            Vec2 { x: 0, y: 5 },
            Vec2 { x: 0, y: 569 },
        ]
    );
}

#[test]
fn rows_empty() {
    let set = Set::<i16>::new();
    let rows = set.rows().map(|elem| elem.copied()).collect::<Vec<_>>();
    assert_eq!(rows, &[]);
}

#[test]
fn columns() {
    let set = make_set();
    let columns = set.columns().map(|elem| elem.copied()).collect::<Vec<_>>();
    assert_eq!(
        columns,
        &[
            Vec2 { x: -51, y: 5 },
            Vec2 { x: -1, y: -2 },
            Vec2 { x: 0, y: -2 },
            Vec2 { x: 0, y: 2 },
            Vec2 { x: 0, y: 5 },
            Vec2 { x: 0, y: 569 },
            Vec2 { x: 1, y: -2 },
            Vec2 { x: 9, y: -2 },
            Vec2 { x: 9, y: 3 },
            Vec2 { x: 100, y: -2 },
            Vec2 { x: 2099, y: 4 },
        ]
    );
}

#[test]
fn columns_empty() {
    let set = Set::<i16>::new();
    let columns = set.columns().map(|elem| elem.copied()).collect::<Vec<_>>();
    assert_eq!(columns, &[]);
}
