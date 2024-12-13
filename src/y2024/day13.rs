use itertools::izip;
use regex::Regex;

fn get_in(inp: &Vec<String>, mt: &str) -> Vec<(i64, i64)> {
    let re = Regex::new(r"([0-9]+)[^0-9]*([0-9]+)").unwrap();
    inp.iter()
        .filter(|w| w.contains(&mt))
        .map(|w| re.captures(w).unwrap())
        .map(|w| {
            (
                w.get(1).unwrap().as_str().parse::<i64>().unwrap(),
                w.get(2).unwrap().as_str().parse::<i64>().unwrap(),
            )
        })
        .collect()
}

fn solver(ax: i64, ay: i64, bx: i64, by: i64, x: i64, y: i64) -> i64 {
    let det = ax * by - bx * ay;
    let iax = by;
    let ibx = -bx;
    let iay = -ay;
    let iby = ax;
    let a = (iax * x + ibx * y) / det;
    let b = (iay * x + iby * y) / det;
    let res1 = a * ax + b * bx;
    let res2 = a * ay + b * by;
    if res1 == x && res2 == y {
        return a * 3 + b;
    }
    0
}

pub fn run(inp: Vec<String>) -> (i64, i64) {
    izip!(get_in(&inp, "A:"), get_in(&inp, "B:"), get_in(&inp, "="))
        .map(|((ax, ay), (bx, by), (x, y))| {
            (
                solver(ax, ay, bx, by, x, y),
                solver(ax, ay, bx, by, x + 10000000000000, y + 10000000000000),
            )
        })
        .fold((0, 0), |(a, aa), (i, j)| (a + i, aa + j))
}
