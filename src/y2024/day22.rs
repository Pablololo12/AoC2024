use rustc_hash::{FxHashMap, FxHashSet};

fn madness(inp: Vec<u64>) -> (u64, i32) {
    let mut full: FxHashMap<(i32, i32, i32, i32), i32> = FxHashMap::default();
    let mut p1: Vec<u64> = vec![];
    for num in inp {
        let mut hm: FxHashSet<(i32, i32, i32, i32)> = FxHashSet::default();
        let mut nums: Vec<i32> = vec![];
        let mut prev = num;
        nums.push(num as i32);
        for _ in 0..2000 {
            prev = ((prev * 64) ^ prev) % 16777216;
            prev = ((prev / 32) ^ prev) % 16777216;
            prev = ((prev * 2048) ^ prev) % 16777216;
            nums.push((prev % 10) as i32);
        }
        p1.push(prev);
        let changes: Vec<(i32, i32)> = nums.windows(2).map(|pair| (pair[1] - pair[0], pair[1])).collect();

        for nums in changes.windows(4) {
            let key = (nums[0].0, nums[1].0, nums[2].0, nums[3].0);
            if hm.insert(key) {
                full.entry(key).and_modify(|e| *e += nums[3].1).or_insert(nums[3].1);
            }
        }
    }
    (p1.iter().sum(), full.iter().map(|(_, v)| *v).max().unwrap())
}

pub fn run(inp: Vec<String>) -> (String, String) {
    let nums: Vec<u64> = inp.iter().map(|w| w.parse::<u64>().unwrap()).collect();
    let (p1, p2) = madness(nums);
    (format!("{p1}"), format!("{p2}"))
}
