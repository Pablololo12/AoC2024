use regex::Regex;

pub fn part1(inp: &Vec<String>) -> i64 {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let mut acc = 0;
    for i in inp.iter() {
        for (_, [num1, num2]) in re.captures_iter(i).map(|c| c.extract()) {
            let n1 = num1.parse::<u64>().unwrap();
            let n2 = num2.parse::<u64>().unwrap();
            acc += n1 * n2;
        }
    }
    acc as i64
}

pub fn part2(inp: &Vec<String>) -> i64 {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)|d(o)(n)\'t\(\)|(d)(o)\(\)").unwrap();
    let mut acc = 0;
    let mut enable = true;
    for i in inp.iter() {
        for (_, [num1, num2]) in re.captures_iter(i).map(|c| c.extract()) {
            if num1.parse::<u64>().is_ok() && enable {
                let n1 = num1.parse::<u64>().unwrap();
                let n2 = num2.parse::<u64>().unwrap();
                acc += n1 * n2;
            } else if num2 == "n" {
                enable = false;
            } else if num2 == "o" {
                enable = true;
            }
        }
    }
    acc as i64
}

pub fn run(inp: Vec<String>) -> (i64, i64) {
    (part1(&inp), part2(&inp))
}
