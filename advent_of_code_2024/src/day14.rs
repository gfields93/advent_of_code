use std::{
    collections::{HashMap, HashSet},
    ops::{Add, AddAssign},
};

use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position(usize, usize);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Velocity(isize, isize);

impl Add<Velocity> for Position {
    type Output = Position;
    fn add(self, other: Velocity) -> Self::Output {
        unimplemented!()
    }
}

impl AddAssign<Velocity> for Position {
    fn add_assign(&mut self, other: Velocity) {
        let (col, row) = (self.0 as isize + other.0, self.1 as isize + other.1);
        // self.0.saturating_add_signed()
        if row < 0 {
            self.1 = (ROWS as isize + row) as usize;
        } else if row >= ROWS as isize {
            self.1 = row as usize % ROWS;
        } else {
            self.1 = row as usize;
        }

        if col < 0 {
            self.0 = (COLUMNS as isize + col) as usize;
        } else if col >= COLUMNS as isize {
            self.0 = col as usize % COLUMNS;
        } else {
            self.0 = col as usize;
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Robot {
    position: Position,
    velocity: Velocity,
}
const TIME_LIMIT: u32 = 100;
const COLUMNS: usize = 101;
const ROWS: usize = 103;

fn load_robots(input: &str) -> Vec<Robot> {
    let regex = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    input
        .lines()
        .map(|line| {
            let captures = regex.captures(line);
            if captures.is_none() {
                panic!("Invalid input format: {}", line);
            }
            let captures = captures.unwrap();
            let x: usize = captures[1].parse().unwrap();
            let y: usize = captures[2].parse().unwrap();
            let vx: isize = captures[3].parse().unwrap();
            let vy: isize = captures[4].parse().unwrap();
            Robot {
                position: Position(x, y),
                velocity: Velocity(vx, vy),
            }
        })
        .collect()
}

fn part1(input: &str) -> u32 {
    // let mut board: [Option<Robot>; 10403] = [None; COLUMNS * ROWS];
    let mut robots = load_robots(input);
    for _ in 0..TIME_LIMIT {
        for robot in robots.iter_mut() {
            robot.position += robot.velocity;
        }
    }
    println!(
        "{:?}",
        robots
            .iter()
            .map(|robot| robot.position)
            .collect::<Vec<_>>()
    );
    println_board(&robots);
    let col_boundary = 50;
    let row_boundary = 51;
    let mut quadrant_count = HashMap::new();
    quadrant_count.insert(1, 0);
    quadrant_count.insert(2, 0);
    quadrant_count.insert(3, 0);
    quadrant_count.insert(4, 0);
    for robot in robots {
        // if row_boundary_set.contains(&(robot.position.0, robot.position.1))
        //     || col_boundary_set.contains(&(robot.position.0, robot.position.1))
        // {
        //     continue;
        // }
        let quadrant = match (robot.position.0, robot.position.1) {
            (column, _) if column == col_boundary => continue,
            (_, row) if row == row_boundary => continue,
            (col, row) if col > col_boundary && row > row_boundary => 4,
            (col, row) if col > col_boundary && row < row_boundary => 3,
            (col, row) if col < col_boundary && row > row_boundary => 2,
            (col, row) if col < col_boundary && row < row_boundary => 1,
            _ => unreachable!(),
        };
        *quadrant_count.get_mut(&quadrant).unwrap() += 1;
    }
    println!(
        "Quadrant count: {:?}",
        quadrant_count.values().collect::<Vec<_>>()
    );
    quadrant_count.values().product()
}

fn println_board(board: &[Robot]) {
    for j in 0..ROWS {
        for i in 0..COLUMNS {
            if i == 50 || j == 51 {
                print!("█");
                continue;
            }
            let count = board
                .iter()
                .filter(|robot| robot.position.0 == i && robot.position.1 == j)
                .count();
            if count > 0 {
                print!("{}", count);
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn part_2(input: &str) -> u32 {
    let mut robots = load_robots(input);
    let num_of_robots = robots.len();
    let mut second = 0;
    let attempted = [8086];
    loop {
        for robot in robots.iter_mut() {
            robot.position += robot.velocity;
        }
        // let mut unique_positions = HashSet::new();
        let unique_count = robots
            .iter()
            .map(|robot| robot.position)
            .collect::<HashSet<_>>()
            .len();
        if unique_count == num_of_robots && !attempted.contains(&second) {
            return second;
        }
        second += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run_part_1() {
        let input = include_str!("input/day14/input.txt");
        println!("{}", part1(input));
    }

    #[test]
    fn run_part_2() {
        let input = include_str!("input/day14/input.txt");
        println!("{}", part_2(input));
    }
}
// 233853750
// 237782862
// 219608400
// 220231440
// 207792000
