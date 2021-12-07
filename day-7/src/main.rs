use std::fs;

fn parse(arg: &str) -> Vec<i32> {
    let text: String = fs::read_to_string(arg).unwrap();
    let results: Vec<i32> = text
        .split(',')
        .into_iter()
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect();
    results
}

fn part1(arg: &str) -> i32 {
    let data = parse(arg);
    let min = *data.iter().min().unwrap();
    let max = *data.iter().max().unwrap();
    let mut result = 1_000_000_000;
    for i in min..=max {
        let sum = data.iter().map(|x| (*x - i).abs()).sum();
        result = result.min(sum);
    }
    result
}

fn part2(arg: &str) -> i32 {
    let data = parse(arg);
    let min = *data.iter().min().unwrap();
    let max = *data.iter().max().unwrap();
    let mut result = 1_000_000_000;
    for i in min..=max {
        let sum = data
            .iter()
            .map(|x| ((*x - i).abs() * ((*x - i).abs() + 1)) / 2)
            .sum();
        result = result.min(sum);
    }
    result
}

fn main() {
    println!("{:?}", part1("./data.txt"));
    println!("{:?}", part2("./data.txt"));
}
