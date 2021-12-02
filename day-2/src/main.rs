use std::fs;

fn part1() {
    let text: String = fs::read_to_string("./data.txt").unwrap();
    let data: Vec<(&str, i32)> = text
        .split_terminator('\n')
        .into_iter()
        .map(|x| {
            let line: Vec<&str> = x.split(' ').collect();
            let (op, x) = (line[0], line[1].parse::<i32>().unwrap());
            (op, x)
        })
        .collect();
    let mut x = 0;
    let mut y = 0;
    for operation in data.iter() {
        match operation.0 {
            "forward" => x += operation.1,
            "up" => y -= operation.1,
            "down" => y += operation.1,
            _ => panic!("no such op error"),
        }
    }
    println!("x={}, y={}", x, y);
    println!("{}", x * y);
}

fn part2() {
    let text: String = fs::read_to_string("./data.txt").unwrap();
    let data: Vec<(&str, i32)> = text
        .split_terminator('\n')
        .into_iter()
        .map(|x| {
            let line: Vec<&str> = x.split(' ').collect();
            let (op, x) = (line[0], line[1].parse::<i32>().unwrap());
            (op, x)
        })
        .collect();
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;
    for operation in data.iter() {
        match operation.0 {
            "forward" => {
                x += operation.1;
                y += aim * operation.1;
            }
            "up" => aim -= operation.1,
            "down" => aim += operation.1,
            _ => panic!("no such op error"),
        }
    }
    println!("x={}, y={}", x, y);
    println!("{}", x * y);
}

fn main() {
    part1();
    part2();
}
