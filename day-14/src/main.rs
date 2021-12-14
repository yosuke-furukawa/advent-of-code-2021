use std::collections::HashMap;
use std::fs;

fn parse(arg: &str) -> (Vec<char>, HashMap<String, char>) {
    let text: String = fs::read_to_string(arg).unwrap();
    let temp: Vec<&str> = text.split("\n\n").collect();
    let initial = temp[0].chars().collect();
    let mut map = HashMap::new();
    for l in temp[1].split('\n') {
        let temp: Vec<&str> = l.split(" -> ").collect();
        map.insert(temp[0].to_string(), temp[1].chars().last().unwrap());
    }
    (initial, map)
}

fn part1(arg: &str, step: i32) -> i32 {
    let (chars, map) = parse(arg);

    let mut chars = chars;
    for _ in 0..step {
        let mut new_chars = vec![];
        for ch in chars.windows(2) {
            let key = ch
                .iter()
                .fold(String::new(), |acc, cur| acc + &cur.to_string());
            let mid = *map.get(&key).unwrap();
            if new_chars.is_empty() {
                new_chars.push(ch[0]);
            }
            new_chars.push(mid);
            new_chars.push(ch[1]);
        }
        chars = new_chars;
    }
    let mut count_map = HashMap::new();
    for c in chars.iter() {
        *count_map.entry(c).or_insert(0) += 1;
    }
    let mut max = 0;
    let mut min = 1_000_000_000;
    for entry in count_map {
        max = max.max(entry.1);
        min = min.min(entry.1);
    }
    max - min
}

fn part2(arg: &str, step: i32) -> u64 {
    let (chars, map) = parse(arg);
    let mut count_map: HashMap<String, u64> = HashMap::new();
    let mut word_map: HashMap<char, u64> = HashMap::new();
    for ch in chars.windows(2) {
        *word_map.entry(ch[0]).or_insert(0) += 1;
        let key = ch
            .iter()
            .fold(String::new(), |acc, cur| acc + &cur.to_string());
        *count_map.entry(key).or_insert(0) += 1;
    }
    *word_map.entry(*chars.last().unwrap()).or_insert(0) += 1;
    for _ in 0..step {
        let mut m = HashMap::new();
        for entry in count_map {
            let chars: Vec<char> = entry.0.chars().collect();
            let mid = *map.get(&entry.0).unwrap();
            let key1 = chars[0].to_string() + &mid.to_string();
            let key2 = mid.to_string() + &chars[1].to_string();
            *m.entry(key1).or_insert(0) += entry.1;
            *m.entry(key2).or_insert(0) += entry.1;
            *word_map.entry(mid).or_insert(0) += entry.1;
        }
        count_map = m;
    }

    let mut max = 0;
    let mut min = 1_000_000_000_000;
    for entry in word_map {
        max = max.max(entry.1);
        min = min.min(entry.1);
    }
    max - min
}

fn main() {
    println!("{:?}", part1("./data.txt", 10));
    println!("{:?}", part2("./data.txt", 40));
}
