use aoc24::Coordinate;

use rustc_hash::FxHashMap;
use std::collections::BTreeSet;

const UP: Coordinate<i32> = Coordinate { x: -1, y: 0 };
const DOWN: Coordinate<i32> = Coordinate { x: 1, y: 0 };
const LEFT: Coordinate<i32> = Coordinate { x: 0, y: -1 };
const RIGHT: Coordinate<i32> = Coordinate { x: 0, y: 1 };

fn astar(available: &[Coordinate<i32>], start: Coordinate<i32>, end: Coordinate<i32>) -> (i64, i64) {
    let mut open: BTreeSet<(i64, Coordinate<i32>)> = BTreeSet::new();
    let mut visited: Vec<Coordinate<i32>> = vec![];
    let mut scores: FxHashMap<Coordinate<i32>, i64> = FxHashMap::default();
    open.insert((0, start));
    visited.push(start);
    scores.insert(start, 0);

    while let Some((sc, pos)) = open.pop_first() {
        if pos == end {
            break;
        }
        [UP, LEFT, DOWN, RIGHT]
            .iter()
            .map(|w| *w + pos)
            .filter(|w| available.contains(w))
            .for_each(|w| {
                if !visited.contains(&w) {
                    visited.push(w);
                    open.insert((sc + 1, w));
                    scores.insert(w, sc + 1);
                }
            });
    }

    let mut cheats = 0;
    let mut extra = 0;
    for v in visited.iter() {
        for vv in visited.iter() {
            let dist: i64 = ((v.x - vv.x).abs() + (v.y - vv.y).abs()) as i64;
            let sv = scores.get(v).unwrap();
            let svv = scores.get(vv).unwrap();
            let saved = svv - sv - dist;
            if svv - sv > 100 && dist == 2 {
                cheats += 1;
            }
            if saved >= 100 && dist <= 20 {
                extra += 1;
            }
        }
    }

    (cheats, extra)
}

pub fn run(inp: Vec<String>) -> (String, String) {
    let mut mapa: Vec<Coordinate<i32>> = vec![];
    let mut start: Coordinate<i32> = Coordinate { x: 0, y: 0 };
    let mut end: Coordinate<i32> = Coordinate { x: 0, y: 0 };
    let mut obstacles: Vec<Coordinate<i32>> = vec![];
    inp.iter().enumerate().for_each(|(i, w)| {
        w.chars().enumerate().for_each(|(j, c)| {
            let co: Coordinate<i32> = Coordinate {
                x: i as i32,
                y: j as i32,
            };
            match c {
                '.' => mapa.push(co),
                'S' => {
                    start = co.clone();
                    mapa.push(co);
                }
                'E' => {
                    end = co;
                    mapa.push(co);
                }
                '#' => {
                    if i != 0 && j != 0 && i < inp.len() - 1 && j < w.len() - 1 {
                        obstacles.push(co);
                    }
                }
                _ => (),
            }
        });
    });
    let (p1, p2) = astar(&mapa, start, end);

    (format!("{p1}"), format!("{p2}"))
}
