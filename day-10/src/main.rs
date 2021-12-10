use std::fs;

fn parse(arg: &str) -> Vec<Vec<char>> {
    let text: String = fs::read_to_string(arg).unwrap();
    text.split_terminator('\n')
        .into_iter()
        .map(|s| s.chars().collect())
        .collect()
}

fn part1(arg: &str) -> i32 {
    let data = parse(arg);
    let mut result = 0;
    for line in data {
        let mut stack = vec![];
        for (i, c) in line.iter().enumerate() {
            if let Some(top) = stack.last() {
                if line[*top] == '<' && *c == '>'
                    || line[*top] == '(' && *c == ')'
                    || line[*top] == '[' && *c == ']'
                    || line[*top] == '{' && *c == '}'
                {
                    stack.pop();
                } else if *c == '<' || *c == '(' || *c == '{' || *c == '[' {
                    stack.push(i);
                } else {
                    result += match *c {
                        '>' => 25137,
                        ')' => 3,
                        '}' => 1197,
                        ']' => 57,
                        _ => panic!("no such char {}", *c),
                    };
                    break;
                }
            } else {
                stack.push(i);
            }
        }
    }
    result
}

fn part2(arg: &str) -> u64 {
    let data = parse(arg);
    let mut incompletes = vec![];
    for line in data {
        let mut stack = vec![];
        let mut incomplete = true;
        for (i, c) in line.iter().enumerate() {
            if let Some(top) = stack.last() {
                if line[*top] == '<' && *c == '>'
                    || line[*top] == '(' && *c == ')'
                    || line[*top] == '[' && *c == ']'
                    || line[*top] == '{' && *c == '}'
                {
                    stack.pop();
                } else if *c == '<' || *c == '(' || *c == '{' || *c == '[' {
                    stack.push(i);
                } else {
                    incomplete = false;
                    break;
                }
            } else {
                stack.push(i);
            }
        }
        if incomplete {
            let mut result: u64 = 0;

            for s in stack.iter().rev() {
                result = result * 5
                    + match line[*s] {
                        '<' => 4,
                        '{' => 3,
                        '[' => 2,
                        '(' => 1,
                        _ => panic!("no such char {}", line[*s]),
                    };
            }
            incompletes.push(result);
        }
    }
    incompletes.sort_unstable();
    incompletes[incompletes.len() / 2]
}

fn main() {
    println!("{:?}", part1("./data.txt"));
    println!("{:?}", part2("./data.txt"));
}
