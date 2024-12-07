fn search(test: i64, acc: i64, inn: &[i64]) -> bool {
    if inn.len() == 0 {
        return test == acc;
    } else if acc > test {
        return false;
    }
    let sum_path = search(test, acc + inn.first().unwrap(), &inn[1..]);
    let mul_path = search(test, acc * inn.first().unwrap(), &inn[1..]);
    sum_path || mul_path
}

fn search2(test: i64, acc: i64, inn: &[i64]) -> bool {
    if inn.len() == 0 {
        return test == acc;
    } else if acc > test {
        return false;
    }
    let head: i64 = inn.first().unwrap().clone();
    let num_digits = head.checked_ilog10().unwrap_or(0) as u32 + 1;

    let sum_path = search2(test, acc + head, &inn[1..]);
    let aggregate_path = search2(test, (acc * (10_i64.pow(num_digits))) + head, &inn[1..]);
    let mul_path = search2(test, acc * head, &inn[1..]);
    sum_path || mul_path || aggregate_path
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
    let p1 = inn.iter().filter(|z| search(z.0, 0, &z.1)).map(|(u, _)| u).sum();
    let p2 = inn.iter().filter(|z| search2(z.0, 0, &z.1)).map(|(u, _)| u).sum();

    (p1, p2)
}
