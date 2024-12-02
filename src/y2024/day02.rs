use aoc24::PairTerror;

pub fn madness(inp: std::slice::Iter<i64>) -> bool {
    let cont = PairTerror::new(inp);
    let jumps = cont.map(|(y, z)| (y - z));
    let all_negative = !jumps.clone().any(|y| y > 0);
    let all_positive = !jumps.clone().any(|y| y < 0);
    let sol = !jumps.clone().any(|y| ((y).abs() > 3 || y == 0)) && (all_negative || all_positive);
    sol
}

pub fn part1(inp: &Vec<String>) -> i64 {
    inp.into_iter()
        .filter(|x| {
            let arr: Vec<i64> = x.split_whitespace().map(|y| y.parse::<i64>().unwrap()).collect();
            madness(arr.iter())
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
                if madness(aux.iter()) {
                    return true;
                }
            }
            return false;
        })
        .count() as i64
}

pub fn run(inp: Vec<String>) -> (i64, i64) {
    (part1(&inp), part2(&inp))
}
