pub fn get_vectors(inp: &Vec<String>) -> (Vec<i64>, Vec<i64>) {
    let d: Vec<(i64, i64)> = inp
        .into_iter()
        .map(|x| {
            let mut aux = x.split_whitespace();
            (
                aux.next().expect("msg").parse::<i64>().expect("msg"),
                aux.next().expect("msg").parse::<i64>().expect("msg"),
            )
        })
        .collect();
    let mut d1: Vec<i64> = d.iter().map(|(first, _)| *first).collect::<Vec<i64>>();
    d1.sort();
    let mut d2: Vec<i64> = d.iter().map(|(_, second)| *second).collect::<Vec<i64>>();
    d2.sort();
    (d1, d2)
}
pub fn part1(inp: &Vec<String>) -> i64 {
    let (d1, d2) = get_vectors(inp);
    let merged: Vec<(i64, i64)> = d1.into_iter().zip(d2.into_iter()).collect();
    let result: i64 = merged
        .into_iter()
        .fold(0, |acc, (f, ff)| acc + (f - ff).abs());
    result
}

pub fn part2(inp: &Vec<String>) -> i64 {
    let (d1, d2) = get_vectors(inp);
    let result = d1
        .into_iter()
        .map(|x| x * d2.iter().filter(|&y| *y == x).count() as i64)
        .fold(0i64, |acc, ff| acc + ff);
    result
}

pub fn run(inp: Vec<String>) -> (i64, i64) {
    println!("Hi! I'm Day01");

    (part1(&inp), part2(&inp))
}
