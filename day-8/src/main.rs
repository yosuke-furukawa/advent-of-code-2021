use std::collections::HashMap;
use std::fs;

fn parse(arg: &str) -> (Vec<Vec<String>>, Vec<Vec<String>>) {
    let text: String = fs::read_to_string(arg).unwrap();
    let data: Vec<&str> = text.split_terminator('\n').into_iter().collect();
    let mut left = vec![];
    let mut right = vec![];
    for d in data {
        let temp: Vec<&str> = d.split('|').into_iter().collect();
        let l: Vec<String> = temp[0]
            .trim()
            .split(' ')
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        let r: Vec<String> = temp[1]
            .trim()
            .split(' ')
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        left.push(l);
        right.push(r);
    }

    (left, right)
}

fn part1(arg: &str) -> i32 {
    let (_, right) = parse(arg);
    let mut count = 0;
    for line in right {
        for text in line {
            count += match text.len() {
                2 | 3 | 4 | 7 => 1,
                _ => 0,
            };
        }
    }
    count
}

fn part2(arg: &str) -> i32 {
    let (left, right) = parse(arg);
    let mut result = 0;

    let len = left.len();
    for i in 0..len {
        let l = left[i].clone();
        let mut chars = ['-'; 7];
        let mut word_map = HashMap::new();
        let mut count_map = HashMap::new();
        for word in l.iter() {
            for c in word.chars() {
                *word_map.entry(c).or_insert(0) += 1;
            }
            count_map.insert(word.len(), word);
        }
        for entry in word_map.iter() {
            if entry.1 == &9 {
                chars[5] = *entry.0;
                let ch: Vec<char> = (*count_map.get(&2).unwrap()).chars().collect();
                chars[2] = ch.into_iter().filter(|c| *c != *entry.0).last().unwrap();
                for e in word_map.iter() {
                    if e.1 == &8 && e.0 != &chars[2] {
                        chars[0] = *e.0;
                    }
                }
            }
            if entry.1 == &4 {
                chars[4] = *entry.0;
            }
            if entry.1 == &6 {
                chars[1] = *entry.0;
            }
        }
        let ch: Vec<char> = count_map.get(&4).unwrap().chars().collect();
        chars[3] = ch
            .into_iter()
            .filter(|c| *c != chars[1] && *c != chars[2] && *c != chars[5])
            .last()
            .unwrap();
        for e in word_map.iter() {
            if e.1 == &7 && e.0 != &chars[3] {
                chars[6] = *e.0;
            }
        }
        let mut dict: HashMap<String, i32> = HashMap::new();
        let mut key = vec![chars[0], chars[1], chars[2], chars[4], chars[5], chars[6]];
        key.sort_unstable();
        dict.insert(
            key.iter()
                .fold(String::new(), |acc, cur| acc + &cur.to_string()),
            0,
        );
        key = vec![chars[2], chars[5]];
        key.sort_unstable();
        dict.insert(
            key.iter()
                .fold(String::new(), |acc, cur| acc + &cur.to_string()),
            1,
        );
        key = vec![chars[0], chars[2], chars[3], chars[4], chars[6]];
        key.sort_unstable();
        dict.insert(
            key.iter()
                .fold(String::new(), |acc, cur| acc + &cur.to_string()),
            2,
        );
        key = vec![chars[0], chars[2], chars[3], chars[5], chars[6]];
        key.sort_unstable();
        dict.insert(
            key.iter()
                .fold(String::new(), |acc, cur| acc + &cur.to_string()),
            3,
        );
        key = vec![chars[1], chars[2], chars[3], chars[5]];
        key.sort_unstable();
        dict.insert(
            key.iter()
                .fold(String::new(), |acc, cur| acc + &cur.to_string()),
            4,
        );
        key = vec![chars[0], chars[1], chars[3], chars[5], chars[6]];
        key.sort_unstable();
        dict.insert(
            key.iter()
                .fold(String::new(), |acc, cur| acc + &cur.to_string()),
            5,
        );
        key = vec![chars[0], chars[1], chars[3], chars[4], chars[5], chars[6]];
        key.sort_unstable();
        dict.insert(
            key.iter()
                .fold(String::new(), |acc, cur| acc + &cur.to_string()),
            6,
        );
        key = vec![chars[0], chars[2], chars[5]];
        key.sort_unstable();
        dict.insert(
            key.iter()
                .fold(String::new(), |acc, cur| acc + &cur.to_string()),
            7,
        );
        key = vec![
            chars[0], chars[1], chars[2], chars[3], chars[4], chars[5], chars[6],
        ];
        key.sort_unstable();
        dict.insert(
            key.iter()
                .fold(String::new(), |acc, cur| acc + &cur.to_string()),
            8,
        );
        key = vec![chars[0], chars[1], chars[2], chars[3], chars[5], chars[6]];
        key.sort_unstable();
        dict.insert(
            key.iter()
                .fold(String::new(), |acc, cur| acc + &cur.to_string()),
            9,
        );

        let mut right_value = 0;
        let r = right[i].clone();
        // println!("{:?}", dict);

        for w in r {
            let mut key: Vec<char> = w.chars().collect();
            key.sort_unstable();
            // println!("{}", key.iter().fold(String::new(), |acc, cur| acc + &cur.to_string()));
            right_value *= 10;
            right_value += dict
                .get(
                    &key.iter()
                        .fold(String::new(), |acc, cur| acc + &cur.to_string()),
                )
                .unwrap();
        }
        // println!("{}", result);
        result += right_value;
    }

    result
}

fn main() {
    println!("{:?}", part1("./data.txt"));
    println!("{:?}", part2("./data.txt"));
}
