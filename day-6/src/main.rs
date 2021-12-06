use std::fs;

fn parse(arg: &str) -> Vec<i32> {
    let text: String = fs::read_to_string(arg).unwrap();
    let results: Vec<i32> = text
        .split_terminator(',')
        .into_iter()
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect();
    results
}

fn part1(arg: &str, days: i32) -> i32 {
    let mut data = parse(arg);
    for _ in 0..days {
        let len = data.len();
        for i in 0..len {
            data[i] -= 1;
            if data[i] < 0 {
                data[i] = 6;
                data.push(8);
            }
        }
    }
    data.len() as i32
}

fn part2(arg: &str, days: i32) -> u128 {
    let data = parse(arg);
    let mut array = [0; 9];
    for d in data {
        array[d as usize] += 1;
    }
    let mut carry_over = 0;
    for _ in 0..days {
        let temp = array[0];
        for i in 1..array.len() {
            array[i - 1] = array[i];
        }
        array[6] += temp;
        array[8] = carry_over;
        carry_over = array[0];
    }
    array.iter().sum()
}

fn main() {
    println!("{:?}", part1("./data.txt", 80));
    println!("{:?}", part2("./data.txt", 256));
}
