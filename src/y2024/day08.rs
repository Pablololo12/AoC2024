use std::collections::HashMap;

fn check_antinodes(inn: &[(i32, i32)], max: i32) -> Vec<(i32, i32)> {
    if inn.len() == 1 {
        return vec![];
    }
    let (hx, hy) = inn[0];
    let mut nodes: Vec<(i32, i32)> = vec![];

    inn[1..].iter().for_each(|(jx, jy)| {
        let (dx, dy) = (jx - hx, jy - hy);
        let (fx, fy) = (hx - dx, hy - dy);
        let (sx, sy) = (jx + dx, jy + dy);
        if fx >= 0 && fx < max && fy >= 0 && fy < max {
            nodes.push((fx, fy));
        }
        if sx >= 0 && sx < max && sy >= 0 && sy < max {
            nodes.push((sx, sy));
        }
    });

    nodes.extend(check_antinodes(&inn[1..], max));
    nodes
}

fn check_antinodes_extended(inn: &[(i32, i32)], max: i32) -> Vec<(i32, i32)> {
    if inn.len() == 1 {
        return vec![];
    }
    let (hx, hy) = inn[0];
    let mut nodes: Vec<(i32, i32)> = vec![];

    inn[1..].iter().for_each(|(jx, jy)| {
        let (dx, dy) = (jx - hx, jy - hy);
        let (mut fx, mut fy) = (hx - dx, hy - dy);
        let (mut sx, mut sy) = (jx + dx, jy + dy);
        loop {
            if fx >= 0 && fx < max && fy >= 0 && fy < max {
                nodes.push((fx, fy));
            } else {
                break;
            }
            fx -= dx;
            fy -= dy;
        }
        loop {
            if sx >= 0 && sx < max && sy >= 0 && sy < max {
                nodes.push((sx, sy));
            } else {
                break;
            }
            sx += dx;
            sy += dy;
        }
    });

    nodes.extend(check_antinodes_extended(&inn[1..], max));
    nodes
}
pub fn run(inp: Vec<String>) -> (i64, i64) {
    let mut antena: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut nodes2: Vec<(i32, i32)> = vec![];
    inp.iter().enumerate().for_each(|(i, l)| {
        l.chars().enumerate().for_each(|(j, c)| {
            if c != '.' {
                antena.entry(c).or_insert(Vec::new()).push((i as i32, j as i32));
                nodes2.push((i as i32, j as i32));
            }
        });
    });
    let mut nodes: Vec<(i32, i32)> = vec![];

    antena
        .iter()
        .for_each(|(_, v)| nodes.extend(check_antinodes(v, inp.len() as i32)));
    nodes.sort();
    nodes.dedup();

    antena
        .iter()
        .for_each(|(_, v)| nodes2.extend(check_antinodes_extended(v, inp.len() as i32)));
    nodes2.sort();
    nodes2.dedup();
    (nodes.len() as i64, nodes2.len() as i64)
}
