use std::fs;

fn part1() {
    let text: String = fs::read_to_string("./data.txt").unwrap();
    let splitted: Vec<&str> = text.split_terminator('\n').collect();
    let data: Vec<i32> = splitted.iter().map(|x| x.parse::<i32>().unwrap()).collect();
    let mut count = 0;
    for (i, v) in data.iter().enumerate().skip(1) {
        if *v > data[i - 1] {
            count += 1;
        }
    }
    println!("{}", count);
}

fn part2() {
    let text: String = fs::read_to_string("./data.txt").unwrap();
    let splitted: Vec<&str> = text.split_terminator('\n').collect();
    let data: Vec<i32> = splitted.iter().map(|x| x.parse::<i32>().unwrap()).collect();
    let mut sums = vec![];
    for i in 0..data.len() - 2 {
        sums.push(data[i] + data[i + 1] + data[i + 2]);
    }
    let mut count = 0;
    for (i, v) in sums.iter().enumerate().skip(1) {
        if *v > sums[i - 1] {
            count += 1;
        }
    }
    println!("{}", count);
}

fn main() {
    part1();
    part2();
}
