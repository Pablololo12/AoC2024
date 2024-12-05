use std::cmp::Ordering;

pub fn parse_pages(inp: &Vec<String>) -> (Vec<(i64, i64)>, Vec<Vec<i64>>) {
    let mut out: Vec<Vec<i64>> = vec![];
    let mut output: Vec<(i64, i64)> = vec![];
    inp.iter().for_each(|f| {
        if f.contains(",") {
            out.push(f.split(",").map(|jj| jj.parse::<i64>().unwrap()).collect());
        }
        if f.contains("|") {
            let mut kk = f.split("|");
            output.push((
                kk.next().unwrap().parse::<i64>().unwrap(),
                kk.next().unwrap().parse::<i64>().unwrap(),
            ));
        }
    });
    (output, out)
}

pub fn check_or(inp: &Vec<(i64, i64)>, val1: i64, val2: i64) -> Ordering {
    if inp.iter().any(|(i, j)| *i == val2 && *j == val1) {
        return Ordering::Greater;
    } else {
        return Ordering::Less;
    }
}

pub fn run(inp: Vec<String>) -> (i64, i64) {
    let (order, pages) = parse_pages(&inp);
    let mut acc = 0;
    let mut acc2 = 0;
    for v in pages.iter() {
        let mut copy = v.clone();
        copy.sort_by(|z, zz| check_or(&order, *z, *zz));
        let same = !v.iter().zip(&copy).any(|(j, jj)| *j != *jj);
        if same {
            acc += copy.get(copy.len() / 2).unwrap();
        } else {
            acc2 += copy.get(copy.len() / 2).unwrap();
        }
    }
    (acc, acc2)
}
