pub fn part1(map: &Vec<Vec<i64>>, pos_: (i32, i32)) -> i64 {
    let mut visited = vec![vec![0u32; map[0].len()]; map.len()];
    let mut dir: (i32, i32) = (-1, 0);
    let mut pos = pos_;
    loop {
        visited[pos.0 as usize][pos.1 as usize] = 1;
        let mut new = pos.clone();
        new.0 += dir.0;
        new.1 += dir.1;
        if new.0 == map.len() as i32 || new.1 == map[0].len() as i32 || new.0 < 0 || new.1 < 0 {
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
        .map(|x| x.chars().map(|y| if y == '#' { 1 } else { 0 }).collect())
        .collect();
    let mut pos: (i32, i32) = (0, 0);
    for (i, x) in inp.iter().enumerate() {
        for (j, y) in x.chars().enumerate() {
            if y == '^' {
                pos = (i as i32, j as i32)
            }
        }
    }
    (part1(&map, pos), 0)
}
