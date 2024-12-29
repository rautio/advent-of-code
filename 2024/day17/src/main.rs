use lazy_static::lazy_static;
use regex::Regex;
use std::fs::read_to_string;
use std::time::Instant;

lazy_static! {
    static ref re_reg: Regex = Regex::new(r"Register (.*): (.*)").unwrap();
    static ref re_prog: Regex = Regex::new(r"Program: (.*)").unwrap();
}

fn run_program(registers: &(i32, i32, i32), program: &Vec<i32>) -> (Vec<i32>, (i32, i32, i32)) {
    let mut output: Vec<i32> = Vec::new();
    let mut reg = registers.clone();

    let mut idx = 0;
    while idx < program.len() {
        let opcode = program[idx];
        let literal_operand = program[idx + 1];
        let mut combo_operand = literal_operand;
        let mut jumped = false;
        match literal_operand {
            4 => {
                combo_operand = reg.0;
            }
            5 => {
                combo_operand = reg.1;
            }
            6 => {
                combo_operand = reg.2;
            }
            _ => {}
        }
        match opcode {
            0 => {
                let base: i32 = 2;
                reg.0 = reg.0 / base.pow(combo_operand as u32);
            }
            1 => {
                reg.1 = reg.1 ^ literal_operand;
            }
            2 => {
                reg.1 = combo_operand % 8;
            }
            3 => {
                if reg.0 != 0 {
                    idx = literal_operand as usize;
                    jumped = true;
                }
            }
            4 => {
                reg.1 = reg.1 ^ reg.2;
            }
            5 => {
                output.push(combo_operand % 8);
            }
            6 => {
                let base: i32 = 2;
                reg.1 = reg.0 / base.pow(combo_operand as u32);
            }
            7 => {
                let base: i32 = 2;
                reg.2 = reg.0 / base.pow(combo_operand as u32);
            }
            _ => {}
        }
        if !jumped {
            idx += 2;
        }
    }

    (output, reg)
}

fn solve_part1(registers: &(i32, i32, i32), program: &Vec<i32>) -> String {
    let (output, _) = run_program(registers, program);

    output
        .iter()
        .map(|x| x.to_string() + ",")
        .collect::<String>()
}

fn main() {
    let mut now = Instant::now();
    let mut registers: (i32, i32, i32) = (0, 0, 0);
    let mut program: Vec<i32> = Vec::new();
    for line in read_to_string("./src/input.txt").unwrap().lines() {
        match re_reg.captures(line) {
            Some(cap_reg) => {
                let val = cap_reg[2].parse::<i32>().unwrap();
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
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect();
            }
            None => {}
        }
    }
    // Part 1
    println!("Part 1: {}", solve_part1(&registers, &program));
    println!("Done in: {:?}!", now.elapsed());
    // Part 2
    now = Instant::now();
    println!("Part 2: {}", 0);
    println!("Done in: {:?}!", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_program() {
        // Operands
        assert_eq!(run_program(&(10, 0, 0), &vec![0, 2]), (vec![], (2, 0, 0)));
        assert_eq!(run_program(&(0, 12, 0), &vec![1, 16]), (vec![], (0, 28, 0)));
        assert_eq!(run_program(&(0, 0, 0), &vec![2, 12]), (vec![], (0, 4, 0)));
        assert_eq!(
            run_program(&(10, 0, 0), &vec![3, 2, 2, 12]),
            (vec![], (10, 4, 0))
        );
        assert_eq!(
            run_program(&(0, 22, 35), &vec![4, 10000]),
            (vec![], (0, 53, 35))
        );
        assert_eq!(
            run_program(&(11, 22, 35), &vec![5, 6]),
            (vec![3], (11, 22, 35))
        );
        assert_eq!(run_program(&(19, 0, 0), &vec![6, 1]), (vec![], (19, 9, 0)));
        assert_eq!(run_program(&(19, 0, 0), &vec![7, 1]), (vec![], (19, 0, 9)));
        // Compound
        assert_eq!(run_program(&(0, 0, 9), &vec![2, 6]), (vec![], (0, 1, 9)));
        assert_eq!(
            run_program(&(10, 0, 0), &vec![5, 0, 5, 1, 5, 4]),
            (vec![0, 1, 2], (10, 0, 0))
        );
        assert_eq!(
            run_program(&(2024, 0, 0), &vec![0, 1, 5, 4, 3, 0]),
            (vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0], (0, 0, 0))
        );
        assert_eq!(run_program(&(0, 29, 0), &vec![1, 7]), (vec![], (0, 26, 0)));
        assert_eq!(
            run_program(&(0, 2024, 43690), &vec![4, 0]),
            (vec![], (0, 44354, 43690))
        );
    }
}
