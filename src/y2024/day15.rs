use aoc24::Coordinate;
use std::{thread, time};

fn pretty_print(start: Coordinate<i32>, obstacles: &Vec<Coordinate<i32>>, boxes: &Vec<Coordinate<i32>>, m: u8) {
    let d = match m {
        0 => '^',
        1 => '>',
        2 => 'v',
        3 => '<',
        _ => '*',
    };
    println!("Move {d}");
    let maxx = obstacles.iter().map(|w| w.x).max().unwrap();
    let maxy = obstacles.iter().map(|w| w.y).max().unwrap();
    for i in 0..=maxx {
        for j in 0..=maxy {
            let co = Coordinate { x: i, y: j };
            if boxes.contains(&co) {
                print!("O");
            } else if obstacles.contains(&co) {
                print!("#");
            } else if co == start {
                print!("@");
            } else {
                print!(" ");
            }
        }
        println!(" ");
    }
}

fn pretty_print2(start: Coordinate<i32>, obstacles: &Vec<Coordinate<i32>>, boxes: &Vec<Coordinate<i32>>, m: u8) {
    let d = match m {
        0 => '^',
        1 => '>',
        2 => 'v',
        3 => '<',
        _ => '*',
    };
    let check = Coordinate { x: 0, y: 1 };
    println!("Move {d}");
    let maxx = obstacles.iter().map(|w| w.x).max().unwrap();
    let maxy = obstacles.iter().map(|w| w.y).max().unwrap();
    for i in 0..=maxx {
        for j in 0..=maxy {
            let co = Coordinate { x: i, y: j };
            if boxes.contains(&co) {
                print!("[");
            } else if obstacles.contains(&co) {
                print!("#");
            } else if co == start {
                print!("@")
            } else if boxes.contains(&(co - check)) {
                print!("]");
            } else {
                print!(".");
            }
        }
        println!(" ");
    }
}

fn crazy_print(
    start: Coordinate<i32>,
    obstacles: &Vec<Coordinate<i32>>,
    boxes: &Vec<Coordinate<i32>>,
    m: u8,
    steps: u32,
) {
    print!("\x1B[2J\x1B[1;1H");
    println!("Step {steps}");
    pretty_print2(start, obstacles, boxes, m);
}
fn movh(
    start: &mut Coordinate<i32>,
    dir: Coordinate<i32>,
    boxes: &mut Vec<Coordinate<i32>>,
    obstacles: &Vec<Coordinate<i32>>,
) {
    let mut new_p = *start + dir;
    let check = Coordinate { x: 0, y: 1 };
    if !boxes.contains(&new_p) && !boxes.contains(&(new_p - check)) && !obstacles.contains(&new_p) {
        *start = new_p;
        return;
    }
    let mut inxx: Vec<usize> = vec![];
    loop {
        let bl = boxes.contains(&new_p);
        let br = boxes.contains(&(new_p - check));
        if !bl && !br && !obstacles.contains(&new_p) {
            break;
        }
        if let Some(inx) = boxes.iter().position(|&r| r == new_p) {
            inxx.push(inx);
        }
        if let Some(inx) = boxes.iter().position(|&r| r == new_p - check) {
            inxx.push(inx);
        }
        if obstacles.contains(&new_p) {
            return;
        }
        new_p = new_p + dir;
    }
    inxx.dedup();
    inxx.iter().for_each(|&w| {
        boxes[w] = boxes[w] + dir;
    });
    *start = *start + dir;
}

fn recur(
    start: Coordinate<i32>,
    dir: Coordinate<i32>,
    obstacles: &Vec<Coordinate<i32>>,
    boxes: &Vec<Coordinate<i32>>,
    co: &mut Vec<usize>,
) -> bool {
    let check = Coordinate { x: 0, y: 1 };
    let left = start + dir;
    let right = start + check + dir;
    let ol = obstacles.contains(&left);
    let or = obstacles.contains(&right);
    if ol || or {
        return false;
    }

    if let Some(inx) = boxes.iter().position(|&r| r == start) {
        co.push(inx);
    }

    let bl = boxes.contains(&left);
    let bll = boxes.contains(&(left - check));
    let br = boxes.contains(&right);

    let mut gl = true;
    let mut gll = true;
    let mut gr = true;
    if bl {
        gl = recur(left, dir, obstacles, boxes, co);
    }

    if br {
        gr = recur(right, dir, obstacles, boxes, co);
    }

    if bll {
        gll = recur(left - check, dir, obstacles, boxes, co);
    }

    gl && gr && gll
}

fn movv(
    start: &mut Coordinate<i32>,
    dir: Coordinate<i32>,
    boxes: &mut Vec<Coordinate<i32>>,
    obstacles: &Vec<Coordinate<i32>>,
) {
    let new_p = *start + dir;
    let check = Coordinate { x: 0, y: -1 };
    if obstacles.contains(&new_p) {
        return;
    }
    let bl = boxes.contains(&new_p);
    let br = boxes.contains(&(new_p + check));
    if !bl && !br {
        *start = new_p;
        return;
    }
    let mut inxx: Vec<usize> = vec![];
    let good: bool;
    if br {
        good = recur(new_p + check, dir, obstacles, boxes, &mut inxx);
    } else {
        good = recur(new_p, dir, obstacles, boxes, &mut inxx);
    }
    inxx.sort();
    inxx.dedup();
    if good {
        inxx.iter().for_each(|&w| {
            boxes[w] = boxes[w] + dir;
        });
        *start = *start + dir;
    }
}

