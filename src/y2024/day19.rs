use rustc_hash::FxHashMap;
use trie_rs::{Trie, TrieBuilder};

fn recursive(trie: &Trie<u8>, towel: &str, cache: &mut FxHashMap<String, usize>) -> usize {
    match cache.get(towel) {
        Some(c) => *c,
        _ if towel.is_empty() => 1,
        _ => {
            let sum = trie
                .common_prefix_search(towel)
                .map(|w: Vec<u8>| recursive(trie, &towel[w.len()..], cache))
                .sum();
            cache.insert(towel.to_string(), sum);
            sum
        }
    }
}

pub fn run(inp: Vec<String>) -> (String, String) {
    let mut builder = TrieBuilder::new();
    inp[0].split(", ").for_each(|w| {
        builder.push(w);
    });
    let trie = builder.build();

    let mut cache: FxHashMap<String, usize> = FxHashMap::default();
    let (p1, p2): (usize, usize) = inp[2..]
        .iter()
        .map(|t| match recursive(&trie, t, &mut cache) {
            0 => (0usize, 0),
            d => (1usize, d),
        })
        .fold((0, 0), |(a, aa), (pp1, pp2)| (a + pp1, aa + pp2));
    (format!("{p1}"), format!("{p2}"))
}
