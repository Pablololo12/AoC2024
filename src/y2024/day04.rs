use regex::Regex;

pub fn find(inp: &Vec<String>) -> i64 {
    let re = Regex::new(r"XMAS").unwrap();
    let rere = Regex::new(r"SAMX").unwrap();
    inp.into_iter().map(|x| re.find_iter(x).count() as i64).sum::<i64>()
        + inp.into_iter().map(|x| rere.find_iter(x).count() as i64).sum::<i64>()
}

pub fn traspose(inp: &Vec<String>) -> Vec<String> {
    let max = inp.iter().map(|f| f.len()).max().unwrap();
    let mut output: Vec<String> = vec![String::new(); max];
    inp.iter().for_each(|s| {
        s.chars().enumerate().for_each(|(i, f)| {
            output[i].push(f);
        })
    });
    output
}

pub fn shift_r(inp: &Vec<String>) -> Vec<String> {
    let mut output: Vec<String> = vec![String::new(); inp.len()];
    let lll = inp.len();
    for i in 0..lll {
        for _ in i..(lll - 1) {
            output[i].push('Y');
        }
        output[i] += &inp[i];
    }
    output
}

pub fn shift_l(inp: &Vec<String>) -> Vec<String> {
    let mut output: Vec<String> = vec![String::new(); inp.len()];
    let lll = inp.len();
    for i in 0..lll {
        for _ in 0..i {
            output[i].push('Y');
        }
        output[i] += &inp[i];
    }
    output
}

pub fn part1(inp: &Vec<String>) -> i64 {
    let horizontal = find(inp);

    let trasp = traspose(inp);
    let vertical = find(&trasp);

    let sr = shift_r(inp);
    let tsr = traspose(&sr);
    let diar = find(&tsr);

    let sl = shift_l(inp);
    let tsl = traspose(&sl);
    let dial = find(&tsl);
    horizontal + vertical + diar + dial
}

pub fn part2(inp: &Vec<String>) -> i64 {
    let new: Vec<Vec<char>> = inp.iter().map(|f| f.chars().collect()).collect();
    let mut acc = 0;
    for i in 1..new.len() - 1 {
        for j in 1..new[0].len() - 1 {
            let m = new[i][j];
            if m == 'A' {
                if new[i - 1][j - 1] == 'M'
                    && new[i - 1][j + 1] == 'M'
                    && new[i + 1][j - 1] == 'S'
                    && new[i + 1][j + 1] == 'S'
                {
                    acc += 1;
                }
                if new[i - 1][j - 1] == 'M'
                    && new[i - 1][j + 1] == 'S'
                    && new[i + 1][j - 1] == 'M'
                    && new[i + 1][j + 1] == 'S'
                {
                    acc += 1;
                }
                if new[i - 1][j - 1] == 'S'
                    && new[i - 1][j + 1] == 'S'
                    && new[i + 1][j - 1] == 'M'
                    && new[i + 1][j + 1] == 'M'
                {
                    acc += 1;
                }
                if new[i - 1][j - 1] == 'S'
                    && new[i - 1][j + 1] == 'M'
                    && new[i + 1][j - 1] == 'S'
                    && new[i + 1][j + 1] == 'M'
                {
                    acc += 1;
                }
            }
        }
    }
    acc
}

pub fn run(inp: Vec<String>) -> (i64, i64) {
    (part1(&inp), part2(&inp))
}
