use lazy_static::lazy_static;
use regex::Regex;
use std::cmp;
use std::fs::read_to_string;
use std::time::Instant;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Pt {
    x: i64,
    y: i64,
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Game {
    a: Pt,
    b: Pt,
    prize: Pt,
}

fn play_game(game: &Game) -> (bool, i64) {
    let a = game.a;
    let b = game.b;
    let prize = game.prize;
    let max_a = cmp::max(prize.x / a.x, prize.y / a.y) + 1;
    let max_b = cmp::max(prize.x / b.x, prize.y / b.y) + 1;
    // A costs 3 tokens
    // B costs 1 tokens
    let mut tokens = 3 * max_a + max_b;
    let mut can_win = false;
    for i in 0..max_a {
        for j in 0..max_b {
            let x = a.x * i + b.x * j;
            let y = a.y * i + b.y * j;
            if x == prize.x && y == prize.y {
                can_win = true;
                let cost = 3 * i + j;
                if cost < tokens {
                    tokens = cost;
                }
            }
        }
    }
    (can_win, tokens)
}

fn play_games(games: &Vec<Game>) -> (i64, usize) {
    let mut tokens = 0;
    let mut prizes = 0;

    for game in games {
        let (can_win, min_tokens) = play_game(game);
        if can_win {
            prizes += 1;
            tokens += min_tokens;
        }
    }

    (tokens, prizes)
}

fn main() {
    lazy_static! {
        static ref reA: Regex = Regex::new(r"Button A: X\+(\d*), Y\+(\d*)").unwrap();
        static ref reB: Regex = Regex::new(r"Button B: X\+(\d*), Y\+(\d*)").unwrap();
        static ref rePrize: Regex = Regex::new(r"Prize: X=(\d*), Y=(\d*)").unwrap();
    }
    let mut now = Instant::now();
    let mut games: Vec<Game> = Vec::new();
    let mut harder_games: Vec<Game> = Vec::new();
    let mut i = 1;
    let mut a = Pt { x: 0, y: 0 };
    let mut b = Pt { x: 0, y: 0 };
    for line in read_to_string("./src/input.txt").unwrap().lines() {
        match i {
            1 => {
                let cap = reA.captures(line).unwrap();
                a = Pt {
                    x: cap[1].parse::<i64>().unwrap(),
                    y: cap[2].parse::<i64>().unwrap(),
                };
                i = 2;
            }
            2 => {
                let cap = reB.captures(line).unwrap();
                b = Pt {
                    x: cap[1].parse::<i64>().unwrap(),
                    y: cap[2].parse::<i64>().unwrap(),
                };
                i = 3;
            }
            3 => {
                let cap = rePrize.captures(line).unwrap();
                games.push(Game {
                    a,
                    b,
                    prize: Pt {
                        x: cap[1].parse::<i64>().unwrap(),
                        y: cap[2].parse::<i64>().unwrap(),
                    },
                });
                harder_games.push(Game {
                    a,
                    b,
                    prize: Pt {
                        x: cap[1].parse::<i64>().unwrap() + 10000000000000,
                        y: cap[2].parse::<i64>().unwrap() + 10000000000000,
                    },
                });
                i = 4;
            }
            4 => {
                i = 1;
            }
            i64::MIN..=0_i64 | 5_i64..=i64::MAX => todo!(),
        };
    }
    // Part 1
    println!("Part 1: {}", play_games(&games).0);
    println!("Done in: {:?}!", now.elapsed());
    // Part 2
    now = Instant::now();
    println!("Part 2: {}", play_games(&harder_games).0);
    println!("Done in: {:?}!", now.elapsed());
}
