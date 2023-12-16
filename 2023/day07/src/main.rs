use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::Instant;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
enum HandType {
    Five,
    Four,
    Full,
    Three,
    TwoPair,
    OnePair,
    High,
}

// Map Hand Type to an index value for comparison
impl HandType {
    pub fn index(&self) -> usize {
        *self as usize
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Hand {
    cards: String,
    bid: u32,
    t: HandType,
}

fn get_hand_type(cards: String, use_jokers: bool) -> HandType {
    let mut hand: HashMap<char, usize> = HashMap::new();
    let mut jokers = 0;
    for c in cards.chars() {
        hand.entry(c).and_modify(|c| *c += 1).or_insert(1);
        if c == 'J' {
            jokers += 1;
        }
    }
    if hand.len() == 1 {
        return HandType::Five;
    }
    if hand.len() == 2 {
        if use_jokers && jokers > 0 {
            return HandType::Five; // Any joker means its five of a kind
        }
        for (_c, count) in hand {
            if count == 3 {
                return HandType::Full;
            }
        }
        return HandType::Four;
    }
    if hand.len() == 3 {
        for (_c, count) in hand {
            if count == 3 && use_jokers && jokers > 0 {
                return HandType::Four;
            }
            if count == 3 {
                return HandType::Three;
            }
        }
        if use_jokers && jokers == 1 {
            return HandType::Full;
        }
        if use_jokers && jokers == 2 {
            return HandType::Four;
        }
        return HandType::TwoPair;
    }
    if hand.len() == 4 {
        if use_jokers && jokers > 0 {
            return HandType::Three;
        }
        return HandType::OnePair;
    }
    if use_jokers && jokers > 0 {
        return HandType::OnePair;
    }
    return HandType::High;
}

fn get_val(c: char, use_joker: bool) -> u32 {
    if c.is_numeric() {
        return c.to_digit(10).unwrap();
    }
    // A, K, Q, J, T
    if c == 'T' {
        return 10;
    }
    if c == 'J' {
        if use_joker {
            return 1;
        }
        return 11;
    }
    if c == 'Q' {
        return 12;
    }
    if c == 'K' {
        return 13;
    }
    if c == 'A' {
        return 14;
    }
    return 0;
}

fn compare_hands(a: &Hand, b: &Hand, use_joker: bool) -> bool {
    if a.t.index() > b.t.index() {
        return true;
    } else if a.t.index() == b.t.index() {
        // Otherwise they're equal - iterate through characters
        let a_c = a.cards.chars();
        let b_c: Vec<char> = b.cards.chars().collect();
        for (i, c) in a_c.into_iter().enumerate() {
            if get_val(c, use_joker) < get_val(b_c[i], use_joker) {
                return true;
            }
            if get_val(c, use_joker) > get_val(b_c[i], use_joker) {
                return false;
            }
        }
        return false;
    }
    return false;
}

fn bubble_sort(hands: &mut Vec<Hand>, use_joker: bool) {
    for i in 0..hands.len() {
        for j in 0..hands.len() - i - 1 {
            if compare_hands(&hands[j + 1], &hands[j], use_joker) {
                hands.swap(j, j + 1);
            }
        }
    }
}

fn get_sum(hands: Vec<Hand>) -> u32 {
    let mut sum = 0;
    let mut rank = 1;
    for h in hands {
        sum += h.bid * rank;
        rank += 1;
    }
    return sum;
}

fn main() {
    let now = Instant::now();
    let mut hands: Vec<Hand> = Vec::new();
    let mut joker_hands: Vec<Hand> = Vec::new();
    for line in read_to_string("./input.txt").unwrap().lines() {
        let splits: Vec<&str> = line.split(" ").collect();
        hands.push(Hand {
            cards: splits[0].to_string(),
            bid: splits[1].parse::<u32>().unwrap(),
            t: get_hand_type(splits[0].to_string(), false),
        });
        joker_hands.push(Hand {
            cards: splits[0].to_string(),
            bid: splits[1].parse::<u32>().unwrap(),
            t: get_hand_type(splits[0].to_string(), true),
        });
    }
    bubble_sort(&mut hands, false);
    println!("Part 1: {}", get_sum(hands));
    bubble_sort(&mut joker_hands, true);
    println!("Part 2: {}", get_sum(joker_hands));
    println!("Done in: {:.2?}!", now.elapsed());
}
