fn part1(mut disk: Vec<u64>) -> u64 {
    let mut end_iter = disk.len() - 1;
    loop {
        if disk[end_iter] != 0 {
            break;
        }
        end_iter -= 1;
    }
    for i in 0.. {
        if i >= end_iter {
            break;
        } else if disk[i] == 0 {
            disk[i] = disk[end_iter];
            loop {
                end_iter -= 1;
                if disk[end_iter] != 0 {
                    break;
                }
            }
        }
    }
    let result = &disk[0..end_iter + 1];
    result.iter().enumerate().map(|(i, x)| i as u64 * (x - 1)).sum()
}

fn part2(mut inp: Vec<(u64, u64)>) -> u64 {
    let start_id: u64 = inp.last().unwrap().0;
    for i in (1..(start_id + 1)).rev() {
        let inx = inp.iter().position(|(xx, _)| *xx == i).unwrap();
        let siz = inp[inx].1;
        let mut j = 0;
        loop {
            if inp[j].0 == 0 && inp[j].1 > siz {
                inp[j].1 -= siz;
                inp[inx] = (0, siz);
                inp.insert(j, (i, siz));
                break;
            } else if inp[j].0 == 0 && inp[j].1 == siz {
                inp[j].0 = i;
                inp[inx] = (0, siz);
                break;
            } else if inp[j].0 == i {
                break;
            }
            j += 1;
        }
    }
    let mut disk: Vec<u64> = vec![];
    inp.iter().for_each(|(i, x)| {
        if *i == 0 {
            disk.extend(std::iter::repeat(0).take(*x as usize));
        } else {
            disk.extend(std::iter::repeat(*i).take(*x as usize));
        }
    });
    disk.iter()
        .enumerate()
        .map(|(i, x)| if *x != 0 { i as u64 * (x - 1) } else { 0 })
        .sum()
}

pub fn run(inp: Vec<String>) -> (i64, i64) {
    let mut disk: Vec<u64> = vec![];
    let mut dis: Vec<(u64, u64)> = vec![];
    inp.first().unwrap().chars().enumerate().for_each(|(i, x)| {
        let n: u64 = x.to_digit(10).unwrap() as u64;
        let id = (i / 2) + 1;
        if i % 2 == 0 {
            disk.extend(std::iter::repeat(id as u64).take(n as usize));
            dis.push((id as u64, n));
        } else {
            disk.extend(std::iter::repeat(0).take(n as usize));
            dis.push((0, n));
        }
    });

    (part1(disk) as i64, part2(dis) as i64)
}
