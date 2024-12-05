use std::cmp::Ordering;

pub fn parse_pages(inp: &Vec<String>) -> (Vec<(i64, i64)>, Vec<Vec<i64>>) {
    let mut out: Vec<Vec<i64>> = vec![];
    let mut output: Vec<(i64, i64)> = vec![];
    inp.iter().for_each(|f| {
        if f.contains(",") {
            out.push(f.split(",").map(|jj| jj.parse::<i64>().unwrap()).collect());
        }
        if f.contains("|") {
            let mut kk = f.split("|");
            output.push((
                kk.next().unwrap().parse::<i64>().unwrap(),
                kk.next().unwrap().parse::<i64>().unwrap(),
            ));
        }
    });
    (output, out)
}

pub fn check(inp: &Vec<(i64, i64)>, val1: i64, val2: i64) -> bool {
    inp.iter().any(|(i, j)| *i == val2 && *j == val1)
}

pub fn check_or(inp: &Vec<(i64, i64)>, val1: i64, val2: i64) -> Ordering {
    if inp.iter().any(|(i, j)| *i == val2 && *j == val1) {
        return Ordering::Greater;
    } else {
        return Ordering::Less;
    }
}

pub fn part1(inp: &Vec<String>) -> i64 {
    let (order, pages) = parse_pages(inp);
    let mut acc = 0;
    for v in pages.iter() {
        let mut before = false;
        let mut after = false;
        for (i, jj) in v.iter().enumerate() {
            before |= v.get(0..i).unwrap().iter().any(|w| check(&order, *w, *jj));
            after |= v.get((i + 1)..).unwrap().iter().any(|w| check(&order, *jj, *w));
        }
        if !before && !after {
            acc += v.get(v.len() / 2).unwrap();
            println!("{:?}", v);
        }
    }
    acc
}

pub fn part2(inp: &Vec<String>) -> i64 {
    let (order, pages) = parse_pages(inp);
    let mut acc = 0;
    for v in pages.iter() {
        let mut before = false;
        let mut after = false;
        for (i, jj) in v.iter().enumerate() {
            before |= v.get(0..i).unwrap().iter().any(|w| check(&order, *w, *jj));
            after |= v.get((i + 1)..).unwrap().iter().any(|w| check(&order, *jj, *w));
        }
        if before || after {
            let mut ooo = v.clone();
            ooo.sort_by(|z, zz| check_or(&order, *z, *zz));
            println!("{:?}", ooo);
            acc += ooo.get(ooo.len() / 2).unwrap();
        }
    }
    acc
}

pub fn run(inp: Vec<String>) -> (i64, i64) {
    (part1(&inp), part2(&inp))
}
