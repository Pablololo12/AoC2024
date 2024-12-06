use rayon::iter::{IntoParallelIterator, ParallelIterator};

pub fn is_loop(map: &Vec<Vec<i64>>, pos_: (i32, i32)) -> bool {
    let mut visited = vec![vec![(0u32, (0i32, 0i32)); map[0].len()]; map.len()];
    let mut dir: (i32, i32) = (-1, 0);
    let mut pos = pos_;
    let mut limit = 1000000;
    loop {
        limit -= 1;
        if limit == 0 {
            println!("Limit reached");
            return true;
        }
        let (did, prev) = visited[pos.0 as usize][pos.1 as usize];
        if did == 1 && prev.0 == dir.0 && prev.1 == dir.1 {
            return true;
        }
        visited[pos.0 as usize][pos.1 as usize] = (1, dir);
        let mut new = pos.clone();
        new.0 += dir.0;
        new.1 += dir.1;
        if new.0 >= map.len() as i32 || new.1 >= map[0].len() as i32 || new.0 < 0 || new.1 < 0 {
            return false;
        } else if map[new.0 as usize][new.1 as usize] == 1 {
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
}

pub fn part2(mapp: Vec<Vec<i64>>, pos_: (i32, i32)) -> i64 {
    //for i in 0..map.len() {
    (0..mapp.len())
        .into_par_iter()
        .map(|i| {
            let mut map = mapp.clone();
            let mut acc = 0;
            for j in 0..map[0].len() {
                if map[i][j] == 0 {
                    map[i][j] = 1;
                    if is_loop(&map, pos_) {
                        acc += 1;
                    }
                    map[i][j] = 0;
                }
            }
            acc
        })
        .sum()
}

pub fn part1(map: &Vec<Vec<i64>>, pos_: (i32, i32)) -> i64 {
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
        } else if map[new.0 as usize][new.1 as usize] == 1 {
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
    let map: Vec<Vec<i64>> = inp
        .iter()
        .map(|x| {
            x.chars()
                .map(|y| {
                    if y == '#' {
                        1
                    } else if y == '^' {
                        2
                    } else {
                        0
                    }
                })
                .collect()
        })
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
