use regex::Regex;
use std::{
    collections::HashMap,
    ops::{Add, Sub},
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct PrizeMachine {
    button_a: Pair,
    button_b: Pair,
    prize: Pair,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Pair(i64, i64);
impl Pair {
    fn from((x, y): (i64, i64)) -> Self {
        Pair(x, y)
    }

    fn from_pair(x: i64, y: i64) -> Self {
        Pair(x, y)
    }
}
impl PrizeMachine {
    fn new(button_a: Pair, button_b: Pair, prize: Pair) -> Self {
        PrizeMachine {
            button_a,
            button_b,
            prize,
        }
    }
}

impl Sub for Pair {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0, self.1 - other.1)
    }
}

impl Add for Pair {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

const TOKEN_A_COST: i64 = 3;
const TOKEN_B_COST: i64 = 1;
const FIXED_COST: i64 = 10000000000000;

fn part_1(prize_machine: &[PrizeMachine]) -> i64 {
    let mut total = 0;
    for machine in prize_machine {
        let (button_a, button_b, mut prize) = (machine.button_a, machine.button_b, machine.prize);
        let mut button_a_cache = HashMap::new();
        let mut round_count = 1;
        while prize.0 >= 0 && prize.1 >= 0 {
            let new_prize = prize - button_a;
            if prize.0 >= 0 && prize.1 >= 0 {
                button_a_cache.insert(round_count, new_prize);
            }
            prize = new_prize;
            round_count += 1;
        }

        round_count = 1;
        let mut button_b_cache = HashMap::new();
        prize = machine.prize;
        while prize.0 >= 0 && prize.1 >= 0 {
            let new_prize = prize - button_b;
            if prize.0 >= 0 && prize.1 >= 0 {
                button_b_cache.insert(round_count, new_prize);
            }
            prize = new_prize;
            round_count += 1;
        }

        let mut minimum_token = std::i64::MAX;
        let mut prize_won = false;
        for (round_a, prize_a) in button_a_cache.iter() {
            for (round_b, prize_b) in button_b_cache.iter() {
                if *prize_a + *prize_b == machine.prize {
                    prize_won = true;
                    let a_token_cost = round_a * TOKEN_A_COST;
                    let b_token_cost = round_b * TOKEN_B_COST;
                    minimum_token = minimum_token.min(a_token_cost + b_token_cost);
                }
            }
        }
        if prize_won {
            // println!("Prize won! Minimum token cost: {}", minimum_token);
            total += minimum_token;
        }
    }
    total
}

fn part_2(machines: &[PrizeMachine]) -> u64 {
    let mut total = 0;
    let fixed_machines: Vec<PrizeMachine> = machines
        .into_iter()
        .map(|machine| {
            let mut new = machine.clone();
            new.prize.0 += FIXED_COST;
            new.prize.1 += FIXED_COST;
            new
        })
        .collect();
    for machine in fixed_machines {
        let sol = solve(machine.button_a, machine.button_b, machine.prize);
        if let Some(sol) = sol {
            // println!("Sol: {}", sol);
            total += 3 * sol.0 as u64 + sol.1 as u64;
        }
    }
    total
}

fn solve(a: Pair, b: Pair, r: Pair) -> Option<Pair> {
    let (x1, y1) = (a.0 as f64, a.1 as f64);
    let (x2, y2) = (b.0 as f64, b.1 as f64);
    let (p1, p2) = (r.0 as f64, r.1 as f64);

    // let A = (by * x - bx * y) / (ax * by - ay * bx);
    // let B = (ay * x - ax * y) / (ay * bx - ax * by);
    let a_num = (y2 * p1 - x2 * p2);
    let a_den = y2 * x1 - x2 * y1;
    let a = a_num / a_den;
    if a.fract() != 0.0 {
        return None;
    }

    let b_num = p1 - x1 * a;
    let b_den = x2;
    let b = b_num / b_den;
    if b.fract() != 0.0 {
        return None;
    }

    Some(Pair::from((a as i64, b as i64)))
}

fn create_prize_machines(input: &str) -> Vec<PrizeMachine> {
    let groups = input.split("\n\n");
    // println!("{:?}", groups);
    let mut prize_machines = Vec::new();
    for group in groups {
        let lines: Vec<&str> = group.lines().collect();
        if lines.len() != 3 {
            panic!("Invalid input format");
        }
        println!("{:?}", lines);
        let button_regex = Regex::new(r"Button [A|B]: X\+(\d+), Y\+(\d+)").unwrap();
        let prize_regex = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
        match (
            button_regex.captures(lines[0]),
            button_regex.captures(lines[1]),
            prize_regex.captures(lines[2]),
        ) {
            (Some(button_a_captures), Some(button_b_captures), Some(prize_captures)) => {
                println!("{:?}", button_a_captures);
                println!("{:?}", button_b_captures);
                println!("{:?}", prize_captures);
                let button_a = Pair::from_pair(
                    button_a_captures[1].parse().unwrap(),
                    button_a_captures[2].parse().unwrap(),
                );
                let button_b = Pair::from_pair(
                    button_b_captures[1].parse().unwrap(),
                    button_b_captures[2].parse().unwrap(),
                );
                let prize = Pair::from_pair(
                    prize_captures[1].parse().unwrap(),
                    prize_captures[2].parse().unwrap(),
                );
                prize_machines.push(PrizeMachine::new(button_a, button_b, prize));
            }
            _ => panic!("Invalid input format"),
        }
    }
    prize_machines
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create_prize_machines() {
        let input = include_str!("input/day13/test_input1.txt");
        let prize_machines = create_prize_machines(input);
        assert_eq!(4, prize_machines.len());
        let first = &prize_machines[0];
        assert_eq!(Pair::from((94, 34)), first.button_a);
        assert_eq!(Pair::from((22, 67)), first.button_b);
        assert_eq!(Pair::from((8400, 5400)), first.prize);
    }

    #[test]
    fn test_part_1() {
        let input = include_str!("input/day13/test_input1.txt");
        let prize_machines = create_prize_machines(input);
        assert_eq!(480, part_1(&prize_machines));
    }

    #[test]
    fn run_part_1() {
        let input = include_str!("input/day13/input.txt");
        let prize_machines = create_prize_machines(input);
        println!("{}", part_1(&prize_machines));
    }

    #[test]
    fn run_part_2() {
        let input = include_str!("input/day13/input.txt");
        let prize_machines = create_prize_machines(input);
        println!("{}", part_2(&prize_machines));
    }
}

// 113584927710783
// 151170889039089
// 151170889039089
// 76358113886726
