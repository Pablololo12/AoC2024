use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn search_p2(acc: i64, inn: &[i64]) -> bool {
    if inn.len() == 0 {
        return acc == 0;
    } else if acc <= 0 {
        return false;
    }

    let head: i64 = inn[0];
    let num_digits = head.checked_ilog10().unwrap_or(0) as u32 + 1;
    let hundred = 10_i64.pow(num_digits);

    let sum_path = search_p2(acc - head, &inn[1..]);
    let mul_path = match acc % head {
        0 => search_p2(acc / head, &inn[1..]),
        _ => false,
    };
    let aggregate_path = match (acc % hundred) == head {
        true => search_p2(acc / hundred, &inn[1..]),
        _ => false,
    };
    sum_path || mul_path || aggregate_path
}

fn search_p1(acc: i64, inn: &[i64]) -> bool {
    if inn.len() == 0 {
        return acc == 0;
    } else if acc <= 0 {
        return false;
    }

    let head: i64 = inn[0];

    let sum_path = search_p1(acc - head, &inn[1..]);
    let mul_path = match acc % head {
        0 => search_p1(acc / head, &inn[1..]),
        _ => false,
    };

    sum_path || mul_path
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
                    .rev()
                    .collect(),
            )
        })
        .collect();

    let p1 = inn
        .clone()
        .into_par_iter()
        .filter(|z| search_p1(z.0, &z.1))
        .map(|(u, _)| u)
        .sum();
    let p2 = inn
        .into_par_iter()
        .filter(|z| search_p2(z.0, &z.1))
        .map(|(u, _)| u)
        .sum();

    (p1, p2)
}
