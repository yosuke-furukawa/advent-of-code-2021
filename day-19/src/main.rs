use std::fs;

use std::ops::Add;
use std::ops::Sub;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

fn parse(arg: &str) -> Vec<Vec<Point>> {
    let text: String = fs::read_to_string(arg).unwrap();
    let temp: Vec<Vec<&str>> = text
        .split_terminator("\n\n")
        .map(|lines| lines.split_terminator('\n').skip(1).collect())
        .collect();
    let mut results = vec![];
    for te in temp {
        let mut result = vec![];
        for t in te {
            let nums: Vec<i32> = t
                .split_terminator(',')
                .map(|n| n.parse::<i32>().unwrap())
                .collect();
            result.push(Point {
                x: nums[0],
                y: nums[1],
                z: nums[2],
            });
        }
        results.push(result);
    }
    results
}

fn get_rotation(v: Point, index: usize) -> Point {
    match index {
        0 => Point {
            x: v.z,
            y: v.y,
            z: -v.x,
        },
        1 => Point {
            x: -v.z,
            y: -v.y,
            z: -v.x,
        },
        2 => Point {
            x: -v.z,
            y: -v.x,
            z: v.y,
        },
        3 => Point {
            x: -v.z,
            y: v.x,
            z: -v.y,
        },
        4 => Point {
            x: -v.z,
            y: v.y,
            z: v.x,
        },
        5 => Point {
            x: -v.y,
            y: -v.z,
            z: v.x,
        },
        6 => Point {
            x: -v.y,
            y: -v.x,
            z: -v.z,
        },
        7 => Point {
            x: -v.y,
            y: v.x,
            z: v.z,
        },
        8 => Point {
            x: -v.y,
            y: v.z,
            z: -v.x,
        },
        9 => Point {
            x: -v.x,
            y: -v.z,
            z: -v.y,
        },
        10 => Point {
            x: -v.x,
            y: -v.y,
            z: v.z,
        },
        11 => Point {
            x: -v.x,
            y: v.y,
            z: -v.z,
        },
        12 => Point {
            x: -v.x,
            y: v.z,
            z: v.y,
        },
        13 => Point {
            x: v.x,
            y: -v.z,
            z: v.y,
        },
        14 => Point {
            x: v.x,
            y: -v.y,
            z: -v.z,
        },
        15 => Point {
            x: v.x,
            y: v.y,
            z: v.z,
        },
        16 => Point {
            x: v.x,
            y: v.z,
            z: -v.y,
        },
        17 => Point {
            x: v.y,
            y: -v.z,
            z: -v.x,
        },
        18 => Point {
            x: v.y,
            y: -v.x,
            z: v.z,
        },
        19 => Point {
            x: v.y,
            y: v.x,
            z: -v.z,
        },
        20 => Point {
            x: v.y,
            y: v.z,
            z: v.x,
        },
        21 => Point {
            x: v.z,
            y: -v.y,
            z: v.x,
        },
        22 => Point {
            x: v.z,
            y: -v.x,
            z: -v.y,
        },
        23 => Point {
            x: v.z,
            y: v.x,
            z: v.y,
        },
        _ => panic!("no such rotation"),
    }
}

fn orient_points(points: &[Point], orientation: usize, offset: Point) -> Vec<Point> {
    let mut result = Vec::new();
    for p in points {
        result.push(offset + get_rotation(*p, orientation));
    }
    result
}

fn orient_beacons(scanners: &mut [Vec<Point>]) -> (usize, Vec<Point>) {
    let mut offsets = Vec::new();
    offsets.resize(scanners.len(), Point { x: 0, y: 0, z: 0 });
    let mut orientation = Vec::new();
    orientation.resize(scanners.len(), 0);
    let mut is_oriented = Vec::new();
    is_oriented.resize(scanners.len(), false);
    is_oriented[0] = true;

    loop {
        let mut done = true;
        for i in 0..scanners.len() - 1 {
            for j in i + 1..scanners.len() {
                if is_oriented[i] == is_oriented[j] {
                    continue;
                }
                done = false;
                let idx_a = if is_oriented[i] { i } else { j };
                let idx_b = if is_oriented[i] { j } else { i };
                let overlaps = find_overlaps(&scanners[idx_a], &scanners[idx_b]);

                if let Some((offset, orientation)) = overlaps {
                    offsets[idx_b] = offset;
                    scanners[idx_b] = orient_points(&scanners[idx_b], orientation, offset);
                    is_oriented[idx_b] = true;
                }
            }
        }
        if done {
            break;
        }
    }
    let mut beacons = Vec::new();
    for scanner in scanners {
        for point in scanner {
            beacons.push(point);
        }
    }
    beacons.sort_unstable();
    beacons.dedup();

    (beacons.len(), offsets)
}

fn overlaps(a: &[Point], b: &[Point], offset: Point, orientation: usize) -> bool {
    let mut num_hits = 0;
    for point_a in a {
        for point_b in b {
            let b_adjusted = offset + get_rotation(*point_b, orientation);
            if *point_a == b_adjusted {
                num_hits += 1;
                if num_hits >= 12 {
                    return true;
                }
                break;
            }
        }
    }

    false
}

fn find_overlaps(a: &[Point], b: &[Point]) -> Option<(Point, usize)> {
    for i in 0..a.len() {
        let point_a = a[i];
        for j in 0..b.len() {
            let point_b = b[j];
            for orientation in 0..24 {
                let offset = point_a - get_rotation(point_b, orientation);
                if overlaps(a, b, offset, orientation) {
                    return Some((offset, orientation));
                }
            }
        }
    }

    None
}

fn part1(arg: &str) -> usize {
    let mut scanners = parse(arg);
    let (len, _) = orient_beacons(&mut scanners);
    len
}

fn part2(arg: &str) -> i32 {
    let mut scanners = parse(arg);
    let (_, positions) = orient_beacons(&mut scanners);
    let mut max = 0;
    for i in 0..positions.len() {
        let p0 = positions[i];
        for j in i + 1..positions.len() {
            let p1 = positions[j];
            let manhattan_distance =
                (p0.x - p1.x).abs() + (p0.y - p1.y).abs() + (p0.z - p1.z).abs();
            max = max.max(manhattan_distance);
        }
    }
    max
}

fn main() {
    println!("{:?}", part1("./data.txt"));
    println!("{:?}", part2("./data.txt"));
}
