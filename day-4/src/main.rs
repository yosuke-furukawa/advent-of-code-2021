use std::collections::HashSet;
use std::fs;

fn parse(arg: &str) -> (Vec<i32>, Vec<Vec<Vec<(i32, bool)>>>) {
    let text: String = fs::read_to_string(arg).unwrap();
    let data: Vec<&str> = text.split_terminator('\n').into_iter().collect();
    let numbers: Vec<i32> = data[0]
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    let mut bingos: Vec<Vec<Vec<(i32, bool)>>> = vec![];
    let mut pos = 1;
    while pos < data.len() {
        let line = data[pos];
        if line.is_empty() {
            pos += 1;
            continue;
        }
        let mut bingo = vec![];
        let mut line_num = 0;
        while line_num < 5 {
            let line = data[pos + line_num];
            let bingo_line: Vec<(i32, bool)> = line
                .split_ascii_whitespace()
                .into_iter()
                .map(|x| (x.parse::<i32>().unwrap(), false))
                .collect();
            bingo.push(bingo_line);
            line_num += 1;
        }
        bingos.push(bingo);
        pos += line_num;
    }
    (numbers, bingos)
}

fn check(bingo: &[Vec<(i32, bool)>]) -> bool {
    // check horizontal
    for bi in bingo {
        if bi.iter().all(|n| n.1) {
            return true;
        }
    }

    // check vertical
    for yi in 0..bingo[0].len() {
        let mut marked_count = 0;
        for xi in 0..bingo.len() {
            if bingo[xi][yi].1 {
                marked_count += 1;
            } else {
                break;
            }
        }
        if marked_count == 5 {
            return true;
        }
    }

    // check diagonal
    for (xi, yi) in [(0, 0), (1, 1), (2, 2), (3, 3), (4, 4)] {
        let mut marked_count = 0;
        if bingo[xi][yi].1 {
            marked_count += 1;
        } else {
            break;
        }
        if marked_count == 5 {
            return true;
        }
    }
    for (xi, yi) in [(0, 4), (1, 3), (2, 2), (3, 1), (4, 0)] {
        let mut marked_count = 0;
        if bingo[xi][yi].1 {
            marked_count += 1;
        } else {
            break;
        }
        if marked_count == 5 {
            return true;
        }
    }

    false
}

fn part1(arg: &str) -> i32 {
    let (numbers, bingos) = parse(arg);
    let mut bingos = bingos;
    for num in numbers.iter() {
        for bingo in bingos.iter_mut() {
            let mut found = false;
            for b in bingo.iter_mut() {
                for (n, bool) in b.iter_mut() {
                    if n == num {
                        *bool = true;
                        found = true;
                    }
                    if found {
                        break;
                    }
                }
                if found {
                    break;
                }
            }
            if found && check(bingo) {
                println!("{:?}", bingo);
                let mut score = 0;
                for b in bingo {
                    for n in b {
                        if n.1 {
                            continue;
                        }
                        score += n.0;
                    }
                }
                return score * num;
            }
        }
    }
    -1
}

fn part2(arg: &str) -> i32 {
    let (numbers, bingos) = parse(arg);
    let mut bingos = bingos;
    let bingos_len = bingos.len();
    let mut finished = HashSet::new();
    for num in numbers.iter() {
        for (i, bingo) in bingos.iter_mut().enumerate() {
            if finished.contains(&i) {
                continue;
            }
            let mut found = false;
            for b in bingo.iter_mut() {
                for (n, bool) in b.iter_mut() {
                    if n == num {
                        *bool = true;
                        found = true;
                    }
                    if found {
                        break;
                    }
                }
                if found {
                    break;
                }
            }
            if found && check(bingo) {
                let mut score = 0;
                for b in bingo {
                    for n in b {
                        if n.1 {
                            continue;
                        }
                        score += n.0;
                    }
                }
                finished.insert(i);
                if finished.len() == bingos_len {
                    return score * num;
                }
            }
        }
    }
    -1
}

fn main() {
    println!("{}", part1("./data.txt"));
    println!("{}", part2("./data.txt"));
}
