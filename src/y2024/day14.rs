use std::io::{self, Read};

use regex::Regex;

const MAXX: i64 = 101;
const MIDDLEX: i64 = 50;
const MAXY: i64 = 103;
const MIDDLEY: i64 = 51;
const STEPS: i64 = 100;

fn step(inp: &Vec<String>, steps: i64) -> Vec<(i64, i64)> {
    let re = Regex::new(r"p=([0-9]+),([0-9]+) v=(-?[0-9]+),(-?[0-9]+)").unwrap();
    inp.iter()
        .map(|w| {
            let c = re.captures(w).unwrap();
            let x = c.get(1).unwrap().as_str().parse::<i64>().unwrap();
            let y = c.get(2).unwrap().as_str().parse::<i64>().unwrap();
            let vx = c.get(3).unwrap().as_str().parse::<i64>().unwrap();
            let vy = c.get(4).unwrap().as_str().parse::<i64>().unwrap();
            let fx = match (x + vx * steps) % MAXX {
                a if a < 0 => MAXX + a,
                a => a,
            };
            let fy = match (y + vy * steps) % MAXY {
                a if a < 0 => MAXY + a,
                a => a,
            };
            (fx, fy)
        })
        .collect()
}

#[allow(dead_code)]
fn print_pretty(inp: &Vec<String>) {
    let mut buffer = [0; 1];
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut steps = 1495; // I was going 1 step at a time until I saw the pattern
    loop {
        println!("Steps {steps}");
        handle.read_exact(&mut buffer).unwrap();
        if buffer[0] == 27 {
            //esc key
            break;
        }
        let mapa = step(&inp, steps);
        for i in 0..MAXY {
            for j in 0..MAXX {
                if mapa.contains(&(j as i64, i as i64)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!(" ");
        }
        steps += 101;
    }
}

pub fn run(inp: Vec<String>) -> (i64, i64) {
    let mapa = step(&inp, STEPS);
    let q1 = mapa.iter().filter(|(x, y)| *x < MIDDLEX && *y < MIDDLEY).count() as i64;
    let q2 = mapa.iter().filter(|(x, y)| *x > MIDDLEX && *y < MIDDLEY).count() as i64;
    let q3 = mapa.iter().filter(|(x, y)| *x < MIDDLEX && *y > MIDDLEY).count() as i64;
    let q4 = mapa.iter().filter(|(x, y)| *x > MIDDLEX && *y > MIDDLEY).count() as i64;

    // Uncomment to see the easter egg
    //print_pretty(&inp);

    (q1 * q2 * q3 * q4, 7858)
}
