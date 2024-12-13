use lazy_static::lazy_static;
use regex::Regex;
use std::fs::read_to_string;
use std::time::Instant;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Pt {
    x: i32,
    y: i32,
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Game {
    a: Pt,
    b: Pt,
    prize: Pt,
}

fn play_game(game: &Game) -> (bool, i32) {
    let max = 100;
    // A costs 3 tokens
    // B costs 1 tokens
    let mut tokens = 3 * max + max;
    let mut can_win = false;
    for i in 0..max {
        for j in 0..max {
            let x = game.a.x * i + game.b.x * j;
            let y = game.a.y * i + game.b.y * j;
            if x == game.prize.x && y == game.prize.y {
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

fn play_games(games: &Vec<Game>) -> (i32, usize) {
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
    let mut i = 1;
    let mut a = Pt { x: 0, y: 0 };
    let mut b = Pt { x: 0, y: 0 };
    for line in read_to_string("./src/input.txt").unwrap().lines() {
        match i {
            1 => {
                let cap = reA.captures(line).unwrap();
                a = Pt {
                    x: cap[1].parse::<i32>().unwrap(),
                    y: cap[2].parse::<i32>().unwrap(),
                };
                i = 2;
            }
            2 => {
                let cap = reB.captures(line).unwrap();
                b = Pt {
                    x: cap[1].parse::<i32>().unwrap(),
                    y: cap[2].parse::<i32>().unwrap(),
                };
                i = 3;
            }
            3 => {
                let cap = rePrize.captures(line).unwrap();
                games.push(Game {
                    a,
                    b,
                    prize: Pt {
                        x: cap[1].parse::<i32>().unwrap(),
                        y: cap[2].parse::<i32>().unwrap(),
                    },
                });
                i = 4;
            }
            4 => {
                i = 1;
            }
            i32::MIN..=0_i32 | 5_i32..=i32::MAX => todo!(),
        };
    }
    // Part 1
    println!("Part 1: {}", play_games(&games).0);
    println!("Done in: {:?}!", now.elapsed());
    // Part 2
    now = Instant::now();
    println!("Part 2: {}", 0);
    println!("Done in: {:?}!", now.elapsed());
}
