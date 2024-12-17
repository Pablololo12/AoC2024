use std::{
    collections::{BTreeSet, HashMap, HashSet},
    i64::MAX,
};

use aoc24::Coordinate;

const UP: Coordinate<i32> = Coordinate { x: -1, y: 0 };
const DOWN: Coordinate<i32> = Coordinate { x: 1, y: 0 };
const LEFT: Coordinate<i32> = Coordinate { x: 0, y: -1 };
const RIGHT: Coordinate<i32> = Coordinate { x: 0, y: 1 };

fn printt(mapa: &HashSet<Coordinate<i32>>) {
    let maxx = mapa.iter().map(|w| w.x).max().unwrap();
    let maxy = mapa.iter().map(|w| w.y).max().unwrap();

    for i in 0..=maxx {
        for j in 0..=maxy {
            if mapa.contains(&Coordinate { x: i, y: j }) {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!(" ");
    }
}

fn magic_rec(
    scores: &HashMap<Coordinate<i32>, HashMap<Coordinate<i32>, i64>>,
    pathh: Vec<Coordinate<i32>>,
    uniques: &mut HashSet<Coordinate<i32>>,
    pos: Coordinate<i32>,
    end: Coordinate<i32>,
    acc: i64,
    score: i64,
) {
    if pathh.contains(&pos) {
        return;
    }
    if acc > score {
        return;
    }
    if acc == score && pos == end {
        pathh.iter().for_each(|f| {
            uniques.insert(*f);
        });
    }
    let sc = scores.get(&pos).unwrap();
    [UP, LEFT, DOWN, RIGHT].iter().for_each(|&w| {
        if let Some(ss) = sc.get(&w) {
            if *ss == acc + 1 || *ss == acc + 1001 {
                let mut tmp = pathh.clone();
                tmp.push(pos);
                magic_rec(scores, tmp, uniques, pos + w, end, *ss, score);
            }
        }
    });
}

fn counter(d1: Coordinate<i32>, d2: Coordinate<i32>) -> bool {
    d2 == get_opposite(d1)
}

fn get_opposite(d1: Coordinate<i32>) -> Coordinate<i32> {
    match d1 {
        UP => DOWN,
        DOWN => UP,
        LEFT => RIGHT,
        RIGHT => LEFT,
        _ => Coordinate { x: 0, y: 0 },
    }
}

fn magic(mapa: &Vec<Coordinate<i32>>, end: Coordinate<i32>, pos: Coordinate<i32>, easter: bool) -> (i64, i64) {
    //let mut open: Vec<(Coordinate<i32>, Coordinate<i32>, i64)> = Vec::new();
    let mut open: BTreeSet<(i64, Coordinate<i32>, Coordinate<i32>)> = BTreeSet::new();
    let mut scores: HashMap<Coordinate<i32>, HashMap<Coordinate<i32>, i64>> = HashMap::new();
    mapa.iter().for_each(|w| {
        let ma: HashMap<Coordinate<i32>, i64> = HashMap::new();
        scores.insert(*w, ma);
    });
    open.insert((0, pos, RIGHT));
    let mut mini = MAX;
    while let Some((prev_s, pp, dir)) = open.pop_first() {
        if pp == end {
            if prev_s < mini {
                mini = prev_s;
            }
            continue;
        }
        [UP, LEFT, DOWN, RIGHT]
            .iter()
            .filter(|w| !counter(dir, **w))
            .map(|w| {
                (
                    pp + *w,
                    *w,
                    match *w == dir {
                        true => 1,
                        _ => 1001,
                    },
                )
            })
            .filter(|(tmp, _, _)| mapa.contains(&tmp))
            .for_each(|(tmp, w, sc)| {
                if let Some(s) = scores.get(&pp).unwrap().get(&get_opposite(w)) {
                    if *s < prev_s + sc {
                        return;
                    }
                }
                if let Some(s) = scores.get_mut(&pp).unwrap().get_mut(&w) {
                    if *s > prev_s + sc {
                        *s = prev_s + sc;
                        open.insert((prev_s, tmp, w));
                    }
                } else {
                    scores.get_mut(&pp).unwrap().insert(w, prev_s + sc);
                    open.insert((prev_s + sc, tmp, w));
                }
            });
    }
    let mut uniques: HashSet<Coordinate<i32>> = HashSet::new();
    magic_rec(&scores, vec![], &mut uniques, pos, end, 0, mini);
    if easter {
        printt(&uniques);
    }

    (mini, uniques.len() as i64 + 1)
}

pub fn run(inp: Vec<String>, easter: bool) -> (String, String) {
    let mut mapa: Vec<Coordinate<i32>> = vec![];
    let mut start: Coordinate<i32> = Coordinate { x: 0, y: 0 };
    let mut end: Coordinate<i32> = Coordinate { x: 0, y: 0 };
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
                _ => (),
            }
        });
    });
    let (p1, p2) = magic(&mapa, end, start, easter);
    (format!("{}", p1), format!("{}", p2))
}
