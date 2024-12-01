use std::{
    fs::File,
    io::{prelude::*, BufReader, stdin, stdout},
    path::Path,
    env,
    fs,
};
use reqwest::{
    cookie::Jar,
    Url,
    blocking::Client
};
use scraper::{
    Html,
    Selector,
};

pub fn get_input(day: i32) -> Option<Vec<String>> {
    if !fs::exists(get_path(day)).expect("FS corrupted") {
        get_input_data(day)?;
    }
    Some(lines_from_file(get_path(day)))
}

pub fn get_example_input(day: i32, num: i32) -> Option<Vec<String>> {
    if !fs::exists(get_example_path(day, num)).expect("FS corrupted") {
        get_example_data(day, num)?;
    }
    Some(lines_from_file(get_example_path(day, num)))
}

fn get_path(day: i32) -> String {
    format!("./inputs/day_{day}.txt")
}

fn get_example_path(day: i32, num: i32) -> String {
    format!("./inputs/day_{}_{}.txt", day, num)
}

fn get_cookie() -> String {
    if env::var("AOC_COOKIE").is_ok() {
        return env::var("AOC_COOKIE").expect("This should not happen");
    }
    if fs::exists("./.cookie").expect("FS corrupted") {
        return lines_from_file("./.cookie").first().expect("LLL").to_owned();
    }

    println!("Enter aoc cookie: ");
    let _ = stdout().flush();
    let mut s = String::new();
    stdin().read_line(&mut s).expect("That was not correct");
    fs::write("./.cookie", s.clone()).ok();
    s
}

fn get_internet_data(addr: String) -> Option<String> {
    let cookie = get_cookie();
    let url = "https://adventofcode.com".parse::<Url>().unwrap();

    let jar = Jar::default();
    jar.add_cookie_str(&cookie, &url);

    let client = Client::builder()
        .cookie_provider(jar.into())
        .build()
        .expect("No cookies");
    client.get(addr)
        .send()
        .ok()?
        .text()
        .ok()
}

fn get_example_data(day: i32, num: i32) -> Option<()> {
    let res = get_internet_data(format!("https://adventofcode.com/2023/day/{day}"))?;
    let fragment = Html::parse_fragment(&res);
    let pre_selector = Selector::parse("pre").unwrap();
    let code_selector = Selector::parse("code").unwrap();

    let mut i = 1;
    for element in fragment.select(&pre_selector) {
        for e in element.select(&code_selector) {
            if i==num {
                let inner = e.inner_html()
                    .replace("&gt;", ">")
                    .replace("&lt;", "<")
                    .replace("&amp;", "&");
                fs::write(get_example_path(day, num), inner).ok()?;
                return Some(())
            }
            i+=1;
        }
    }
    None
}

fn get_input_data(day: i32) -> Option<()> {
    let res = get_internet_data(format!("https://adventofcode.com/2023/day/{day}/input"))?;
    fs::write(get_path(day), res).ok()?;
    Some(())
}


fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
