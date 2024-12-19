use aoc24::Coordinate;

use rustc_hash::FxHashSet;
use std::collections::BTreeSet;

const SIZE: i64 = 70;
const SIM: usize = 1024;
const UP: Coordinate<i64> = Coordinate { x: -1, y: 0 };
const DOWN: Coordinate<i64> = Coordinate { x: 1, y: 0 };
const LEFT: Coordinate<i64> = Coordinate { x: 0, y: -1 };
const RIGHT: Coordinate<i64> = Coordinate { x: 0, y: 1 };

fn astar(obstacles: &[Coordinate<i64>]) -> i64 {
    let mut open: BTreeSet<(i64, Coordinate<i64>)> = BTreeSet::new();
    let mut visited: FxHashSet<Coordinate<i64>> = FxHashSet::default();
    let end = Coordinate { x: SIZE, y: SIZE };
    open.insert((0, Coordinate { x: 0, y: 0 }));
    let mut tot = -1;

    while let Some((sc, pos)) = open.pop_first() {
        if pos == end {
            tot = sc;
            break;
        }
        [UP, LEFT, DOWN, RIGHT]
            .iter()
            .map(|w| *w + pos)
            .filter(|w| !obstacles.contains(w) && !w.out_of_bounds(SIZE + 1))
            .filter(|w| visited.insert(*w))
            .for_each(|w| {
                open.insert((sc + 1, w));
            });
    }

    tot
}

pub fn run(inp: Vec<String>) -> (String, String) {
    let list: Vec<Coordinate<i64>> = inp
        .iter()
        .map(|l| {
            let mut aux = l.split(',');
            Coordinate::<i64> {
                y: aux.next().unwrap().parse::<i64>().unwrap(),
                x: aux.next().unwrap().parse::<i64>().unwrap(),
            }
        })
        .collect();
    let p1 = astar(&list[0..SIM]);

    let mut itt = SIM;
    let mut upper = list.len();
    loop {
        let search = (upper + itt + 1) / 2;
        let p1 = astar(&list[0..search]);
        if p1 == -1 && itt == upper - 1 {
            break;
        }
        if p1 == -1 {
            upper = search;
        } else {
            itt = search;
        }
    }
    let co = list.get(itt).unwrap();
    (format!("{}", p1), format!("{},{}", co.y, co.x))
}
