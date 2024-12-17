pub fn get_vectors(inp: &Vec<String>) -> (Vec<i64>, Vec<i64>) {
    let (mut d1, mut d2): (Vec<i64>, Vec<i64>) = inp
        .into_iter()
        .map(|x| {
            let mut aux = x.split_whitespace();
            (
                aux.next().unwrap().parse::<i64>().unwrap(),
                aux.next().unwrap().parse::<i64>().unwrap(),
            )
        })
        .unzip();
    d1.sort();
    d2.sort();
    (d1, d2)
}
pub fn part1(d1: &Vec<i64>, d2: &Vec<i64>) -> i64 {
    d1.into_iter()
        .zip(d2.into_iter())
        .fold(0, |acc, (f, ff)| acc + (f - ff).abs())
}

pub fn part2(d1: &Vec<i64>, d2: &Vec<i64>) -> i64 {
    d1.into_iter()
        .map(|x| x * d2.iter().filter(|&y| *y == *x).count() as i64)
        .sum()
}

pub fn run(inp: Vec<String>) -> (String, String) {
    let (d1, d2) = get_vectors(&inp);
    (format!("{}", part1(&d1, &d2)), format!("{}", part2(&d1, &d2)))
}
