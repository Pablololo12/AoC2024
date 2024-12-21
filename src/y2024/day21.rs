use cached::proc_macro::cached;
use std::{
    cmp::min,
    collections::VecDeque,
    usize::{self, MAX},
};

#[cached]
fn where_control(prev_pos: char, new_pos: char, numpads: usize) -> usize {
    let num_pad = vec!['C', '^', 'A', '<', 'v', '>'];
    let pp = num_pad.iter().position(|w| *w == prev_pos).unwrap();
    let np = num_pad.iter().position(|w| *w == new_pos).unwrap();
    let pp = (pp / 3, pp % 3);
    let np = (np / 3, np % 3);
    let mut open: VecDeque<((usize, usize), String)> = VecDeque::new();
    open.push_back((pp, "".to_string()));
    let mut solution = MAX;
    while let Some((sc, typed)) = open.pop_front() {
        if np.0 == sc.0 && np.1 == sc.1 {
            solution = min(solution, where_robot(typed + "A", numpads - 1));
            continue;
        }
        if sc.0 == 0 && sc.1 == 0 {
            continue;
        } else {
            if sc.0 < np.0 {
                open.push_back(((sc.0 + 1, sc.1), typed.clone() + "v"));
            } else if sc.0 > np.0 {
                open.push_back(((sc.0 - 1, sc.1), typed.clone() + "^"));
            }
            if sc.1 < np.1 {
                open.push_back(((sc.0, sc.1 + 1), typed + ">"));
            } else if sc.1 > np.1 {
                open.push_back(((sc.0, sc.1 - 1), typed + "<"));
            }
        }
    }
    solution
}
fn where_robot(to_enter: String, numpads: usize) -> usize {
    if numpads == 0 {
        return to_enter.len();
    }
    let mut current = 'A';
    let mut result = 0;
    for c in to_enter.chars() {
        result += where_control(current, c, numpads);
        current = c
    }
    result
}

fn where_numpad(prev_pos: char, new_pos: char, numpads: usize) -> usize {
    let num_pad = vec!['7', '8', '9', '4', '5', '6', '1', '2', '3', 'C', '0', 'A'];
    let pp = num_pad.iter().position(|w| *w == prev_pos).unwrap();
    let np = num_pad.iter().position(|w| *w == new_pos).unwrap();
    let pp = (pp / 3, pp % 3);
    let np = (np / 3, np % 3);
    let mut open: VecDeque<((usize, usize), String)> = VecDeque::new();
    open.push_back((pp, "".to_string()));
    let mut solution = MAX;
    while let Some((sc, typed)) = open.pop_front() {
        if np.0 == sc.0 && np.1 == sc.1 {
            solution = min(solution, where_robot(typed + "A", numpads));
            continue;
        }
        if sc.0 == 3 && sc.1 == 0 {
            continue;
        } else {
            if sc.0 < np.0 {
                open.push_back(((sc.0 + 1, sc.1), typed.clone() + "v"));
            } else if sc.0 > np.0 {
                open.push_back(((sc.0 - 1, sc.1), typed.clone() + "^"));
            }
            if sc.1 < np.1 {
                open.push_back(((sc.0, sc.1 + 1), typed + ">"));
            } else if sc.1 > np.1 {
                open.push_back(((sc.0, sc.1 - 1), typed + "<"));
            }
        }
    }
    solution
}

pub fn run(inp: Vec<String>) -> (String, String) {
    let mut sum = 0;
    let mut sum2 = 0;
    for line in inp {
        let mut current = 'A';
        let mut result = 0;
        for c in line.chars() {
            result += where_numpad(current, c, 2);
            current = c
        }
        let mut result2 = 0;
        for c in line.chars() {
            result2 += where_numpad(current, c, 25);
            current = c
        }

        let val = line[0..3].parse::<usize>().unwrap();
        sum += result * val;
        sum2 += result2 * val;
    }
    (format!("{sum}"), format!("{sum2}"))
}
