use itertools::Itertools;
use regex::Regex;

fn get_value(ra: i64, rb: i64, rc: i64, op: u8) -> i64 {
    match op {
        4 => ra,
        5 => rb,
        6 => rc,
        _ => op as i64,
    }
}

fn sim(mut ra: i64, mut rb: i64, mut rc: i64, inst: &Vec<u8>) -> Vec<u8> {
    let mut pc: i64 = 0;
    let mut output: Vec<u8> = vec![];
    while let Some(ins) = inst.get(pc as usize) {
        let op = inst.get(pc as usize + 1).unwrap();
        let val = get_value(ra, rb, rc, *op);
        match ins {
            0 => ra = ra / 2i64.pow(val as u32),
            1 => rb = rb ^ *op as i64,
            2 => rb = val % 8,
            3 => {
                if ra != 0 {
                    pc = *op as i64 - 2;
                }
            }
            4 => rb = rb ^ rc,
            5 => output.push((val % 8) as u8),
            6 => rb = ra / 2i64.pow(val as u32),
            7 => rc = ra / 2i64.pow(val as u32),
            _ => todo!(),
        }
        pc += 2;
    }
    output
}

pub fn run(inp: Vec<String>) -> (String, String) {
    let re = Regex::new(r"Register [A|B|C]: ([0-9]+)").unwrap();
    let mut ra = 0;
    let mut rb = 0;
    let mut rc = 0;
    let mut inst: Vec<u8> = vec![];
    inp.iter().for_each(|w| {
        if w.contains('A') {
            let c = re.captures(&w).unwrap();
            ra = c.get(1).unwrap().as_str().parse::<i64>().unwrap();
        }
        if w.contains('B') {
            let c = re.captures(&w).unwrap();
            rb = c.get(1).unwrap().as_str().parse::<i64>().unwrap();
        }
        if w.contains('C') {
            let c = re.captures(&w).unwrap();
            rc = c.get(1).unwrap().as_str().parse::<i64>().unwrap();
        }
        if w.contains(',') {
            w.split_whitespace().for_each(|t| {
                if t.contains(',') {
                    t.split(',').for_each(|n| inst.push(n.parse::<u8>().unwrap()));
                }
            });
        }
    });
    let output = sim(ra, rb, rc, &inst);

    let mut omg = 0;
    for i in 1..=inst.len() {
        loop {
            let out = sim(omg, rb, rc, &inst);
            let matching = out
                .iter()
                .rev()
                .take(i)
                .zip(inst.iter().rev().take(i))
                .filter(|&(a, b)| a == b)
                .count();
            if matching == i {
                omg = omg * 8;
                break;
            }
            omg += 1;
        }
    }
    (format!("{}", output.iter().format(",")), format!("{}", omg / 8))
}
