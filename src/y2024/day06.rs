use rayon::iter::{IntoParallelIterator, ParallelIterator};

// Dir
// 0b1000 up
// 0b0100 down
// 0b0010 left
// 0b0001 right
fn get_new(inn: (i32, i32), di: u8) -> (i32, i32) {
    let (x, y) = inn;
    match di {
        0b1000 => (x - 1, y),
        0b0100 => (x + 1, y),
        0b0010 => (x, y - 1),
        0b0001 => (x, y + 1),
        _ => (0, 0),
    }
}
fn is_loop(map: &Vec<Vec<bool>>, pos_: (i32, i32)) -> bool {
    let mut visited = vec![vec![(0u32, 0u8); map[0].len()]; map.len()];
    let mut dir: u8 = 0b1000;
    let mut pos = pos_;
    loop {
        let (did, prev) = visited[pos.0 as usize][pos.1 as usize];
        if did == 1 && (prev & dir) != 0 {
            return true;
        }
        visited[pos.0 as usize][pos.1 as usize] = (1, prev | dir);
        let new = get_new(pos, dir);
        if new.0 >= map.len() as i32 || new.1 >= map[0].len() as i32 || new.0 < 0 || new.1 < 0 {
            return false;
        } else if map[new.0 as usize][new.1 as usize] {
            if dir == 0b1000 {
                dir = 0b0001;
            } else if dir == 0b0100 {
                dir = 0b0010;
            } else if dir == 0b0010 {
                dir = 0b1000;
            } else if dir == 0b0001 {
                dir = 0b0100;
            }
        } else {
            pos = new.clone();
        }
    }
}

fn part2(mapp: Vec<Vec<bool>>, pos_: (i32, i32)) -> i64 {
    (0..mapp.len())
        .into_par_iter()
        .map(|i| {
            let mut map = mapp.clone();
            let mut acc = 0;
            for j in 0..map[0].len() {
                if !map[i][j] {
                    map[i][j] = true;
                    if is_loop(&map, pos_) {
                        acc += 1;
                    }
                    map[i][j] = false;
                }
            }
            acc
        })
        .sum()
}

pub fn part1(map: &Vec<Vec<bool>>, pos_: (i32, i32)) -> i64 {
    let mut visited = vec![vec![0u32; map[0].len()]; map.len()];
    let mut dir: (i32, i32) = (-1, 0);
    let mut pos = pos_;
    loop {
        visited[pos.0 as usize][pos.1 as usize] = 1;
        let mut new = pos.clone();
        new.0 += dir.0;
        new.1 += dir.1;
        if new.0 >= map.len() as i32 || new.1 >= map[0].len() as i32 || new.0 < 0 || new.1 < 0 {
            break;
        } else if map[new.0 as usize][new.1 as usize] {
            if dir.0 == -1 {
                dir.0 = 0;
                dir.1 = 1;
            } else if dir.1 == 1 {
                dir.0 = 1;
                dir.1 = 0;
            } else if dir.0 == 1 {
                dir.0 = 0;
                dir.1 = -1;
            } else if dir.1 == -1 {
                dir.0 = -1;
                dir.1 = 0;
            }
        } else {
            pos = new.clone();
        }
    }

    visited.iter().map(|h| h.iter().sum::<u32>()).sum::<u32>() as i64
}

pub fn run(inp: Vec<String>) -> (i64, i64) {
    let map: Vec<Vec<bool>> = inp
        .iter()
        .map(|x| x.chars().map(|y| if y == '#' { true } else { false }).collect())
        .collect();
    let mut pos: (i32, i32) = (0, 0);
    for (i, x) in inp.iter().enumerate() {
        for (j, y) in x.chars().enumerate() {
            if y == '^' {
                pos = (i as i32, j as i32)
            }
        }
    }
    (part1(&map, pos), part2(map, pos))
}
