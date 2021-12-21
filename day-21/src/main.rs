use std::collections::VecDeque;

fn part1(player1: usize, player2: usize) -> usize {
    let mut p1 = player1;
    let mut p2 = player2;
    let mut prev_player1 = 0;
    let mut prev_player2 = 0;
    let mut player1 = 0;
    let mut player2 = 0;
    let mut died = 0;
    let mut rolled = 0;
    while player1 < 1000 && player2 < 1000 {
        prev_player1 = player1;
        prev_player2 = player2;
        for _ in 0..3 {
            died += 1;
            if died > 100 {
                died = 1;
            }
            rolled += 1;
            p1 += died;
        }
        p1 %= 10;
        if p1 == 0 {
            player1 += 10;
        } else {
            player1 += p1;
        }
        if player1 >= 1000 {
            break;
        }
        for _ in 0..3 {
            died += 1;
            if died > 100 {
                died = 1;
            }
            rolled += 1;
            p2 += died;
        }
        p2 %= 10;
        if p2 == 0 {
            player2 += 10;
        } else {
            player2 += p2;
        }
    }
    if player1 > player2 {
        prev_player2 * rolled
    } else {
        prev_player1 * rolled
    }
}

#[derive(Debug, Clone)]
struct Player {
    total: usize,
    current: usize,
}

impl Player {
    fn next(&mut self, die: usize) {
        let next = (self.current + die - 1) % 10 + 1;
        let total = self.total + next;
        self.current = next;
        self.total = total;
    }
}

#[derive(Debug, Clone)]
struct Universe {
    player1: Player,
    player2: Player,
    player1_turn: bool,
    nums: usize,
}

impl Universe {
    fn next(&self, die: usize) -> Universe {
        let universes = [0, 0, 0, 1, 3, 6, 7, 6, 3, 1];
        let mut new_universe = self.clone();
        let mut player1 = new_universe.player1;
        let mut player2 = new_universe.player2;
        if new_universe.player1_turn {
            player1.next(die);
        } else {
            player2.next(die);
        }
        new_universe.player1_turn = !new_universe.player1_turn;
        if die < universes.len() {
            new_universe.nums *= universes[die];
        }
        new_universe.player1 = player1;
        new_universe.player2 = player2;
        new_universe
    }
}

fn part2(player1: usize, player2: usize) -> usize {
    let mut queue = VecDeque::new();
    let mut player1_wins = 0;
    let mut player2_wins = 0;
    queue.push_back(Universe {
        player1: Player {
            total: 0,
            current: player1,
        },
        player2: Player {
            total: 0,
            current: player2,
        },
        player1_turn: true,
        nums: 1,
    });
    while let Some(universe) = queue.pop_front() {
        for die in 3..=9 {
            let new_universe = universe.next(die);
            if new_universe.player1.total >= 21 || new_universe.player2.total >= 21 {
                if new_universe.player1.total > new_universe.player2.total {
                    player1_wins += new_universe.nums;
                } else {
                    player2_wins += new_universe.nums;
                }
            } else {
                queue.push_back(new_universe);
            }
        }
    }
    println!("p1 {}, p2 {}", player1_wins, player2_wins);
    player1_wins.max(player2_wins)
}

fn main() {
    println!("{}", part1(6, 4));
    println!("{}", part2(6, 4));
}
