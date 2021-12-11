use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;

fn parse(arg: &str) -> Vec<Vec<i32>> {
    let text: String = fs::read_to_string(arg).unwrap();
    text.split_terminator('\n')
        .into_iter()
        .map(|s| {
            s.chars()
                .map(|c| c.to_string().parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn get_adjacent_octopus(x: i32, y: i32, max_x: i32, max_y: i32) -> Vec<(i32, i32)> {
    let dxs = [-1, 0, 1];
    let dys = [-1, 0, 1];
    let mut results = vec![];
    for dy in dys {
        for dx in dxs {
            let nx = x + dx;
            let ny = y + dy;
            if nx == x && ny == y {
                continue;
            }
            if nx >= 0 && nx < max_x && ny >= 0 && ny < max_y {
                results.push((nx, ny));
            }
        }
    }
    results
}

fn part1(arg: &str, step: usize) -> i32 {
    let mut octopuses = parse(arg);
    let max_x = octopuses[0].len();
    let max_y = octopuses.len();
    let mut result = 0;
    for _ in 0..step {
        let mut marked = VecDeque::new();
        let mut visited = HashSet::new();
        for y in 0..octopuses.len() {
            for x in 0..octopuses[y].len() {
                octopuses[y][x] += 1;
                if octopuses[y][x] > 9 {
                    marked.push_back((x, y));
                    visited.insert((x, y));
                }
            }
        }

        while let Some((x, y)) = marked.pop_front() {
            octopuses[y][x] = 0;
            for (nx, ny) in get_adjacent_octopus(x as i32, y as i32, max_x as i32, max_y as i32) {
                if visited.contains(&(nx as usize, ny as usize)) {
                    continue;
                }
                if octopuses[ny as usize][nx as usize] > 0 {
                    octopuses[ny as usize][nx as usize] += 1;
                }
                if octopuses[ny as usize][nx as usize] > 9 {
                    marked.push_back((nx as usize, ny as usize));
                    visited.insert((nx as usize, ny as usize));
                }
            }
        }
        result += visited.len() as usize;
    }
    result as i32
}

fn part2(arg: &str) -> i32 {
    let mut octopuses = parse(arg);
    let max_x = octopuses[0].len();
    let max_y = octopuses.len();
    let mut step = 1;
    loop {
        let mut marked = VecDeque::new();
        let mut visited = HashSet::new();
        for y in 0..octopuses.len() {
            for x in 0..octopuses[y].len() {
                octopuses[y][x] += 1;
                if octopuses[y][x] > 9 {
                    marked.push_back((x, y));
                    visited.insert((x, y));
                }
            }
        }

        while let Some((x, y)) = marked.pop_front() {
            octopuses[y][x] = 0;
            for (nx, ny) in get_adjacent_octopus(x as i32, y as i32, max_x as i32, max_y as i32) {
                if visited.contains(&(nx as usize, ny as usize)) {
                    continue;
                }
                if octopuses[ny as usize][nx as usize] > 0 {
                    octopuses[ny as usize][nx as usize] += 1;
                }
                if octopuses[ny as usize][nx as usize] > 9 {
                    marked.push_back((nx as usize, ny as usize));
                    visited.insert((nx as usize, ny as usize));
                }
            }
        }
        if visited.len() == max_x * max_y {
            println!("{}", step);
            for o in octopuses.iter() {
                println!("{:?}", o);
            }
            break;
        }
        step += 1;
    }
    step
}

fn main() {
    println!("{:?}", part1("./data.txt", 100));
    println!("{:?}", part2("./data.txt"));
}
