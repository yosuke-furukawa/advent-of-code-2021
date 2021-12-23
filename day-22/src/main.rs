use regex::Regex;
use std::collections::HashSet;
use std::fs;

#[derive(Debug, Clone, Default)]
struct Button {
    operation: String,
    x1: i64,
    x2: i64,
    y1: i64,
    y2: i64,
    z1: i64,
    z2: i64,
}

impl Button {
    fn area(&self) -> i64 {
        (self.x2 - self.x1 + 1) * (self.y2 - self.y1 + 1) * (self.z2 - self.z1 + 1)
    }
}

fn parse(arg: &str) -> Vec<Button> {
    let text: String = fs::read_to_string(arg).unwrap();
    let lines: Vec<&str> = text.split_terminator('\n').collect();
    let mut result = vec![];
    for line in lines {
        let re = Regex::new(r"^(on|off) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)$")
            .unwrap();
        let caps = re.captures(line).unwrap();
        let op = caps.get(1).map_or("", |m| m.as_str());
        let x1 = caps
            .get(2)
            .map_or("", |m| m.as_str())
            .parse::<i64>()
            .unwrap();
        let x2 = caps
            .get(3)
            .map_or("", |m| m.as_str())
            .parse::<i64>()
            .unwrap();
        let y1 = caps
            .get(4)
            .map_or("", |m| m.as_str())
            .parse::<i64>()
            .unwrap();
        let y2 = caps
            .get(5)
            .map_or("", |m| m.as_str())
            .parse::<i64>()
            .unwrap();
        let z1 = caps
            .get(6)
            .map_or("", |m| m.as_str())
            .parse::<i64>()
            .unwrap();
        let z2 = caps
            .get(7)
            .map_or("", |m| m.as_str())
            .parse::<i64>()
            .unwrap();
        let button = Button {
            operation: op.to_string(),
            x1,
            x2,
            y1,
            y2,
            z1,
            z2,
        };
        result.push(button);
    }
    result
}

fn part1(arg: &str) -> usize {
    let buttons = parse(arg);
    let mut set = HashSet::new();

    for button in buttons.iter() {
        for x in button.x1..=button.x2 {
            if x < -50 || x > 50 {
                continue;
            }
            for y in button.y1..=button.y2 {
                if y < -50 || y > 50 {
                    continue;
                }
                for z in button.z1..=button.z2 {
                    if z < -50 || z > 50 {
                        continue;
                    }
                    if !set.contains(&(button.operation.clone(), x, y, z)) {
                        if button.operation == "on" {
                            set.remove(&("off".to_string(), x, y, z));
                        } else {
                            set.remove(&("on".to_string(), x, y, z));
                        }
                        set.insert((button.operation.clone(), x, y, z));
                    }
                }
            }
        }
    }
    set.into_iter().filter(|b| b.0 == "on").count()
}

fn overlap(origin: Button, buttons: &[Button]) -> i64 {
    buttons
        .iter()
        .enumerate()
        .map(|(i, other)| {
            let min_x = origin.x1.max(other.x1);
            let max_x = origin.x2.min(other.x2);
            let min_y = origin.y1.max(other.y1);
            let max_y = origin.y2.min(other.y2);
            let min_z = origin.z1.max(other.z1);
            let max_z = origin.z2.min(other.z2);
            if max_x - min_x >= 0 && max_y - min_y >= 0 && max_z - min_z >= 0 {
                let new_button = Button {
                    operation: "any".to_string(),
                    x1: min_x,
                    x2: max_x,
                    y1: min_y,
                    y2: max_y,
                    z1: min_z,
                    z2: max_z,
                };
                new_button.area() - overlap(new_button, &buttons[i + 1..])
            } else {
                0
            }
        })
        .sum()
}

fn part2(arg: &str) -> i64 {
    let buttons = parse(arg);
    let mut bs = vec![];
    let mut result = 0;
    for button in buttons.into_iter().rev() {
        if button.operation == "on" {
            result += button.area() - overlap(button.clone(), &bs);
        }
        bs.push(button);
    }
    result
}

fn main() {
    println!("{}", part1("./data.txt"));
    println!("{}", part2("./data.txt"));
}
