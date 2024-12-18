use aoc24::Coordinate;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use std::collections::{BTreeSet, HashMap};

const SIZE: i64 = 70;
//const SIZE: i64 = 6;
const SIM: usize = 1024;
//const SIM: usize = 12;
const UP: Coordinate<i64> = Coordinate { x: -1, y: 0 };
const DOWN: Coordinate<i64> = Coordinate { x: 1, y: 0 };
const LEFT: Coordinate<i64> = Coordinate { x: 0, y: -1 };
const RIGHT: Coordinate<i64> = Coordinate { x: 0, y: 1 };

fn printt(mapa: &Vec<Coordinate<i64>>, path: &Vec<Coordinate<i64>>) {
    for i in 0..=SIZE {
        for j in 0..=SIZE {
            if mapa.contains(&Coordinate { x: i, y: j }) {
                print!("#");
            } else if path.contains(&Coordinate { x: i, y: j }) {
                print!("O");
            } else {
                print!(" ");
            }
        }
        println!(" ");
    }
}
fn astar(obstacles: &Vec<Coordinate<i64>>, full: bool, easter: bool) -> i64 {
    let mut open: BTreeSet<(i64, Coordinate<i64>)> = BTreeSet::new();
    let mut came_from: HashMap<Coordinate<i64>, Coordinate<i64>> = HashMap::new();
    let mut score: HashMap<Coordinate<i64>, i64> = HashMap::new();
    let start = Coordinate { x: 0, y: 0 };
    let end = Coordinate { x: SIZE, y: SIZE };
    open.insert((0, Coordinate { x: 0, y: 0 }));
    score.insert(Coordinate { x: 0, y: 0 }, 0);

    while let Some((_, pos)) = open.pop_first() {
        if pos == end {
            break;
        }
        [UP, LEFT, DOWN, RIGHT]
            .iter()
            .map(|w| *w + pos)
            .filter(|w| !obstacles.contains(w) && !w.out_of_bounds(SIZE + 1))
            .for_each(|w| {
                let tentsc = score.get(&pos).unwrap() + 1;
                if let Some(scc) = score.get(&w) {
                    if tentsc < *scc {
                        *came_from.get_mut(&w).unwrap() = pos;
                        *score.get_mut(&w).unwrap() = tentsc;
                        open.insert((tentsc, w));
                    }
                } else {
                    came_from.insert(w, pos);
                    score.insert(w, tentsc);
                    open.insert((tentsc, w));
                }
            });
    }

    if !came_from.contains_key(&end) {
        return -1;
    }
    if !full {
        return 1;
    }
    let mut mov = end;
    let mut path: Vec<Coordinate<i64>> = vec![];
    loop {
        path.push(mov);
        mov = *came_from.get(&mov).unwrap();
        if mov == start {
            break;
        }
    }
    if easter {
        printt(obstacles, &path);
    }
    path.len() as i64
}

pub fn run(inp: Vec<String>, easter: bool) -> (String, String) {
    let list: Vec<Coordinate<i64>> = inp
        .iter()
        .filter(|l| l.contains(','))
        .map(|l| {
            let mut aux = l.split(',');
            Coordinate::<i64> {
                y: aux.next().unwrap().parse::<i64>().unwrap(),
                x: aux.next().unwrap().parse::<i64>().unwrap(),
            }
        })
        .collect();
    let inp: Vec<Coordinate<i64>> = list.iter().take(SIM).map(|w| *w).collect();
    let p1 = astar(&inp, true, easter);

    let first = (SIM..list.len())
        .into_par_iter()
        .find_first(|i| {
            let inp: Vec<Coordinate<i64>> = list.iter().take(*i).map(|w| *w).collect();
            let p1 = astar(&inp, false, false);
            p1 == -1
        })
        .unwrap();
    let co = list.get(first - 1).unwrap();
    (format!("{}", p1), format!("{},{}", co.y, co.x))
}
