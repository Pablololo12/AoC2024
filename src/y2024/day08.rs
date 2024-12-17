use aoc24::Coordinate;
use std::collections::{HashMap, HashSet};

fn check_antinodes_extended(inn: &[Coordinate<i32>], out: &mut HashSet<Coordinate<i32>>, max: i32, limit: bool) {
    if inn.len() == 1 {
        return;
    }
    let head = inn[0];
    inn[1..].iter().for_each(|&other| {
        let diff = other - head;
        let mut first = head - diff;
        while !first.out_of_bounds(max as i32) {
            out.insert(first);
            first = first - diff;
            if limit {
                break;
            }
        }
        let mut second = other + diff;
        while !second.out_of_bounds(max as i32) {
            out.insert(second);
            second = second + diff;
            if limit {
                break;
            }
        }
    });
    check_antinodes_extended(&inn[1..], out, max, limit);
}
pub fn run(inp: Vec<String>) -> (String, String) {
    let mut antena: HashMap<char, Vec<Coordinate<i32>>> = HashMap::new();
    let mut nodes: HashSet<Coordinate<i32>> = HashSet::new();
    let mut nodes2: HashSet<Coordinate<i32>> = HashSet::new();

    inp.iter().enumerate().for_each(|(i, l)| {
        l.chars().enumerate().for_each(|(j, c)| {
            if c != '.' {
                antena
                    .entry(c)
                    .or_insert(Vec::new())
                    .push(Coordinate::new(i as i32, j as i32));
                nodes2.insert(Coordinate::new(i as i32, j as i32));
            }
        });
    });

    antena
        .iter()
        .for_each(|(_, v)| check_antinodes_extended(v, &mut nodes, inp.len() as i32, true));

    antena
        .iter()
        .for_each(|(_, v)| check_antinodes_extended(v, &mut nodes2, inp.len() as i32, false));
    (format!("{}", nodes.len()), format!("{}", nodes2.len()))
}
