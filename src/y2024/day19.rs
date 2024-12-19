use rustc_hash::FxHashMap;
use trie_rs::{Trie, TrieBuilder};

fn recursive(trie: &Trie<u8>, towel: &str, cache: &mut FxHashMap<String, usize>) -> usize {
    if towel.is_empty() {
        return 1;
    }

    if let Some(c) = cache.get(towel) {
        return *c;
    }

    let sum = trie
        .common_prefix_search(towel)
        .map(|w: Vec<u8>| recursive(trie, &towel[w.len()..], cache))
        .sum();
    cache.insert(towel.to_string(), sum);
    sum
}

pub fn run(inp: Vec<String>) -> (String, String) {
    let mut builder = TrieBuilder::new();
    inp[0].split(", ").for_each(|w| {
        builder.push(w);
    });
    let trie = builder.build();

    let towels = &inp[2..];

    let mut p1 = 0;
    let mut p2 = 0;
    let mut cache: FxHashMap<String, usize> = FxHashMap::default();
    for t in towels {
        let count = recursive(&trie, t, &mut cache);
        if count > 0 {
            p1 += 1;
            p2 += count;
        }
    }
    (format!("{p1}"), format!("{p2}"))
}
