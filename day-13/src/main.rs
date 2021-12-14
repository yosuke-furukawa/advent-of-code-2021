use std::fs;

fn parse(arg: &str) -> (Vec<Vec<i32>>, Vec<(String, i32)>) {
    let text: String = fs::read_to_string(arg).unwrap();
    let temp: Vec<&str> = text.split("\n\n").collect();
    let spots: Vec<(i32, i32)> = temp[0]
        .split_terminator('\n')
        .into_iter()
        .map(|s| {
            let p: Vec<&str> = s.split(',').into_iter().collect();
            (p[0].parse::<i32>().unwrap(), p[1].parse::<i32>().unwrap())
        })
        .collect();
    let max_x = spots.iter().map(|s| s.0).max().unwrap();
    let max_y = spots.iter().map(|s| s.1).max().unwrap();
    let mut maps = vec![vec![0; max_x as usize + 1]; max_y as usize + 1];
    for (x, y) in spots {
        maps[y as usize][x as usize] = 1;
    }

    let folds: Vec<(String, i32)> = temp[1]
        .split_terminator('\n')
        .into_iter()
        .map(|s| {
            let p: Vec<&str> = s.split(' ').into_iter().collect();
            let mut pos = p.last().unwrap().split('=');
            let direction = pos.next().unwrap().to_string();
            let position = pos.next().unwrap().parse::<i32>().unwrap();
            (direction, position)
        })
        .collect();

    (maps, folds)
}

fn part1(arg: &str) -> i32 {
    let (maps, folds) = parse(arg);

    let mut results = maps;
    for (direction, position) in folds.iter().take(1) {
        if direction == "y" {
            let mut new_results = vec![vec![0; results[0].len()]; *position as usize];
            for y in 0..*position as usize {
                for x in 0..results[y].len() {
                    new_results[y][x] = results[y][x];
                }
            }
            for y in (*position as usize + 1..results.len()).rev() {
                for x in 0..results[y].len() {
                    new_results[results.len() - y - 1][x] += results[y][x];
                }
            }
            results = new_results;
        } else {
            let mut new_results = vec![vec![0; *position as usize]; results.len()];
            for x in 0..*position as usize {
                for y in 0..results.len() {
                    new_results[y][x] = results[y][x];
                }
            }
            for x in (*position as usize + 1..results[0].len()).rev() {
                for y in 0..results.len() {
                    new_results[y][results[0].len() - x - 1] += results[y][x];
                }
            }
            results = new_results;
        }
    }
    results
        .into_iter()
        .map(|r| r.into_iter().filter(|s| *s > 0).count() as i32)
        .sum()
}

fn part2(arg: &str) {
    let (maps, folds) = parse(arg);

    let mut results = maps;
    for (direction, position) in folds.iter() {
        if direction == "y" {
            let mut new_results = vec![vec![0; results[0].len()]; *position as usize];
            for y in 0..*position as usize {
                for x in 0..results[y].len() {
                    new_results[y][x] = results[y][x];
                }
            }
            for y in (*position as usize + 1..results.len()).rev() {
                for x in 0..results[y].len() {
                    new_results[results.len() - y - 1][x] += results[y][x];
                }
            }
            results = new_results;
        } else {
            let mut new_results = vec![vec![0; *position as usize]; results.len()];
            for x in 0..*position as usize {
                for y in 0..results.len() {
                    new_results[y][x] = results[y][x];
                }
            }
            for x in (*position as usize + 1..results[0].len()).rev() {
                for y in 0..results.len() {
                    new_results[y][results[0].len() - x - 1] += results[y][x];
                }
            }
            results = new_results;
        }
    }

    for r in results {
        for q in r.iter() {
            if *q > 0 {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn main() {
    println!("{:?}", part1("./data.txt"));
    part2("./data.txt");
}
