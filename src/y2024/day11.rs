use std::collections::HashMap;

use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn merge_pairs(p: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let mut counts: HashMap<u64, u64> = HashMap::new();

    p.iter().for_each(|(number, count)| {
        *counts.entry(*number).or_insert(0) += count;
    });

    counts.into_iter().collect()
}

fn solve(mut v: Vec<(u64, u64)>, n: u64) -> u64 {
    for _ in 0..n {
        for i in 0..v.len() {
            if v[i].0 == 0 {
                v[i].0 = 1;
            } else {
                let num_digits = v[i].0.checked_ilog10().unwrap_or(0) as u32 + 1;
                if num_digits % 2 == 0 {
                    let dv = 10_u64.pow(num_digits / 2);
                    v.push((v[i].0 % dv, v[i].1));
                    v[i].0 = v[i].0 / dv;
                } else {
                    v[i].0 *= 2024;
                }
            }
        }
        v = merge_pairs(v);
    }
    v.iter().fold(0u64, |acc, w| acc + w.1 as u64)
}

pub fn run(inp: Vec<String>) -> (i64, i64) {
    let arr: Vec<(u64, u64)> = inp
        .first()
        .unwrap()
        .split_whitespace()
        .map(|f| (f.parse::<u64>().unwrap(), 1))
        .collect();
    let p1: u64 = arr.clone().into_par_iter().map(|(w, _)| solve(vec![(w, 1)], 25)).sum();
    let p2: u64 = arr.into_par_iter().map(|(w, _)| solve(vec![(w, 1)], 75)).sum();
    (p1 as i64, p2 as i64)
    //(solve(arr.clone(), 25) as i64, solve(arr, 75) as i64)
}
