pub mod helper;
pub mod y2024;

use argparse::{ArgumentParser, Store, StoreTrue};
use helper::{get_example_input, get_input};
use std::time::Instant;

fn run(which: i32, input: Vec<String>) -> (i64, i64) {
    let now = Instant::now();
    let (first, second) = match which {
        1 => y2024::day01::run(input),
        2 => y2024::day02::run(input),
        3 => y2024::day03::run(input),
        4 => y2024::day04::run(input),
        5 => y2024::day05::run(input),
        6 => y2024::day06::run(input),
        7 => todo!(),
        8 => todo!(),
        9 => todo!(),
        10 => todo!(),
        11 => todo!(),
        12 => todo!(),
        13 => todo!(),
        14 => todo!(),
        15 => todo!(),
        16 => todo!(),
        17 => todo!(),
        18 => todo!(),
        19 => todo!(),
        20 => todo!(),
        21 => todo!(),
        22 => todo!(),
        23 => todo!(),
        24 => todo!(),
        25 => todo!(),
        _ => (0, 0),
    };
    let elapsed = now.elapsed();
    println!(
        "Day {} first solution: {}, second solution {} in {:.2?}",
        which, first, second, elapsed
    );
    (first, second)
}

fn run_all() {
    for i in 1..=25 {
        run(i, get_input(i).expect("No valid input"));
    }
}

fn main() {
    let mut day = 0;
    let mut example = false;
    let mut example_n = 0;
    {
        // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("Rust solution to Advent of Code");
        ap.refer(&mut day).add_option(&["-d"], Store, "Input day number");
        ap.refer(&mut example)
            .add_option(&["-e"], StoreTrue, "Runs examples rather than inputs");
        ap.refer(&mut example_n)
            .add_option(&["-n"], Store, "Choose example number starting with 1");
        ap.parse_args_or_exit();
    }

    println!(
        "Options selected: day {}, example {}, which example {}",
        day, example, example_n
    );

    if day == 0 {
        run_all();
    } else if example {
        run(day, get_example_input(day, example_n).expect("No valid input"));
    } else {
        run(day, get_input(day).expect("No valid input"));
    }
}
