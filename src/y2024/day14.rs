use regex::Regex;

const MAXX: i64 = 101;
const MAXY: i64 = 103;
const STEPS: i64 = 100;

fn get_in(inp: &Vec<String>) -> Vec<(i64, i64, i64, i64)> {
    let re = Regex::new(r"p=([0-9]+),([0-9]+) v=(-?[0-9]+),(-?[0-9]+)").unwrap();
    inp.iter()
        .map(|w| {
            let c = re.captures(w).unwrap();
            let vx = match c.get(3).unwrap().as_str().parse::<i64>().unwrap() {
                a if a < 0 => MAXX + a,
                a => a,
            };
            let vy = match c.get(4).unwrap().as_str().parse::<i64>().unwrap() {
                a if a < 0 => MAXY + a,
                a => a,
            };
            (
                c.get(1).unwrap().as_str().parse::<i64>().unwrap(),
                c.get(2).unwrap().as_str().parse::<i64>().unwrap(),
                vx,
                vy,
            )
        })
        .collect()
}

fn step(inp: &Vec<(i64, i64, i64, i64)>, steps: i64) -> Vec<(i64, i64)> {
    inp.iter()
        .map(|(x, y, vx, vy)| {
            let fx = (x + vx * steps) % MAXX;
            let fy = (y + vy * steps) % MAXY;
            (fx, fy)
        })
        .collect()
}

fn print_pretty(inp: &Vec<(i64, i64, i64, i64)>, printt: bool) -> i64 {
    let mut steps = 0;
    let mut mapa;
    loop {
        steps += 1;
        mapa = step(&inp, steps);
        let len_b = mapa.len();
        mapa.sort();
        mapa.dedup();
        if len_b != mapa.len() {
            continue;
        }
        break;
    }
    if printt {
        for i in 0..MAXY {
            for j in 0..MAXX {
                if mapa.contains(&(j as i64, i as i64)) {
                    print!("#");
                } else {
                    print!(" ");
                }
            }
            println!(" ");
        }
    }
    steps
}

pub fn run(inp: Vec<String>, easter: bool) -> (String, String) {
    let i = get_in(&inp);
    let mapa = step(&i, STEPS);
    let q1 = mapa.iter().filter(|(x, y)| *x < MAXX / 2 && *y < MAXY / 2).count() as i64;
    let q2 = mapa.iter().filter(|(x, y)| *x > MAXX / 2 && *y < MAXY / 2).count() as i64;
    let q3 = mapa.iter().filter(|(x, y)| *x < MAXX / 2 && *y > MAXY / 2).count() as i64;
    let q4 = mapa.iter().filter(|(x, y)| *x > MAXX / 2 && *y > MAXY / 2).count() as i64;

    let p2 = print_pretty(&i, easter);

    (format!("{}", q1 * q2 * q3 * q4), format!("{}", p2))
}
