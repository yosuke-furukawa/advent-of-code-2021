fn part1(y: i32) -> i32 {
    ((y.abs() - 1) * (y.abs())) / 2
}

fn part2(x1: i32, x2: i32, y1: i32, y2: i32) -> i32 {
    let min_x = ((x1 * 2) as f64).sqrt() as i32;
    let max_x = x2;
    let min_y = y2;
    let max_y = y2.abs() - 1;
    let mut count = 0;
    for ix in min_x..=max_x {
        for iy in min_y..=max_y {
            let mut x = 0;
            let mut y = 0;
            let mut dx = ix;
            let mut dy = iy;
            while x < x2 && y > y2 {
                x += dx;
                y += dy;
                if x >= x1 && x <= x2 && y >= y2 && y <= y1 {
                    count += 1;
                    break;
                }
                if dx > 0 {
                    dx -= 1;
                }
                dy -= 1;
            }
        }
    }
    count
}

fn main() {
    println!("{:?}", part1(-74));
    println!("{:?}", part2(20, 30, -5, -10));
    println!("{:?}", part2(281, 311, -54, -74));
}
