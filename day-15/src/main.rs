use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fs;

#[derive(Eq)]
struct Point {
    x: i32,
    y: i32,
    v: i32,
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        other.v.cmp(&self.v)
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.v == other.v
    }
}

fn parse(arg: &str) -> Vec<Vec<i32>> {
    let text: String = fs::read_to_string(arg).unwrap();
    text.split_terminator('\n')
        .map(|s| {
            s.chars()
                .map(|x| x.to_string().parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn parse2(arg: &str) -> Vec<Vec<i32>> {
    let data = parse(arg);
    let mut temp = vec![vec![0; data[0].len() * 5]; data.len() * 5];
    for cy in 0..5 {
        for (y, d) in data.iter().enumerate() {
            for cx in 0..5 {
                for (x, t) in d.iter().enumerate() {
                    let mut v = t + cy + cx;
                    if v > 9 {
                        v -= 9;
                    }
                    temp[y + data.len() * cy as usize][x + data[0].len() * cx as usize] = v;
                }
            }
        }
    }
    temp
}

fn get_adjacent_paths(x: i32, y: i32, max_x: i32, max_y: i32) -> Vec<(i32, i32)> {
    let ds = [(0, -1), (-1, 0), (0, 1), (1, 0)];
    let mut results = vec![];
    for d in ds {
        let nx = x + d.0;
        let ny = y + d.1;
        if nx >= 0 && nx < max_x && ny >= 0 && ny < max_y {
            results.push((nx, ny));
        }
    }
    results
}

fn part1(arg: &str) -> i32 {
    let data = parse(arg);
    let mut queue = BinaryHeap::new();
    let mut visited = HashMap::new();
    queue.push(Point { x: 0, y: 0, v: 0 });
    let mut result = std::i32::MAX;
    while let Some(p) = queue.pop() {
        if p.v > result {
            continue;
        }
        if p.x == data[0].len() as i32 - 1 && p.y == data.len() as i32 - 1 {
            result = result.min(p.v);
            continue;
        }
        visited.insert((p.x, p.y), p.v);
        for path in get_adjacent_paths(p.x, p.y, data[0].len() as i32, data.len() as i32) {
            let prev = visited.entry((path.0, path.1)).or_insert(std::i32::MAX);
            let d = (*prev).min(p.v + data[path.1 as usize][path.0 as usize]);
            if *prev > d {
                *prev = d;
                queue.push(Point {
                    x: path.0,
                    y: path.1,
                    v: d,
                });
            }
        }
    }
    result
}

fn part2(arg: &str) -> i32 {
    let data = parse2(arg);
    let mut queue = BinaryHeap::new();
    let mut visited = HashMap::new();
    queue.push(Point { x: 0, y: 0, v: 0 });
    let mut result = std::i32::MAX;
    while let Some(p) = queue.pop() {
        if p.v > result {
            continue;
        }
        if p.x == data[0].len() as i32 - 1 && p.y == data.len() as i32 - 1 {
            result = result.min(p.v);
            continue;
        }
        visited.insert((p.x, p.y), p.v);
        for path in get_adjacent_paths(p.x, p.y, data[0].len() as i32, data.len() as i32) {
            let prev = visited.entry((path.0, path.1)).or_insert(std::i32::MAX);
            let d = (*prev).min(p.v + data[path.1 as usize][path.0 as usize]);
            if *prev > d {
                *prev = d;
                queue.push(Point {
                    x: path.0,
                    y: path.1,
                    v: d,
                });
            }
        }
    }
    result
}

fn main() {
    println!("{:?}", part1("./data.txt"));
    println!("{:?}", part2("./data.txt"));
}
