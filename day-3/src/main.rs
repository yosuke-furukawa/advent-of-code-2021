use std::fs;

fn part1() {
    let text: String = fs::read_to_string("./data.txt").unwrap();
    let data: Vec<Vec<i32>> = text
        .split_terminator('\n')
        .into_iter()
        .map(|x| x.chars().map(|x| x as i32 - '0' as i32).collect())
        .collect();
    let mut a = 0;
    let mut b = 0;
    let x = data[0].len() as u32;
    for i in 0..x {
        let mut zero_count = 0;
        for d in &data {
            if d[i as usize] == 0 {
                zero_count += 1;
            }
        }
        if zero_count > data.len() / 2 {
            b += 2_i32.pow(x - i - 1);
        } else {
            a += 2_i32.pow(x - i - 1);
        }
    }
    println!("a={}, b={}", a, b);
    println!("{}", a * b);
}

fn part2() {
    let text: String = fs::read_to_string("./data.txt").unwrap();
    let data: Vec<Vec<i32>> = text
        .split_terminator('\n')
        .into_iter()
        .map(|x| x.chars().map(|x| x as i32 - '0' as i32).collect())
        .collect();
    let x = data[0].len();
    let mut oxys = data.clone();
    for i in 0..x {
        if oxys.len() == 1 {
            break;
        }
        let mut one_count = 0;
        let mut zero_count = 0;
        for o in oxys.iter() {
            if o[i] == 0 {
                zero_count += 1;
            } else {
                one_count += 1;
            }
        }
        let filter = match (zero_count, one_count) {
            (z, o) if z > o => 0,
            (z, o) if z < o => 1,
            _ => 1,
        };
        oxys = oxys.into_iter().filter(|d| d[i] == filter).collect();
    }
    let mut co2s = data;
    for i in 0..x {
        if co2s.len() == 1 {
            break;
        }
        let mut one_count = 0;
        let mut zero_count = 0;
        for c in co2s.iter() {
            if c[i] == 0 {
                zero_count += 1;
            } else {
                one_count += 1;
            }
        }
        let filter = match (zero_count, one_count) {
            (z, o) if z > o => 1,
            (z, o) if z < o => 0,
            _ => 0,
        };
        co2s = co2s.into_iter().filter(|d| d[i] == filter).collect();
    }
    let mut oxy = 0;
    for (i, o) in oxys[0].iter().enumerate() {
        if *o == 1 {
            oxy += 2_i32.pow(x as u32 - i as u32 - 1);
        }
    }
    let mut co2 = 0;
    for (i, c) in co2s[0].iter().enumerate() {
        if *c == 1 {
            co2 += 2_i32.pow(x as u32 - i as u32 - 1);
        }
    }
    println!("{}", oxy * co2);
}

fn main() {
    part1();
    part2();
}
