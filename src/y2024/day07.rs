fn search(test: i64, acc: i64, inn: &[i64]) -> i64 {
    if inn.len() == 0 && test == acc {
        return test;
    } else if inn.len() == 0 {
        return 0;
    } else if acc > test {
        return 0;
    }
    let sum_path = search(test, acc + inn.first().unwrap(), &inn[1..]);
    let mut mul_path = 0;
    if acc != 0 {
        mul_path = search(test, acc * inn.first().unwrap(), &inn[1..]);
    }
    if sum_path != 0 || mul_path != 0 {
        return test;
    }
    0
}

fn search2(test: i64, acc: i64, inn: &[i64]) -> i64 {
    if inn.len() == 0 && test == acc {
        return test;
    } else if inn.len() == 0 {
        return 0;
    } else if acc > test {
        return 0;
    }
    let head: i64 = inn.first().unwrap().clone();
    let num_digits = head.checked_ilog10().unwrap_or(0) as u32 + 1;
    let sum_path = search2(test, acc + head, &inn[1..]);
    let aggregate_path = search2(test, (acc * (10_i64.pow(num_digits))) + head, &inn[1..]);
    let mut mul_path = 0;
    if acc != 0 {
        mul_path = search2(test, acc * head, &inn[1..]);
    }
    if sum_path != 0 || mul_path != 0 || aggregate_path != 0 {
        return test;
    }
    0
}

fn part1(test: i64, inn: &Vec<i64>) -> i64 {
    search(test, 0, &inn[..])
}

fn part2(test: i64, inn: &Vec<i64>) -> i64 {
    search2(test, 0, &inn[..])
}

pub fn run(inp: Vec<String>) -> (i64, i64) {
    let inn: Vec<(i64, Vec<i64>)> = inp
        .iter()
        .map(|y| {
            let mut kk = y.split(":");
            (
                kk.next().unwrap().parse::<i64>().unwrap(),
                kk.next()
                    .unwrap()
                    .split_whitespace()
                    .map(|x| x.parse::<i64>().unwrap())
                    .collect(),
            )
        })
        .collect();
    let p1 = inn.iter().map(|z| part1(z.0, &z.1)).sum();
    let p2 = inn.iter().map(|z| part2(z.0, &z.1)).sum();

    (p1, p2)
}
