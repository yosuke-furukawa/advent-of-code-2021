use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;

fn parse(
    arg: &str,
) -> (
    Vec<Vec<bool>>,
    HashMap<String, usize>,
    HashMap<usize, String>,
) {
    let mut map1 = HashMap::new();
    let mut map2 = HashMap::new();
    let mut set = HashSet::new();
    let text: String = fs::read_to_string(arg).unwrap();
    let temp: Vec<(&str, &str)> = text
        .split_terminator('\n')
        .into_iter()
        .map(|s| {
            let path: Vec<&str> = s.split('-').collect();
            let start = path[0];
            let end = path[1];
            set.insert(start);
            set.insert(end);
            (start, end)
        })
        .collect();

    let mut vec = vec![vec![false; set.len()]; set.len()];
    let mut index: usize = 0;
    for (start, end) in temp {
        if !map1.contains_key(&start.to_string()) {
            map1.insert(start.to_string(), index);
            map2.insert(index, start.to_string());
            index += 1;
        }
        if !map1.contains_key(&end.to_string()) {
            map1.insert(end.to_string(), index);
            map2.insert(index, end.to_string());
            index += 1;
        }
        vec[*map1.get(start).unwrap()][*map1.get(end).unwrap()] = true;
        vec[*map1.get(end).unwrap()][*map1.get(start).unwrap()] = true;
    }
    (vec, map1, map2)
}

fn debug(paths: Vec<usize>, map: HashMap<usize, String>) -> Vec<String> {
    paths.iter().map(|p| map.get(p).unwrap().clone()).collect()
}

fn part1(arg: &str) -> i32 {
    let (path, map1, map2) = parse(arg);

    let mut queue = VecDeque::new();
    let start_pos = *map1.get(&"start".to_string()).unwrap();
    let end_pos = *map1.get(&"end".to_string()).unwrap();
    queue.push_back((start_pos, HashSet::new(), vec![]));
    let mut result = 0;
    while let Some((pos, mut visited, mut paths)) = queue.pop_front() {
        paths.push(pos);
        if pos == end_pos {
            result += 1;
            continue;
        }
        if let Some(n) = map2.get(&pos) {
            let is_capital = n.chars().any(|c| c.is_uppercase());
            if !is_capital {
                visited.insert(pos);
            }
        }
        for (i, p) in path[pos].iter().enumerate() {
            let prev = paths.last();
            if *p && Some(&i) != prev && !visited.contains(&i) {
                queue.push_back((i, visited.clone(), paths.clone()));
            }
        }
    }
    result
}

fn part2(arg: &str) -> i32 {
    let (path, map1, map2) = parse(arg);

    let mut queue = VecDeque::new();
    let start_pos = *map1.get(&"start".to_string()).unwrap();
    let end_pos = *map1.get(&"end".to_string()).unwrap();
    queue.push_back((start_pos, HashMap::new(), vec![], 0));
    let mut result = 0;
    while let Some((pos, mut visited, mut paths, mut max_visits)) = queue.pop_front() {
        paths.push(pos);
        if pos == end_pos {
            result += 1;
            continue;
        }
        if let Some(n) = map2.get(&pos) {
            if n == "start" {
                *visited.entry(pos).or_insert(0) += 2;
            } else {
                let is_capital = n.chars().any(|c| c.is_uppercase());
                if !is_capital {
                    *visited.entry(pos).or_insert(0) += 1;
                    max_visits = max_visits.max(*visited.get(&pos).unwrap());
                }
            }
        }
        for (i, p) in path[pos].iter().enumerate() {
            let prev = paths.last();
            let visit_count = *visited.entry(i).or_default();
            if max_visits == 2 {
                if *p && Some(&i) != prev && visit_count < 1 {
                    queue.push_back((i, visited.clone(), paths.clone(), max_visits));
                }
            } else if *p && Some(&i) != prev && visit_count < 2 {
                queue.push_back((i, visited.clone(), paths.clone(), max_visits));
            }
        }
    }
    result
}

fn main() {
    println!("{:?}", part1("./data.txt"));
    println!("{:?}", part2("./data.txt"));
}
