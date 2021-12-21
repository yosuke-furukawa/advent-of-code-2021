use std::fs;

fn parse(arg: &str) -> (Vec<u8>, Vec<Vec<u8>>) {
    let text: String = fs::read_to_string(arg).unwrap();
    let temp: Vec<&str> = text.split_terminator("\n\n").collect();
    let algorithm = temp[0]
        .chars()
        .map(|c| if c == '#' { 1 } else { 0 })
        .collect();
    let image = temp[1]
        .split_terminator('\n')
        .map(|line| line.chars().map(|c| if c == '#' { 1 } else { 0 }).collect())
        .collect();
    (algorithm, image)
}

fn decode_from_binary(bin: &[u8]) -> u128 {
    bin.iter().enumerate().rev().fold(0, |acc, (pos, num)| {
        acc + *num as u128 * 2_u128.pow(bin.len() as u32 - 1 - pos as u32)
    })
}

fn solve(arg: &str, steps: i32) -> i32 {
    let (algorithm, image) = parse(arg);
    let mut prev = image;
    for step in 0..steps {
        let mut next = vec![];
        for y in -1..prev.len() as i32 + 1 {
            let mut next_line = vec![];
            for x in -1..prev[0].len() as i32 + 1 {
                let mut binary = vec![];
                for yy in y - 1..=y + 1 {
                    for xx in x - 1..=x + 1 {
                        if yy >= 0 && yy < prev.len() as i32 && xx >= 0 && xx < prev[0].len() as i32
                        {
                            binary.push(prev[yy as usize][xx as usize]);
                        } else {
                            binary.push(if step % 2 != 0 { 1 } else { 0 });
                        }
                    }
                }
                next_line.push(algorithm[decode_from_binary(&binary) as usize]);
            }
            next.push(next_line);
        }
        prev = next;
    }
    let mut result = 0;
    for p in prev.iter() {
        for n in p.iter() {
            if *n > 0 {
                result += 1;
            }
        }
    }
    result
}

fn part1(arg: &str) -> i32 {
    solve(arg, 2)
}

fn part2(arg: &str) -> i32 {
    solve(arg, 50)
}

fn main() {
    println!("{:?}", part1("./data.txt"));
    println!("{:?}", part2("./data.txt"));
}
