use std::collections::VecDeque;

use aoc24::Coordinate;

fn part1(start: Coordinate<usize>, map: &Vec<Vec<u8>>) -> u32 {
    let max = map.len();
    let vertical = Coordinate::new(1, 0);
    let horizontal = Coordinate::new(0, 1);
    let mut visited: Vec<Coordinate<usize>> = vec![];
    let mut open: VecDeque<Coordinate<usize>> = VecDeque::new();
    let mut count = 0;
    visited.push(start);
    open.push_back(start);
    while let Some(o) = open.pop_front() {
        let val = map[o.x][o.y];
        if val == 9 {
            count += 1;
            continue;
        }
        let u = o - vertical;
        if !u.out_of_bounds(max) && map[u.x][u.y] - val == 1 && !visited.iter().any(|k| *k == u) {
            visited.push(u);
            open.push_back(u);
        }
        let d = o + vertical;
        if !d.out_of_bounds(max) && map[d.x][d.y] - val == 1 && !visited.iter().any(|k| *k == d) {
            visited.push(d);
            open.push_back(d);
        }
        let r = o + horizontal;
        if !r.out_of_bounds(max) && map[r.x][r.y] - val == 1 && !visited.iter().any(|k| *k == r) {
            visited.push(r);
            open.push_back(r);
        }
        let l = o - horizontal;
        if !l.out_of_bounds(max) && map[l.x][l.y] - val == 1 && !visited.iter().any(|k| *k == l) {
            visited.push(l);
            open.push_back(l);
        }
    }
    count
}

fn part2(start: Coordinate<usize>, mapa: &Vec<Vec<u8>>) -> u32 {
    let max = mapa.len();
    let vertical = Coordinate::new(1, 0);
    let horizontal = Coordinate::new(0, 1);
    let mut open: VecDeque<Coordinate<usize>> = VecDeque::new();
    let mut count = 0;
    open.push_back(start);
    while let Some(o) = open.pop_front() {
        let val = mapa[o.x][o.y];
        if val == 9 {
            count += 1;
            continue;
        }
        let u = o - vertical;
        if !u.out_of_bounds(max) && mapa[u.x][u.y] - val == 1 {
            open.push_back(u);
        }
        let d = o + vertical;
        if !d.out_of_bounds(max) && mapa[d.x][d.y] - val == 1 {
            open.push_back(d);
        }
        let r = o + horizontal;
        if !r.out_of_bounds(max) && mapa[r.x][r.y] - val == 1 {
            open.push_back(r);
        }
        let l = o - horizontal;
        if !l.out_of_bounds(max) && mapa[l.x][l.y] - val == 1 {
            open.push_back(l);
        }
    }
    count
}

pub fn run(inp: Vec<String>) -> (String, String) {
    let mut head: Vec<Coordinate<usize>> = vec![];
    let map: Vec<Vec<u8>> = inp
        .iter()
        .enumerate()
        .map(|(i, f)| {
            f.chars()
                .enumerate()
                .map(|(j, c)| {
                    let n = c.to_digit(10).unwrap() as u8;
                    if n == 0 {
                        head.push(Coordinate::new(i, j));
                    }
                    n
                })
                .collect()
        })
        .collect();

    let p1: u32 = head.iter().map(|w| part1(*w, &map)).sum();
    let p2: u32 = head.iter().map(|w| part2(*w, &map)).sum();
    (format!("{}", p1), format!("{}", p2))
}
