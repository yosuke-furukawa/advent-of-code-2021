use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;

fn parse(arg: &str) -> Vec<Vec<i32>> {
    let text: String = fs::read_to_string(arg).unwrap();
    let data: Vec<&str> = text.split_terminator('\n').into_iter().collect();
    let mut results = vec![];
    for d in data {
        results.push(
            d.chars()
                .map(|x| x.to_string().parse::<i32>().unwrap())
                .collect(),
        );
    }
    results
}

fn part1(arg: &str) -> i32 {
    let data = parse(arg);
    let mut result = 0;
    for y in 0..data.len() {
        for x in 0..data[y].len() {
            let num = data[y][x];
            let mut west = 10;
            if x > 0 {
                west = data[y][x - 1];
            }
            let mut north = 10;
            if y > 0 {
                north = data[y - 1][x];
            }
            let mut east = 10;
            if x < data[y].len() - 1 {
                east = data[y][x + 1];
            }
            let mut south = 10;
            if y < data.len() - 1 {
                south = data[y + 1][x];
            }

            if west > num && north > num && east > num && south > num {
                result += num + 1;
            }
        }
    }
    result
}

// fn count(x: usize, y: usize, data: &[Vec<i32>], area_count: &mut i32, visited: &mut HashSet<(usize, usize)>) {
//     let num = data[y][x];
//     visited.insert((x, y));
//     if x > 0 && num < 8 && data[y][x-1] == num + 1 && !visited.contains(&(x - 1, y)) {
//         *area_count += 1;
//         count(x-1, y, data, area_count, visited);
//     }
//     if y > 0 && num < 8 && data[y-1][x] == num + 1 && !visited.contains(&(x, y - 1)) {
//         *area_count += 1;
//         count(x, y-1, data, area_count, visited);

//     }
//     if x < data[0].len() - 1 && num < 8 && data[y][x+1] == num + 1 && !visited.contains(&(x + 1, y)) {
//         *area_count += 1;
//         count(x+1, y, data, area_count, visited);
//     }
//     if y < data.len() - 1 && num < 8 && data[y+1][x] == num + 1 && !visited.contains(&(x, y + 1)) {
//         *area_count += 1;
//         count(x, y+1, data, area_count, visited);
//     }
// }

fn get_adjacent_points(x: usize, y: usize, max_x: usize, max_y: usize) -> Vec<(usize, usize)> {
    let mut v = vec![];
    if x > 0 {
        v.push((x - 1, y));
    }
    if y > 0 {
        v.push((x, y - 1));
    }
    if x < max_x - 1 {
        v.push((x + 1, y));
    }
    if y < max_y - 1 {
        v.push((x, y + 1));
    }
    v
}

fn get_positions(data: &[Vec<i32>]) -> Vec<(usize, usize)> {
    let mut positions = vec![];
    for y in 0..data.len() {
        for x in 0..data[y].len() {
            let num = data[y][x];
            let mut west = 10;
            if x > 0 {
                west = data[y][x - 1];
            }
            let mut north = 10;
            if y > 0 {
                north = data[y - 1][x];
            }
            let mut east = 10;
            if x < data[y].len() - 1 {
                east = data[y][x + 1];
            }
            let mut south = 10;
            if y < data.len() - 1 {
                south = data[y + 1][x];
            }

            if west > num && north > num && east > num && south > num {
                positions.push((x, y));
            }
        }
    }
    positions
}

fn get_basin_size(x: usize, y: usize, data: &[Vec<i32>]) -> i32 {
    let mut queue = VecDeque::from(vec![(x, y)]);
    let mut visited = HashSet::new();
    visited.insert((x, y));
    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        for pos in get_adjacent_points(current.0, current.1, data[0].len(), data.len()) {
            if data[pos.1][pos.0] > data[current.1][current.0]
                && data[pos.1][pos.0] < 9
                && !visited.contains(&pos)
            {
                queue.push_back(pos);
                visited.insert(pos);
            }
        }
    }
    visited.len() as i32
}

fn part2(arg: &str) -> i32 {
    let data = parse(arg);
    let positions = get_positions(&data);
    let mut result = vec![];
    for position in positions {
        result.push(get_basin_size(position.0, position.1, &data));
    }
    result.sort_unstable_by(|a, b| b.cmp(a));
    result[0] * result[1] * result[2]
}

fn main() {
    println!("{:?}", part1("./data.txt"));
    println!("{:?}", part2("./data.txt"));
}
