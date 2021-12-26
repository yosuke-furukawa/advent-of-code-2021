use std::fs;

#[derive(Clone)]
struct Fish {
    // east => 1, south => 2
    // will east => -1, will south => -2
    direction: i32,
}

fn parse(arg: &str) -> Vec<Vec<Option<Fish>>> {
    let text = fs::read_to_string(arg).unwrap();
    let lines = text.split_terminator('\n');
    let mut results = vec![];
    for line in lines {
        let mut result = vec![];
        for c in line.chars() {
            result.push(match c {
                '.' => None,
                '>' => Some(Fish { direction: 1 }),
                'v' => Some(Fish { direction: 2 }),
                _ => panic!(),
            });
        }
        results.push(result);
    }
    results
}

fn get_adjacent(fish: &Fish, x: usize, y: usize, max_x: usize, max_y: usize) -> (usize, usize) {
    if fish.direction == 1 {
        if x < max_x - 1 {
            (x + 1, y)
        } else {
            (0, y)
        }
    } else if fish.direction == 2 {
        if y < max_y - 1 {
            (x, y + 1)
        } else {
            (x, 0)
        }
    } else {
        (x, y)
    }
}

fn debug(map: &Vec<Vec<Option<Fish>>>) {
    for m in map {
        for f in m {
            if let Some(fish) = f {
                if fish.direction == 1 {
                    print!(">");
                } else {
                    print!("v");
                }
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn part1(arg: &str) -> usize {
    let mut map = parse(arg);
    // debug(&map);
    let max_x = map[0].len();
    let max_y = map.len();
    let mut moved = true;
    let mut count = 0;
    while moved {
        moved = false;
        for y in 0..max_y {
            for x in 0..max_x {
                if let Some(fish) = &map[y][x] {
                    if fish.direction == 1 {
                        let (nx, ny) = get_adjacent(fish, x, y, max_x, max_y);
                        if map[ny][nx].is_none() {
                            map[y][x] = Some(Fish { direction: 0 });
                            map[ny][nx] = Some(Fish { direction: -1 });
                            moved = true;
                        }
                    }
                }
            }
        }
        for y in 0..max_y {
            for x in 0..max_x {
                if let Some(fish) = &map[y][x] {
                    if fish.direction < 0 {
                        map[y][x] = Some(Fish {
                            direction: fish.direction * -1,
                        });
                    } else {
                        if fish.direction == 0 {
                            map[y][x] = None;
                        }
                    }
                }
            }
        }
        for y in 0..max_y {
            for x in 0..max_x {
                if let Some(fish) = &map[y][x] {
                    if fish.direction == 2 {
                        let (nx, ny) = get_adjacent(fish, x, y, max_x, max_y);
                        if map[ny][nx].is_none() {
                            map[y][x] = Some(Fish { direction: 0 });
                            map[ny][nx] = Some(Fish { direction: -2 });
                            moved = true;
                        }
                    }
                }
            }
        }
        for y in 0..max_y {
            for x in 0..max_x {
                if let Some(fish) = &map[y][x] {
                    if fish.direction < 0 {
                        map[y][x] = Some(Fish {
                            direction: fish.direction * -1,
                        });
                    } else {
                        if fish.direction == 0 {
                            map[y][x] = None;
                        }
                    }
                }
            }
        }

        println!("step = {}", count);
        // debug(&map);
        count += 1;
    }
    count
}

fn main() {
    println!("{}", part1("./data.txt"));
}
