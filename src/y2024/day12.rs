use aoc24::Coordinate;
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
            let ix = visited.iter().map(|f| f.x).min().unwrap() - 1;
            let ax = visited.iter().map(|f| f.x).max().unwrap() + 1;
            let iy = visited.iter().map(|f| f.y).min().unwrap() - 1;
            let ay = visited.iter().map(|f| f.y).max().unwrap() + 1;
            let mat: Vec<Vec<u8>> = (ix..=ax)
                .map(|x| {
                    (iy..=ay)
                        .map(|y| {
                            let c = Coordinate::new(x, y);
                            if visited.contains(&c) {
                                return 1;
                            } else {
                                return 0;
                            }
                        })
                        .collect()
                })
                .collect();
            let mut count = 0;
            for i in 0..(mat.len() - 1) {
                for j in 0..(mat[i].len() - 1) {
                    let hh = mat[i][j] + mat[i + 1][j] + mat[i][j + 1] + mat[i + 1][j + 1];
                    if hh == 3 || hh == 1 {
                        count += 1;
                    }
                    if mat[i][j] == 0 && mat[i][j + 1] == 1 && mat[i + 1][j] == 1 && mat[i + 1][j + 1] == 0 {
                        count += 2;
                    }
                    if mat[i][j] == 1 && mat[i][j + 1] == 0 && mat[i + 1][j] == 0 && mat[i + 1][j + 1] == 1 {
                        count += 2;
                    }
                }
            }
            count * area
        })
        .sum()
}

pub fn run(inp: Vec<String>) -> (i64, i64) {
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
    (p1 as i64, p2 as i64)
}
