use std::collections::BTreeSet;

use rustc_hash::{FxHashMap, FxHashSet};

fn part2(groups: &FxHashMap<String, FxHashSet<String>>) -> Vec<Vec<String>> {
    let mut p: BTreeSet<String> = BTreeSet::from_iter(groups.keys().cloned());
    let mut cliques: Vec<Vec<String>> = vec![];
    while let Some(pp) = p.pop_first() {
        let mut accum: FxHashMap<String, u32> = FxHashMap::default();
        for cc in groups.get(&pp).unwrap() {
            *accum.entry(cc.clone()).or_insert(0) += 1;
            for dd in groups.get(cc).unwrap() {
                *accum.entry(dd.clone()).or_insert(0) += 1;
            }
        }
        let ma = accum.values().max().unwrap();
        for i in (0..=*ma).rev() {
            let cou = accum.values().filter(|f| **f == i).count() as u32;
            if cou == i {
                let mut gh: Vec<String> = accum.iter().filter(|(_, v)| **v == i).map(|(k, _)| k.clone()).collect();
                gh.push(pp.clone());
                cliques.push(gh);
            }
        }
    }
    cliques
}

pub fn run(inp: Vec<String>) -> (String, String) {
    let mut groups: FxHashMap<String, FxHashSet<String>> = FxHashMap::default();
    inp.iter().for_each(|w| {
        let mut s = w.split('-');
        let fst = s.next().unwrap().to_string();
        let snd = s.next().unwrap().to_string();
        groups.entry(fst.clone()).or_default().insert(snd.clone());
        groups.entry(snd).or_default().insert(fst);
    });

    let mut triangles: FxHashSet<(String, String, String)> = FxHashSet::default();
    let mut count = 0;

    for i in groups.keys() {
        for j in groups[i].iter() {
            for k in groups[j].iter() {
                if groups[k].contains(i) && i < j && j < k {
                    triangles.insert((i.clone(), j.clone(), k.clone()));
                    if i.starts_with('t') || j.starts_with('t') || k.starts_with('t') {
                        count += 1;
                    }
                }
            }
        }
    }
    let cl = part2(&groups);
    let ma = cl.iter().map(|w| w.len()).max().unwrap();
    let mm: Vec<Vec<String>> = cl
        .iter()
        .filter(|w| w.len() == ma)
        .map(|w| {
            let mut e = w.clone();
            e.sort();
            e
        })
        .collect();
    let my = mm.first().unwrap().join(",");
    (format!("{count}"), format!("{my}"))
}
