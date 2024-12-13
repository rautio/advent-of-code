use lazy_static::lazy_static;
use regex::Regex;
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

fn determinant(matrix: &Vec<Vec<i64>>) -> i64 {
    matrix[0][0] * matrix[1][1] - matrix[0][1] * matrix[1][0]
}

fn inverse_matrix(matrix: &Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let mut inv: Vec<Vec<i64>> = Vec::new();
    inv.push(vec![matrix[1][1], -1 * matrix[0][1]]);
    inv.push(vec![-1 * matrix[1][0], matrix[0][0]]);
    inv
}

fn play_game(game: &Game) -> (bool, i64) {
    let a = game.a;
    let b = game.b;
    let prize = game.prize;
    // A costs 3 tokens
    // B costs 1 tokens
    let matrix: Vec<Vec<i64>> = vec![vec![a.x, b.x], vec![a.y, b.y]];
    let det = determinant(&matrix);
    if det == 0 {
        // No solution
        return (false, 0);
    }
    let inv = inverse_matrix(&matrix);
    let res: Vec<i64> = inv
        .into_iter()
        .map(|row| row[0] * prize.x + row[1] * prize.y)
        .collect();
    let a_count: f64 = res[0] as f64 / det as f64;
    let b_count: f64 = res[1] as f64 / det as f64;

    if a_count >= 0.0 && b_count >= 0.0 && a_count.fract() == 0.0 && b_count.fract() == 0.0 {
        // There is a solution!
        let tokens = 3 * a_count as i64 + b_count as i64;
        return (true, tokens);
    }
    return (false, 0);
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
