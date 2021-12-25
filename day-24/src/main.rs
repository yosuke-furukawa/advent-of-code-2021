use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone)]
struct Operation {
    operation: String,
    argument0: String,
    argument1: String,
}

fn to_index(x: &String) -> usize {
    match x.as_str() {
        "x" => 0,
        "y" => 1,
        "z" => 2,
        "w" => 3,
        _ => panic!("no such char {}", x),
    }
}

impl Operation {
    fn execute(&self, variants: &mut [i64], input: i64) {
        match self.operation.as_str() {
            "inp" => {
                variants[to_index(&self.argument0)] = input;
            }
            "add" => {
                let b = self
                    .argument1
                    .parse::<i64>()
                    .unwrap_or_else(|_| variants[to_index(&self.argument1)]);
                variants[to_index(&self.argument0)] += b;
            }
            "mul" => {
                let b = self
                    .argument1
                    .parse::<i64>()
                    .unwrap_or_else(|_| variants[to_index(&self.argument1)]);
                variants[to_index(&self.argument0)] *= b;
            }
            "div" => {
                let b = self
                    .argument1
                    .parse::<i64>()
                    .unwrap_or_else(|_| variants[to_index(&self.argument1)]);
                variants[to_index(&self.argument0)] /= b;
            }
            "mod" => {
                let b = self
                    .argument1
                    .parse::<i64>()
                    .unwrap_or_else(|_| variants[to_index(&self.argument1)]);
                variants[to_index(&self.argument0)] %= b;
            }
            "eql" => {
                let b = self
                    .argument1
                    .parse::<i64>()
                    .unwrap_or_else(|_| variants[to_index(&self.argument1)]);
                let a = variants[to_index(&self.argument0)];
                variants[to_index(&self.argument0)] = if a == b { 1 } else { 0 };
            }
            _ => panic!("no such operation {}", self.operation),
        }
    }
}

fn parse(arg: &str) -> Vec<Operation> {
    let mut results = vec![];
    let text = fs::read_to_string(arg).unwrap();
    let lines: Vec<&str> = text.split_terminator('\n').collect();
    for line in lines {
        let terms: Vec<&str> = line.split(' ').collect();
        let op = match terms.len() {
            3 => Operation {
                operation: terms[0].to_string(),
                argument0: terms[1].to_string(),
                argument1: terms[2].to_string(),
            },
            2 => Operation {
                operation: terms[0].to_string(),
                argument0: terms[1].to_string(),
                argument1: "".to_string(),
            },
            _ => panic!("no such len {}", terms.len()),
        };
        results.push(op);
    }
    results
}

fn execute(operations: &[Operation], variants: &mut [i64; 4], input: i64) -> [i64; 4] {
    let mut variants = variants.clone();
    for operation in operations {
        operation.execute(&mut variants, input);
    }
    variants
}

fn dfs(
    operations: &[Vec<Operation>],
    digit: usize,
    variants: &mut [i64; 4],
    results: &mut [i64],
    visited: &mut HashMap<(usize, [i64; 4]), i64>,
    range: [i64; 9],
) -> i64 {
    if digit > 13 {
        return 0;
    }
    if let Some(value) = visited.get(&(digit, *variants)) {
        return *value;
    }

    for input in range {
        let mut new_variants = execute(&operations[digit], variants, input);
        if digit == 13 && new_variants[2] == 0 {
            visited.insert((digit, *variants), input);
            results[digit] = input;
            return input;
        }
        let result = dfs(
            operations,
            digit + 1,
            &mut new_variants,
            results,
            visited,
            range,
        );
        if result != 0 {
            visited.insert((digit, *variants), result);
            results[digit] = input;
            return input;
        }
        visited.insert((digit, *variants), result);
    }
    0
}

fn part1(arg: &str) -> i64 {
    let operations = parse(arg);
    let mut input_operations = vec![];
    let mut i = 0;
    while i < operations.len() {
        if operations[i].operation == "inp" {
            let mut ops = vec![];
            ops.push(operations[i].clone());
            i += 1;
            let mut count = 0;
            for operation in operations.iter().skip(i) {
                if operation.operation == "inp" {
                    count -= 1;
                    break;
                }
                ops.push(operation.clone());
                count += 1;
            }
            input_operations.push(ops);
            i += count;
            continue;
        }
        i += 1;
    }
    let mut variants = [0, 0, 0, 0];
    let mut results = vec![0; 14];

    dfs(
        &input_operations,
        0,
        &mut variants,
        &mut results,
        &mut HashMap::new(),
        [9, 8, 7, 6, 5, 4, 3, 2, 1],
    );
    println!("{:?}", variants);
    println!("{:?}", results);
    results.into_iter().fold(0, |acc, cur| acc * 10 + cur)
}

fn part2(arg: &str) -> i64 {
    let operations = parse(arg);
    let mut input_operations = vec![];
    let mut i = 0;
    while i < operations.len() {
        if operations[i].operation == "inp" {
            let mut ops = vec![];
            ops.push(operations[i].clone());
            i += 1;
            let mut count = 0;
            for operation in operations.iter().skip(i) {
                if operation.operation == "inp" {
                    count -= 1;
                    break;
                }
                ops.push(operation.clone());
                count += 1;
            }
            input_operations.push(ops);
            i += count;
            continue;
        }
        i += 1;
    }
    let mut variants = [0, 0, 0, 0];
    let mut results = vec![0; 14];

    dfs(
        &input_operations,
        0,
        &mut variants,
        &mut results,
        &mut HashMap::new(),
        [1, 2, 3, 4, 5, 6, 7, 8, 9],
    );
    println!("{:?}", variants);
    println!("{:?}", results);
    results.into_iter().fold(0, |acc, cur| acc * 10 + cur)
}

fn check(arg: &str, args: &str) {
    let operations = parse(arg);
    let mut variants = [0, 0, 0, 0];
    let mut args: Vec<i64> = args
        .chars()
        .map(|x| x.to_string().parse::<i64>().unwrap())
        .rev()
        .collect();
    for operation in operations.iter() {
        if operation.operation == "inp" {
            operation.execute(&mut variants, args.pop().unwrap());
        } else {
            operation.execute(&mut variants, 0);
        }
    }
    println!("{:?}", variants);
}

fn main() {
    println!("{:?}", part1("./data.txt"));
    println!("{:?}", check("./data.txt", "91897399498995"));
    println!("{:?}", part2("./data.txt"));
    println!("{:?}", check("./data.txt", "51121176121391"));
}
