pub mod helper;
pub mod y2024;

use argparse::{ArgumentParser, Store, StoreTrue};
use helper::{get_example_input, get_input};
use std::time::Instant;

fn run(which: i32, input: Vec<String>, easter: bool) -> (String, String) {
    let now = Instant::now();
    let (first, second) = match which {
        1 => y2024::day01::run(input),
        2 => y2024::day02::run(input),
        3 => y2024::day03::run(input),
        4 => y2024::day04::run(input),
        5 => y2024::day05::run(input),
        6 => y2024::day06::run(input),
        7 => y2024::day07::run(input),
        8 => y2024::day08::run(input),
        9 => y2024::day09::run(input),
        10 => y2024::day10::run(input),
        11 => y2024::day11::run(input),
        12 => y2024::day12::run(input),
        13 => y2024::day13::run(input),
        14 => y2024::day14::run(input, easter),
        15 => y2024::day15::run(input, easter),
        16 => y2024::day16::run(input, easter),
        17 => y2024::day17::run(input),
        _ => ("NOO".to_string(), "NOO".to_string()),
    };
    let elapsed = now.elapsed();
    println!(
        "Day {:02} first solution: {}, second solution {} in {:.2?}",
        which, first, second, elapsed
    );
    (first, second)
}

fn run_all(easter: bool) {
    for i in 1..=25 {
        run(i, get_input(i).expect("No valid input"), easter);
    }
}

fn main() {
    let mut day = 0;
    let mut example_n = 0;
    let mut easter = false;
    {
        // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("Rust solution to Advent of Code");
        ap.refer(&mut day).add_option(&["-d"], Store, "Input day number");
        ap.refer(&mut example_n)
            .add_option(&["-e"], Store, "Runs examples rather than inputs");
        ap.refer(&mut easter).add_option(&["--easter"], StoreTrue, "Surprise!");
        ap.parse_args_or_exit();
    }

    println!("Options selected: day {}, which example {}", day, example_n);

    if day == 0 {
        run_all(easter);
    } else if example_n != 0 {
        run(day, get_example_input(day, example_n).expect("No valid input"), easter);
    } else {
        run(day, get_input(day).expect("No valid input"), easter);
    }
}
