use std::fs;

fn parse(arg: &str) -> Vec<(i32, i32, i32, i32)> {
    let text: String = fs::read_to_string(arg).unwrap();
    let data: Vec<&str> = text.split_terminator('\n').into_iter().collect();
    let mut results = vec![];
    for d in data {
        let nums: Vec<&str> = d.split(" -> ").into_iter().collect();
        let x1y1: Vec<&str> = nums[0].split(',').into_iter().collect();
        let x2y2: Vec<&str> = nums[1].split(',').into_iter().collect();
        results.push((
            x1y1[0].parse::<i32>().unwrap(),
            x1y1[1].parse::<i32>().unwrap(),
            x2y2[0].parse::<i32>().unwrap(),
            x2y2[1].parse::<i32>().unwrap(),
        ));
    }
    results
}

fn part1(arg: &str) -> i32 {
    let data = parse(arg);
    let mut max_x = 0;
    let mut max_y = 0;
    for d in data.iter() {
        max_x = max_x.max(d.0).max(d.2);
        max_y = max_y.max(d.1).max(d.3);
    }
    let mut matrix = vec![vec![0; max_x as usize + 1]; max_y as usize + 1];
    let mut count = 0;
    for d in data.iter() {
        if d.0 == d.2 || d.1 == d.3 {
            let x1 = d.0.min(d.2);
            let x2 = d.0.max(d.2);
            let y1 = d.1.min(d.3);
            let y2 = d.1.max(d.3);
            for x in x1..=x2 {
                for y in y1..=y2 {
                    matrix[y as usize][x as usize] += 1;
                    if matrix[y as usize][x as usize] == 2 {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

fn part2(arg: &str) -> i32 {
    let data = parse(arg);
    let mut max_x = 0;
    let mut max_y = 0;
    for d in data.iter() {
        max_x = max_x.max(d.0).max(d.2);
        max_y = max_y.max(d.1).max(d.3);
    }
    let mut matrix = vec![vec![0; max_x as usize + 1]; max_y as usize + 1];
    let mut count = 0;
    for d in data.iter() {
        let x1 = d.0.min(d.2);
        let x2 = d.0.max(d.2);
        let y1 = d.1.min(d.3);
        let y2 = d.1.max(d.3);
        if d.0 == d.2 || d.1 == d.3 {
            for y in y1..=y2 {
                for x in x1..=x2 {
                    matrix[y as usize][x as usize] += 1;
                    if matrix[y as usize][x as usize] == 2 {
                        count += 1;
                    }
                }
            }
        } else if d.0.max(d.2) - d.0.min(d.2) == d.1.max(d.3) - d.1.min(d.3) {
            if (d.0 < d.2 && d.1 < d.3) || (d.2 < d.0 && d.3 < d.1) {
                let mut x = x1;
                let mut y = y1;
                while x <= x2 && y <= y2 {
                    matrix[y as usize][x as usize] += 1;
                    if matrix[y as usize][x as usize] == 2 {
                        count += 1;
                    }
                    x += 1;
                    y += 1;
                }
            } else {
                let mut x = x1;
                let mut y = y2;
                while x <= x2 && y >= y1 {
                    matrix[y as usize][x as usize] += 1;
                    if matrix[y as usize][x as usize] == 2 {
                        count += 1;
                    }
                    x += 1;
                    y -= 1;
                }
            }
        }
    }
    count
}

fn main() {
    println!("{:?}", part1("./data.txt"));
    println!("{:?}", part2("./data.txt"));
}
