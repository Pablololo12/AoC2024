use aoc24::Coordinate;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

const UP: Coordinate<i32> = Coordinate { x: -1, y: 0 };
const DOWN: Coordinate<i32> = Coordinate { x: 1, y: 0 };
const LEFT: Coordinate<i32> = Coordinate { x: 0, y: -1 };
const RIGHT: Coordinate<i32> = Coordinate { x: 0, y: 1 };

fn part1(
    start: Coordinate<i32>,
    key: char,
    mapa: &HashMap<Coordinate<i32>, char>,
    visited: &mut HashSet<Coordinate<i32>>,
) -> (u32, u32) {
    match start {
        s if Some(&key) != mapa.get(&s) => (1, 0),
        d if !visited.insert(d) => (0, 0),
        _ => ([UP, DOWN, LEFT, RIGHT]).iter().fold((0, 1), |(a, aa), d| {
            let (b, bb) = part1(start + *d, key, mapa, visited);
            (a + b, aa + bb)
        }),
    }
}

fn part2_aux(
    start: Coordinate<i32>,
    key: char,
    mapa: &HashMap<Coordinate<i32>, char>,
    visited: &mut HashSet<Coordinate<i32>>,
    thistime: &mut HashSet<Coordinate<i32>>,
) -> u32 {
    if Some(&key) != mapa.get(&start) || !visited.insert(start) {
        return 0;
    }
    thistime.insert(start);
    ([UP, DOWN, LEFT, RIGHT])
        .iter()
        .map(|w| part2_aux(start + *w, key, mapa, visited, thistime))
        .sum::<u32>()
        + 1
}

fn part2(mapa: &HashMap<Coordinate<i32>, char>) -> u32 {
    let mut total_v: HashSet<Coordinate<i32>> = HashSet::new();
    mapa.keys()
        .map(|w| {
            if total_v.contains(w) {
                return 0;
            }
            let mut visited: HashSet<Coordinate<i32>> = HashSet::new();
            let area = part2_aux(*w, *mapa.get(w).unwrap(), &mapa, &mut total_v, &mut visited);
            let (ix, ax) = visited.iter().map(|f| f.x).minmax().into_option().unwrap();
            let (iy, ay) = visited.iter().map(|f| f.y).minmax().into_option().unwrap();
            (ix - 1..=ax + 1)
                .map(|i| {
                    (iy - 1..=ay + 1)
                        .map(|j| {
                            let start = Coordinate { x: i, y: j };
                            let tl = visited.contains(&start);
                            let tr = visited.contains(&(start + RIGHT));
                            let bl = visited.contains(&(start + DOWN));
                            let br = visited.contains(&(start + DOWN + RIGHT));
                            let c = [tl, tr, bl, br].iter().filter(|w| **w).count();
                            if tl == br && tr == bl && tr != tl {
                                return 2;
                            } else {
                                return (c % 2) as u32;
                            }
                        })
                        .sum::<u32>()
                })
                .sum::<u32>()
                * area
        })
        .sum()
}

pub fn run(inp: Vec<String>) -> (String, String) {
    let mapa: HashMap<Coordinate<i32>, char> = inp
        .iter()
        .enumerate()
        .flat_map(|(i, l)| {
            l.chars()
                .enumerate()
                .map(move |(j, c)| (Coordinate::new(i as i32, j as i32), c))
        })
        .collect();

    let mut visited: HashSet<Coordinate<i32>> = HashSet::new();
    let p1: u64 = mapa
        .keys()
        .map(|w| part1(*w, *mapa.get(w).unwrap(), &mapa, &mut visited))
        .fold(0, |acc, (f, ff)| acc + (f * ff) as u64);

    let p2 = part2(&mapa);
    (format!("{}", p1), format!("{}", p2))
}
