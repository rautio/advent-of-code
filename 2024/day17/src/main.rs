use lazy_static::lazy_static;
use regex::Regex;
use std::collections::VecDeque;
use std::fs::read_to_string;
use std::time::Instant;

lazy_static! {
    static ref re_reg: Regex = Regex::new(r"Register (.*): (.*)").unwrap();
    static ref re_prog: Regex = Regex::new(r"Program: (.*)").unwrap();
}

#[derive(Debug)]
#[repr(u8)]
enum Opcode {
    ADV = 0,
    BXL = 1,
    BST = 2,
    JNZ = 3,
    BXC = 4,
    OUT = 5,
    BDV = 6,
    CDV = 7,
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        unsafe { std::mem::transmute::<u8, Opcode>(value) }
    }
}

const A: usize = 4;
const B: usize = 5;
const C: usize = 6;

fn exec(registers: &(i64, i64, i64), program: &Vec<u8>) -> (Vec<u8>, (i64, i64, i64)) {
    let mut output: Vec<u8> = Vec::new();
    let mut reg = [0, 1, 2, 3, registers.0, registers.1, registers.2];

    let mut idx = 0;
    while idx < program.len() {
        let operand = program[idx + 1] as usize;
        let mut jumped = false;
        let op = Opcode::from(program[idx]);
        let mut new_idx = idx + 2;
        match op {
            // 0
            Opcode::ADV => reg[A] = reg[A] >> reg[operand],
            // 1
            Opcode::BXL => reg[B] ^= operand as i64,
            // 2
            Opcode::BST => reg[B] = reg[operand] % 8,
            // 3
            Opcode::JNZ => {
                new_idx = if reg[A] != 0 {
                    operand as usize
                } else {
                    new_idx
                }
            }
            // 4
            Opcode::BXC => reg[B] = reg[B] ^ reg[C],
            // 5
            Opcode::OUT => output.push((reg[operand] % 8) as u8),
            // 6
            Opcode::BDV => reg[B] = reg[A] >> reg[operand],
            // 7
            Opcode::CDV => reg[C] = reg[A] >> reg[operand],
        }
        idx = new_idx;
    }

    (output, (reg[A], reg[B], reg[C]))
}

fn solve_part1(registers: &(i64, i64, i64), program: &Vec<u8>) -> String {
    let (output, _) = exec(registers, program);

    output
        .iter()
        .map(|x| x.to_string() + ",")
        .collect::<String>()
}

fn solve_part2(program: &Vec<u8>) -> i64 {
    let mut to_visit = VecDeque::from([(program.len(), 0)]);

    while let Some((pos, a)) = to_visit.pop_front() {
        for i in 0..8 {
            let new_a: i64 = a * 8 + i;
            let (o, _) = exec(&(new_a, 0, 0), program);
            if o.iter().map(|a| *a).collect::<Vec<_>>() == program[pos - 1..] {
                to_visit.push_back((pos - 1, new_a));
                if o.len() == program.len() {
                    return new_a;
                }
            }
        }
    }

    0
}

fn main() {
    let mut now = Instant::now();
    let mut registers: (i64, i64, i64) = (0, 0, 0);
    let mut program: Vec<u8> = Vec::new();
    for line in read_to_string("./src/input.txt").unwrap().lines() {
        match re_reg.captures(line) {
            Some(cap_reg) => {
                let val = cap_reg[2].parse::<i64>().unwrap();
                match &cap_reg[1] {
                    "A" => {
                        registers.0 = val;
                    }
                    "B" => {
                        registers.1 = val;
                    }
                    "C" => {
                        registers.2 = val;
                    }
                    _ => {}
                }
            }
            None => {}
        }
        match re_prog.captures(line) {
            Some(cap_prog) => {
                program = cap_prog[1]
                    .split(',')
                    .map(|x| x.parse::<u8>().unwrap())
                    .collect();
            }
            None => {}
        }
    }
    // Part 1
    let output = solve_part1(&registers, &program);
    println!("Part 1: {}", output);
    println!("Done in: {:?}!", now.elapsed());
    // Part 2
    now = Instant::now();
    println!("Part 2: {}", solve_part2(&program));
    println!("Done in: {:?}!", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exec() {
        // Operands
        assert_eq!(exec(&(10, 0, 0), &vec![0, 2]), (vec![], (2, 0, 0)));
        assert_eq!(exec(&(0, 12, 0), &vec![1, 16]), (vec![], (0, 28, 0)));
        assert_eq!(exec(&(0, 0, 9), &vec![2, 6]), (vec![], (0, 1, 9)));
        assert_eq!(exec(&(10, 0, 4), &vec![3, 2, 2, 6]), (vec![], (10, 4, 4)));
        assert_eq!(exec(&(0, 22, 35), &vec![4, 6]), (vec![], (0, 53, 35)));
        assert_eq!(exec(&(11, 22, 35), &vec![5, 6]), (vec![3], (11, 22, 35)));
        assert_eq!(exec(&(19, 0, 0), &vec![6, 1]), (vec![], (19, 9, 0)));
        assert_eq!(exec(&(19, 0, 0), &vec![7, 1]), (vec![], (19, 0, 9)));
        // Compound
        assert_eq!(exec(&(0, 0, 9), &vec![2, 6]), (vec![], (0, 1, 9)));
        assert_eq!(
            exec(&(10, 0, 0), &vec![5, 0, 5, 1, 5, 4]),
            (vec![0, 1, 2], (10, 0, 0))
        );
        assert_eq!(
            exec(&(2024, 0, 0), &vec![0, 1, 5, 4, 3, 0]),
            (vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0], (0, 0, 0))
        );
        assert_eq!(exec(&(0, 29, 0), &vec![1, 7]), (vec![], (0, 26, 0)));
        assert_eq!(
            exec(&(0, 2024, 43690), &vec![4, 0]),
            (vec![], (0, 44354, 43690))
        );
    }
}
