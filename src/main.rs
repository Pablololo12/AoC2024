pub mod helper;
pub mod y2024;

use argparse::{ArgumentParser, Store, StoreTrue};
use helper::{get_example_input, get_input};

fn run(which: i32, input: Vec<String>) -> (i64, i64) {
    let (first, second) = match which {
        1 => y2024::day01::run(input),
        2 => todo!(),
        3 => todo!(),
        4 => todo!(),
        5 => todo!(),
        6 => todo!(),
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
    println!(
        "Day {} first solution: {}, second solution {}",
        which, first, second
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
        ap.refer(&mut day)
            .add_option(&["-d"], Store, "Input day number");
        ap.refer(&mut example)
            .add_option(&["-e"], StoreTrue, "Runs examples rather than inputs");
        ap.refer(&mut example_n).add_option(
            &["-n"],
            Store,
            "Choose example number starting with 1",
        );
        ap.parse_args_or_exit();
    }

    println!(
        "Options selected: day {}, example {}, which example {}",
        day, example, example_n
    );

    if day == 0 {
        run_all();
    } else if example {
        run(
            day,
            get_example_input(day, example_n).expect("No valid input"),
        );
    } else {
        run(day, get_input(day).expect("No valid input"));
    }
}
