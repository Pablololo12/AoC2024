pub fn madness(inp: &Vec<i64>) -> bool {
    let mut arr2 = inp.clone();
    arr2.rotate_right(1);
    let mut jumps: Vec<i64> = inp.into_iter().zip(arr2.into_iter()).map(|(y, z)| (y - z)).collect();
    jumps.remove(0);
    let all_negative = jumps.iter().filter(|y| **y > 0).count() == 0;
    let all_positive = jumps.iter().filter(|y| **y < 0).count() == 0;
    let sol = jumps.iter().filter(|y| (y.abs() > 3 || **y == 0)).count() == 0 && (all_negative || all_positive);
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
            if madness(&arr) {
                return true;
            }
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

pub fn run(inp: Vec<String>) -> (i64, i64) {
    (part1(&inp), part2(&inp))
}