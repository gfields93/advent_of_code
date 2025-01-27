mod part1;

use crate::day15::part1::{Map, NodeType};

use colored::Colorize;
use std::collections::HashSet;
use std::io::{stdout, Write};
use std::{
    collections::{HashMap, VecDeque},
    isize,
    ops::{Index, IndexMut},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = String;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Direction::Up),
            'v' => Ok(Direction::Down),
            '<' => Ok(Direction::Left),
            '>' => Ok(Direction::Right),
            _ => Err(format!("Invalid direction: {}", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Position {
    row: usize,
    column: usize,
}

impl Position {
    fn index(&self) -> usize {
        100 * self.row + self.column
    }

    fn move_direction(&self, direction: Direction) -> Position {
        match direction {
            Direction::Up => Position {
                row: self.row - 1,
                column: self.column,
            },
            Direction::Down => Position {
                row: self.row + 1,
                column: self.column,
            },
            Direction::Left => Position {
                row: self.row,
                column: self.column - 1,
            },
            Direction::Right => Position {
                row: self.row,
                column: self.column + 1,
            },
        }
    }

    fn from(row: usize, column: usize) -> Position {
        Position { row, column }
    }
}

fn part_1(input: &str) -> u32 {
    let mut map: Map = Map::from(input);
    println!("Initial map:");
    map.print_map();
    let moves = map.moves.clone();
    for _move in moves {
        match _move {
            Direction::Up => println!("^"),
            Direction::Down => println!("v"),
            Direction::Left => println!("<"),
            Direction::Right => println!(">"),
        }
        map.move_robot(_move);
        map.print_map();
    }

    let (row, column) = (map.rows, map.columns);
    let mut count = 0;
    for i in 0..row {
        for j in 0..column {
            if map.map[map.position_to_index(&Position { row: i, column: j })] == NodeType::Box {
                count += Position { row: i, column: j }.index() as u32;
            }
        }
    }
    count
}

fn part_2(input: &str) -> u32 {
    let mut map: ExpandedMap = ExpandedMap::from(input);
    // map.print_map();
    // println!();
    let path = map.path.clone();
    for direction in path {
        // println!("Direction: {:?}", direction);
        map.move_robot(direction);
        // map.print_map();
        // println!();
    }

    let (height, width) = (map.height, map.width);
    let mut count = 0;
    for row in 0..height {
        for column in 0..width {
            if map[Position::from(row, column)] == Some(NodeTypeII::LeftBox) {
                count += Position { row, column }.index() as u32;
                map[Position { row, column }] = None;
                map[Position {
                    row,
                    column: column + 1,
                }] = None;
            }
        }
    }
    count
}

#[derive(PartialEq, Clone, Copy, Eq, Hash, Debug)]
enum NodeTypeII {
    Robot,
    LeftBox,
    RightBox,
    Wall,
    // Empty,
}

struct ExpandedMap {
    map: Vec<Option<NodeTypeII>>,
    height: usize,
    width: usize,
    robot: Position,
    path: Vec<Direction>,
    cardinals: HashMap<Direction, (isize, isize)>,
}
impl Index<Position> for ExpandedMap {
    type Output = Option<NodeTypeII>;
    fn index(&self, index: Position) -> &Self::Output {
        let idx = index.row * self.width + index.column;
        &self.map[idx]
    }
}
impl IndexMut<Position> for ExpandedMap {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        let idx = index.row * self.width + index.column;
        &mut self.map[idx]
    }
}

impl ExpandedMap {
    pub fn from(input: &str) -> Self {
        let (input, _moves) = input.split_once("\n\n").unwrap();
        let height = input.lines().count();
        let width = input.lines().next().unwrap().chars().count() * 2;
        println!("Height: {}, Width: {}", height, width);
        let mut map = Vec::new();
        let mut robot = Position { row: 0, column: 0 };
        for (i, line) in input.lines().enumerate() {
            for (j, c) in line.chars().enumerate() {
                match c {
                    '#' => {
                        map.extend_from_slice(&[Some(NodeTypeII::Wall); 2]);
                    }
                    '.' => {
                        map.extend_from_slice(&[None; 2]);
                    }
                    '@' => {
                        robot = Position {
                            row: i,
                            column: j * 2,
                        };
                        map.extend_from_slice(&[Some(NodeTypeII::Robot), None]);
                    }
                    'O' => {
                        map.extend_from_slice(&[
                            Some(NodeTypeII::LeftBox),
                            Some(NodeTypeII::RightBox),
                        ]);
                    }
                    _ => continue,
                }
            }
        }
        let path = _moves
            .chars()
            .filter_map(|c| Direction::try_from(c).ok())
            .collect();
        let cardinals: HashMap<Direction, (isize, isize)> = [
            (Direction::Up, (0, -1)),
            (Direction::Down, (0, 1)),
            (Direction::Left, (-1, 0)),
            (Direction::Right, (1, 0)),
            // (Direction::None, (0, 0)),
        ]
        .iter()
        .cloned()
        .collect();
        ExpandedMap {
            map,
            height,
            width,
            robot,
            path,
            cardinals,
        }
    }

    fn get_index(&self, position: &Position) -> usize {
        position.row * self.width + position.column
    }

    fn move_robot(&mut self, direction: Direction) {
        let cur_robot_pos = self.robot;
        // println!(
        //     "Current robot position and index: {:?} => idx: {}",
        //     cur_robot_pos,
        //     self.get_index(&cur_robot_pos)
        // );
        let next_position = cur_robot_pos.move_direction(direction);
        // println!(
        //     "Next position and index: {:?} => idx: {}",
        //     next_position,
        //     self.get_index(&next_position)
        // );
        let next_node_type = self[next_position];
        match next_node_type {
            Some(NodeTypeII::LeftBox) | Some(NodeTypeII::RightBox) => {
                let mut boxes_to_move = self.find_block_of_boxes(direction);
                if boxes_to_move.is_empty() {
                    return;
                }
                self.shift_boxes(&mut boxes_to_move, direction);
                self[cur_robot_pos] = None;
                self[next_position] = Some(NodeTypeII::Robot);
                self.robot = next_position;
            }
            Some(NodeTypeII::Wall) => return,
            None => {
                self[cur_robot_pos] = None;
                self.robot = next_position;
                self[next_position] = Some(NodeTypeII::Robot);
            }
            _ => {
                panic!("Unexpected node at {:?}", next_position);
            }
        }
    }

    fn find_block_of_boxes(&self, direction: Direction) -> Vec<Position> {
        let mut queue = VecDeque::new();
        let mut boxes_to_move = Vec::new();
        let mut visited = HashSet::new();

        let robot = self.robot;
        let current_node_pos = robot.move_direction(direction);
        queue.push_back(current_node_pos);

        while let Some(current_pos) = queue.pop_front() {
            if visited.contains(&current_pos) {
                continue;
            } else {
                visited.insert(current_pos);
            }
            let current_node_type = self[current_pos];
            if current_node_type.is_some() && current_node_type.unwrap() == NodeTypeII::Robot {
                panic!("Unexpected node at {:?}", current_pos);
            }
            if let None = current_node_type {
                continue;
            }
            if current_node_type.is_some() && current_node_type.unwrap() == NodeTypeII::Wall {
                println!("Wall encountered, breaking search");
                return vec![];
            }

            // Current node is a box, add it to the list of boxes to move
            boxes_to_move.push(current_pos);
            match direction {
                Direction::Left | Direction::Right => {
                    let next_node_pos = current_pos.move_direction(direction);
                    queue.push_back(next_node_pos);
                }
                Direction::Up | Direction::Down => {
                    let other_block = if self[current_pos] == Some(NodeTypeII::LeftBox) {
                        current_pos.move_direction(Direction::Right)
                    } else {
                        current_pos.move_direction(Direction::Left)
                    };
                    let new_blocks = &[current_pos.move_direction(direction), other_block];
                    queue.extend(new_blocks.into_iter());
                }
            }
        }
        boxes_to_move
    }

    fn shift_boxes(&mut self, mut blocks: &mut [Position], direction: Direction) {
        while !blocks.is_empty() {
            let top_block = blocks.first().unwrap();
            let new_pos = top_block.move_direction(direction);
            match self[new_pos] {
                None => {
                    let top_pos_idx = self.get_index(top_block);
                    let new_pos_idx = self.get_index(&new_pos);
                    self.map[new_pos_idx] = self.map[top_pos_idx];
                    self.map[top_pos_idx] = None;
                    blocks = blocks.split_first_mut().map(|(_, rest)| rest).unwrap();
                }
                Some(NodeTypeII::Wall) => panic!("Unexpected wall at {:?}", new_pos),
                _ => blocks.rotate_left(1),
            }
        }
    }

    pub fn print_map(&self) {
        let height = self.height;
        let width = self.width;
        for i in 0..height {
            for j in 0..width {
                let idx = i * width + j;
                match self.map[idx] {
                    None => print!("{:<3}:{} ", idx, '.'),
                    Some(NodeTypeII::Wall) => print!("{:<3}:{} ", idx, "#".red()),
                    Some(NodeTypeII::Robot) => print!("{:<3}:{} ", idx, "@".green()),
                    Some(NodeTypeII::LeftBox) => print!("{:<3}:{} ", idx, "[".yellow()),
                    Some(NodeTypeII::RightBox) => print!("{:<3}:{} ", idx, "]".yellow()),
                }
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("input/day15/test_input.txt");
        assert_eq!(part_1(input), 2028);
    }

    #[test]
    fn run_part_1() {
        let input = include_str!("input/day15/input.txt");
        println!("Part 1: {}", part_1(input));
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("input/day15/test_input.txt");
        assert_eq!(part_2(input), 9021);
    }

    #[test]
    fn run_simple_test() {
        let input = include_str!("input/day15/simple_test.txt");
        println!("Part 2: {}", part_2(input));
    }

    #[test]
    fn nontrivial_test() {
        let input = include_str!("input/day15/nontrivial_test.txt");
        assert_eq!(9021, part_2(input));
    }

    #[test]
    fn run_part_2() {
        let input = include_str!("input/day15/input.txt");
        println!("Part 2: {}", part_2(input));
    }
}
