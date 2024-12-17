use itertools::Itertools;

pub fn madness(inp: &Vec<i64>) -> bool {
    let jumps: Vec<i64> = inp.iter().tuple_windows().map(|(y, z)| (y - z)).collect();
    let all_negative = !jumps.iter().any(|y| *y > 0);
    let all_positive = !jumps.iter().any(|y| *y < 0);
    let sol = !jumps.iter().any(|y| ((*y).abs() > 3 || *y == 0)) && (all_negative || all_positive);
    sol
}

pub fn part1(inp: &Vec<String>) -> i64 {
    inp.into_iter()
        .filter(|x| {
            let arr: Vec<i64> = x.split_whitespace().map(|y| y.parse::<i64>().unwrap()).collect();
            madness(&arr)
        })
        .count() as i64
}

pub fn part2(inp: &Vec<String>) -> i64 {
    inp.into_iter()
        .filter(|x| {
            let arr: Vec<i64> = x.split_whitespace().map(|y| y.parse::<i64>().unwrap()).collect();
            for var in 0..arr.len() {
                let mut aux = arr.clone();
                aux.remove(var);
                if madness(&aux) {
                    return true;
                }
            }
            return false;
        })
        .count() as i64
}

pub fn run(inp: Vec<String>) -> (String, String) {
    (format!("{}", part1(&inp)), format!("{}", part2(&inp)))
}
