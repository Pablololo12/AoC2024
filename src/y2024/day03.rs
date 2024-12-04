use regex::Regex;

pub fn part1(inp: &String) -> i64 {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    re.captures_iter(inp)
        .map(|c| {
            let x = c.get(1).unwrap().as_str().parse::<i64>().unwrap();
            let y = c.get(2).unwrap().as_str().parse::<i64>().unwrap();
            x * y
        })
        .sum()
}

pub fn part2(inp: &String) -> i64 {
    let after = Regex::new(r"don\'t\(\)(.*?)do\(\)")
        .unwrap()
        .replace_all(inp, "")
        .into_owned();
    part1(&after)
}

pub fn run(inp: Vec<String>) -> (i64, i64) {
    let jj = inp.join("");
    (part1(&jj), part2(&jj))
}