fn part2(
    mut start: Coordinate<i32>,
    obstacles: &Vec<Coordinate<i32>>,
    boxes: &mut Vec<Coordinate<i32>>,
    moves: &Vec<u8>,
    easter: bool,
) -> i64 {
    let mut ii = 0;
    let mil = time::Duration::from_millis(200);
    moves.iter().for_each(|m| {
        match m {
            0 => {
                movv(&mut start, Coordinate { x: -1, y: 0 }, boxes, obstacles);
            }
            1 => {
                movh(&mut start, Coordinate { x: 0, y: 1 }, boxes, obstacles);
            }
            2 => {
                movv(&mut start, Coordinate { x: 1, y: 0 }, boxes, obstacles);
            }
            3 => {
                movh(&mut start, Coordinate { x: 0, y: -1 }, boxes, obstacles);
            }
            _ => {
                println!("Problems again");
                ()
            }
        };
        if easter && ii > 8000 {
            crazy_print(start, obstacles, boxes, *m, ii);
            thread::sleep(mil);
        }
        ii += 1;
    });
    boxes.iter().map(|&w| w.x as i64 * 100 + w.y as i64).sum::<i64>() as i64
}

fn mov(
    start: &mut Coordinate<i32>,
    dir: Coordinate<i32>,
    boxes: &mut Vec<Coordinate<i32>>,
    obstacles: &Vec<Coordinate<i32>>,
) {
    let mut new_p = *start + dir;
    if !boxes.contains(&new_p) && !obstacles.contains(&new_p) {
        *start = new_p;
        return;
    }
    loop {
        if !boxes.contains(&new_p) && !obstacles.contains(&new_p) {
            break;
        }
        if obstacles.contains(&new_p) {
            return;
        }
        new_p = new_p + dir;
    }
    let mut it = *start + dir;
    while it != new_p {
        if let Some(inx) = boxes.iter().position(|&r| r == it) {
            boxes[inx] = it + dir;
        }
        it = it + dir;
    }
    *start = *start + dir;
}

fn part1(
    mut start: Coordinate<i32>,
    obstacles: &Vec<Coordinate<i32>>,
    boxes: &mut Vec<Coordinate<i32>>,
    moves: &Vec<u8>,
) -> i64 {
    moves.iter().for_each(|m| match m {
        0 => {
            mov(&mut start, Coordinate { x: -1, y: 0 }, boxes, obstacles);
        }
        1 => {
            mov(&mut start, Coordinate { x: 0, y: 1 }, boxes, obstacles);
        }
        2 => {
            mov(&mut start, Coordinate { x: 1, y: 0 }, boxes, obstacles);
        }
        3 => {
            mov(&mut start, Coordinate { x: 0, y: -1 }, boxes, obstacles);
        }
        _ => {
            println!("Problems again");
            ()
        }
    });
    boxes.iter().map(|&w| w.x * 100 + w.y).sum::<i32>() as i64
}

pub fn run(inp: Vec<String>, easter: bool) -> (i64, i64) {
    let mut obstacles: Vec<Coordinate<i32>> = vec![];
    let mut boxes: Vec<Coordinate<i32>> = vec![];
    let mut robot: Coordinate<i32> = Coordinate { x: 0, y: 0 };
    inp.iter().filter(|w| w.contains('#')).enumerate().for_each(|(i, w)| {
        w.char_indices().for_each(|(j, q)| match q {
            '#' => obstacles.push(Coordinate {
                x: i as i32,
                y: j as i32,
            }),
            'O' => boxes.push(Coordinate {
                x: i as i32,
                y: j as i32,
            }),
            '@' => {
                robot = Coordinate {
                    x: i as i32,
                    y: j as i32,
                }
            }
            _ => (),
        })
    });
    let moves: Vec<u8> = inp
        .iter()
        .filter(|q| q.contains('<') || q.contains('>'))
        .flat_map(|w| {
            w.chars()
                .map(|q| match q {
                    '^' => 0,
                    '>' => 1,
                    'v' => 2,
                    '<' => 3,
                    _ => {
                        println!("Problem! {q}");
                        4
                    }
                })
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<u8>>();

    let mut obstacles2: Vec<Coordinate<i32>> = vec![];
    let mut boxes2: Vec<Coordinate<i32>> = vec![];
    let mut robot2: Coordinate<i32> = Coordinate { x: 0, y: 0 };
    inp.iter().filter(|w| w.contains('#')).enumerate().for_each(|(i, w)| {
        w.char_indices().for_each(|(j, q)| match q {
            '#' => {
                obstacles2.push(Coordinate {
                    x: i as i32,
                    y: j as i32 * 2,
                });
                obstacles2.push(Coordinate {
                    x: i as i32,
                    y: (j as i32 * 2) + 1,
                });
            }
            'O' => boxes2.push(Coordinate {
                x: i as i32,
                y: j as i32 * 2,
            }),
            '@' => {
                robot2 = Coordinate {
                    x: i as i32,
                    y: j as i32 * 2,
                }
            }
            _ => (),
        })
    });

    let p1 = part1(robot, &obstacles, &mut boxes, &moves);
    let p2 = part2(robot2, &obstacles2, &mut boxes2, &moves, easter);
    if easter {
        pretty_print(robot, &obstacles, &boxes, 4);
        pretty_print2(robot2, &obstacles2, &boxes2, 4);
    }
    (p1, p2)
}
