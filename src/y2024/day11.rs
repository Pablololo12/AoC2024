use cached::proc_macro::cached;

#[cached]
fn recursive_solv(n: u64, d: u64) -> u64 {
    if d == 0 {
        return 1;
    }
    let digs = n.checked_ilog10().unwrap_or(0) + 1;
    let count = match n {
        0 => recursive_solv(1, d - 1),
        y if digs % 2 == 0 => {
            recursive_solv(y / 10_u64.pow(digs / 2), d - 1) + recursive_solv(y % 10_u64.pow(digs / 2), d - 1)
        }
        _ => recursive_solv(n * 2024, d - 1),
    };
    count
}

pub fn run(inp: Vec<String>) -> (String, String) {
    let arr: Vec<u64> = inp
        .first()
        .unwrap()
        .split_whitespace()
        .map(|f| f.parse::<u64>().unwrap())
        .collect();
    let p1: u64 = arr.iter().map(|w| recursive_solv(*w, 25)).sum();
    let p2: u64 = arr.iter().map(|w| recursive_solv(*w, 75)).sum();
    (format!("{}", p1), format!("{}", p2))
}
