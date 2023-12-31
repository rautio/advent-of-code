use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::read_to_string;
use std::time::Instant;

lazy_static! {
    static ref CARD: Regex = Regex::new(r"^Card[ \t]+([0-9]+): (.*)$").unwrap();
}
#[derive(Debug, Clone)]
struct Card {
    winning: Vec<u32>,
    mine: Vec<u32>,
    num: u32,
}

fn count_wins(winning: Vec<u32>, mine: Vec<u32>) -> u32 {
    let mut wins = 0;
    for w in winning {
        if mine.contains(&w) {
            wins += 1;
        }
    }
    let base: i32 = 2;
    if wins > 0 {
        return base.pow(wins - 1) as u32;
    }
    return 0;
}

fn get_wins(winning: Vec<u32>, mine: Vec<u32>) -> Vec<u32> {
    let mut cards: Vec<u32> = Vec::new();
    for w in winning {
        if mine.contains(&w) {
            cards.push(w);
        }
    }
    return cards;
}

fn parse_nums(line: &str) -> (u32, Vec<u32>, Vec<u32>) {
    let cap = CARD.captures(line).unwrap();
    let card_num: u32 = cap.get(1).unwrap().as_str().trim().parse().unwrap();
    let nums = cap.get(2).unwrap().as_str();
    let splits: Vec<&str> = nums.split("|").collect();
    let winning: Vec<u32> = splits[0]
        .trim()
        .split(" ")
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.trim())
        .filter(|x| *x != "")
        .map(|x| x.parse().unwrap())
        .collect();
    let mine: Vec<u32> = splits[1]
        .trim()
        .split(" ")
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.trim())
        .filter(|x| *x != "")
        .map(|x| x.parse().unwrap())
        .collect();
    return (card_num, winning, mine);
}

fn solve_part1(input_file: &str) -> u32 {
    let mut sum = 0;
    for line in read_to_string(input_file).unwrap().lines() {
        let (_card, winning, mine) = parse_nums(line);
        sum += count_wins(winning, mine);
    }
    return sum;
}

fn solve_part2(input_file: &str) -> u32 {
    let mut card_map: HashMap<u32, Card> = HashMap::new();
    let mut cards: VecDeque<Card> = VecDeque::new();
    for line in read_to_string(input_file).unwrap().lines() {
        let (card, winning, mine) = parse_nums(line);
        let c = Card {
            num: card,
            winning: winning,
            mine: mine,
        };
        card_map.insert(card, c.clone());
        cards.push_back(c);
    }
    let mut num_cards = 0;
    while cards.len() > 0 {
        num_cards += 1;
        let card = cards.pop_front().unwrap();
        let wins = get_wins(card.winning, card.mine);
        if wins.len() > 0 {
            for i in 0..wins.len() {
                let new_card_num = card.num + i as u32 + 1;
                if card_map.contains_key(&new_card_num) {
                    cards.push_back(card_map.get(&new_card_num).unwrap().clone());
                }
            }
        }
    }
    return num_cards;
}

fn main() {
    let mut now = Instant::now();
    println!("Part 1: {}", solve_part1("./input.txt"));
    println!("Done in: {:?}!", now.elapsed());
    now = Instant::now();
    println!("Part 2: {0}", solve_part2("./input.txt"));
    println!("Done in: {:?}!", now.elapsed());
}
