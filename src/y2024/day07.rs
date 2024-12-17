use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn search_p2(acc: i64, inn: &[i64]) -> bool {
    if inn.len() == 0 {
        return acc == 0;
    }

    let head: i64 = inn[0];
    if acc < head {
        return false;
    }

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
    }

    let head: i64 = inn[0];
    if acc < head {
        return false;
    }

    let sum_path = search_p1(acc - head, &inn[1..]);
    let mul_path = match acc % head {
        0 => search_p1(acc / head, &inn[1..]),
        _ => false,
    };

    sum_path || mul_path
}

pub fn run(inp: Vec<String>) -> (String, String) {
    let (p1, p2) = inp
        .into_par_iter()
        .map(|y| {
            let mut kk = y.split(":");
            (
                kk.next().unwrap().parse::<i64>().unwrap(),
                kk.next()
                    .unwrap()
                    .split_whitespace()
                    .map(|x| x.parse::<i64>().unwrap())
                    .rev()
                    .collect::<Vec<i64>>(),
            )
        })
        .map(|(z, l)| {
            if search_p1(z, &l) {
                (z, z)
            } else if search_p2(z, &l) {
                (0, z)
            } else {
                (0, 0)
            }
        })
        .reduce(|| (0, 0), |(a, b), (c, d)| (a + c, b + d));
    (format!("{}", p1), format!("{}", p2))
}
