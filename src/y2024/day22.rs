use rustc_hash::FxHashMap;
fn part1(num: u64) -> u64 {
    let mut prev = num;
    for _ in 0..2000 {
        prev = ((prev * 64) ^ prev) % 16777216;
        prev = ((prev / 32) ^ prev) % 16777216;
        prev = ((prev * 2048) ^ prev) % 16777216;
    }
    prev
}

fn part2(num: u64) -> FxHashMap<(i32, i32, i32, i32), i32> {
    let mut hm: FxHashMap<(i32, i32, i32, i32), i32> = FxHashMap::default();
    let mut nums: Vec<u64> = vec![];
    let mut prev = num;
    nums.push(num);
    for _ in 0..2000 {
        prev = ((prev * 64) ^ prev) % 16777216;
        prev = ((prev / 32) ^ prev) % 16777216;
        prev = ((prev * 2048) ^ prev) % 16777216;
        nums.push(prev);
    }
    let changes: Vec<(i32, i32)> = nums
        .windows(2)
        .map(|pair| ((pair[1] % 10) as i32 - (pair[0] % 10) as i32, (pair[1] % 10) as i32))
        .collect();

    for nums in changes.windows(4) {
        let key = (nums[0].0, nums[1].0, nums[2].0, nums[3].0);
        if !hm.contains_key(&key) {
            hm.insert(key, nums[3].1);
        }
    }

    hm
}

pub fn run(inp: Vec<String>) -> (String, String) {
    let nums: Vec<u64> = inp.iter().map(|w| part1(w.parse::<u64>().unwrap())).collect();
    let p1: u64 = nums.iter().sum();
    let hashes: Vec<FxHashMap<(i32, i32, i32, i32), i32>> =
        inp.iter().map(|n| part2(n.parse::<u64>().unwrap())).collect();
    let mut hm: FxHashMap<(i32, i32, i32, i32), i32> = FxHashMap::default();

    for hh in hashes.iter() {
        for key in hh.keys() {
            if !hm.contains_key(key) {
                let sum: i32 = hashes.iter().map(|map| map.get(key).unwrap_or(&0)).sum();
                hm.insert(*key, sum);
            }
        }
    }
    let p2: i32 = hm.iter().map(|(_, v)| *v).max().unwrap();
    (format!("{p1}"), format!("{p2}"))
}
