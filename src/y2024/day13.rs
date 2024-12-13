use regex::Regex;

pub fn run(inp: Vec<String>) -> (i64, i64) {
    let re = Regex::new(r"([0-9]+)[^0-9]*([0-9]+)").unwrap();
    let mut total = 0;
    let mut total2 = 0;
    let mut counter = 0;
    let mut ax = 0;
    let mut ay = 0;
    let mut bx = 0;
    let mut by = 0;
    let mut x = 0;
    let mut y = 0;
    for i in inp {
        if i.contains('A') {
            if let Some(c) = re.captures(&i) {
                ax = c.get(1).unwrap().as_str().parse::<i64>().unwrap();
                ay = c.get(2).unwrap().as_str().parse::<i64>().unwrap();
                counter += 1;
            }
        }
        if i.contains("B:") {
            if let Some(c) = re.captures(&i) {
                bx = c.get(1).unwrap().as_str().parse::<i64>().unwrap();
                by = c.get(2).unwrap().as_str().parse::<i64>().unwrap();
                counter += 1;
            }
        }
        if i.contains('=') {
            if let Some(c) = re.captures(&i) {
                x = c.get(1).unwrap().as_str().parse::<i64>().unwrap();
                y = c.get(2).unwrap().as_str().parse::<i64>().unwrap();
                counter += 1;
            }
        }
        if counter == 3 {
            println!("{} {} {} {}", ax, ay, bx, by);
            counter = 0;
            let det = ax * by - bx * ay;
            if det == 0 {
                println!("WHOOPS");
                continue;
            }
            println!("{det}");
            let iax = by;
            let ibx = -bx;
            let iay = -ay;
            let iby = ax;
            println!("{} {} {} {}", iax, iay, ibx, iby);
            let mut a = iax * x + ibx * y;
            let mut b = iay * x + iby * y;
            a /= det;
            b /= det;
            let res1 = a * ax + b * bx;
            let res2 = a * ay + b * by;
            if res1 == x && res2 == y {
                println!("{} {}", a, b);
                total += a * 3 + b;
            }
            x += 10000000000000;
            y += 10000000000000;
            let mut a2 = iax * x + ibx * y;
            let mut b2 = iay * x + iby * y;
            a2 /= det;
            b2 /= det;
            let res12 = a2 * ax + b2 * bx;
            let res22 = a2 * ay + b2 * by;
            if res12 == x && res22 == y {
                println!("*{} {}", a2, b2);
                total2 += a2 * 3 + b2;
            }
        }
    }
    (total, total2)
}
