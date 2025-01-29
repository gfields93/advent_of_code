use std::{collections::HashMap, ops::Index};
use petgraph::{prelude::*, algo::{all_simple_paths, dijkstra}};
enum NodeType {
    Wall,
    Start,
    Empty,
    End,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

struct Node {
    node_type: NodeType,
    visited: bool,
    score: u32,
    id: usize,
}

#[derive(Debug, PartialEq, Eq, Hash, Default, Clone, Copy)]
struct Position(usize, usize);
impl Position {
    fn from(row: usize, column: usize) -> Self {
        Position(row, column)
    }
}

struct Map {
    map: DiGraphMap<usize, Direction>,
    height: usize,
    width: usize,
    start: usize,
    end: usize,
}

impl Map {
    pub fn new(input: &str) -> Self {
        let input_vec: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let height = input_vec.len();
        let width = input_vec[0].len();
        
        let mut graph: GraphMap<usize, Direction, Directed> = DiGraphMap::new();

        let mut row = input_vec.iter().position(|line| line.contains(&'S')).unwrap();
        let mut column = input_vec[row].iter().position(|c| c == &'S').unwrap();
        let start_pos = row * width + column;


        row = input_vec.iter().position(|line| line.contains(&'E')).unwrap();
        column = input_vec[row].iter().position(|c| c == &'E').unwrap();
        let end_pos = row * width + column;

        let start = graph.add_node(start_pos);
        let end = graph.add_node(end_pos);

        let direction_map: HashMap<Direction, (isize, isize)> = [
            (Direction::North, (-1, 0)),
            (Direction::South, (1, 0)),
            (Direction::East, (0, 1)),
            (Direction::West, (0, -1))
        ].into_iter().collect();

        for (row_idx, line) in input_vec.iter().enumerate() {
            for (col_idx, node_str) in line.iter().enumerate() {
                let node = graph.add_node(row_idx * width + col_idx);
                // print!("Node {} neighbors => ", node);
                if node_str == &'#' {
                    graph.add_node(node);
                    // println!();
                    continue;
                }            
                for (direction, (dy, dx)) in direction_map.iter() {
                    let new_row_idx = row_idx as isize + dy;
                    let new_col_idx = col_idx as isize + dx;
                    if (new_row_idx < 0 || new_row_idx > height as isize || new_col_idx < 0 || new_col_idx > width as isize) 
                        || input_vec[new_row_idx as usize][new_col_idx as usize] == '#' {
                        continue;
                    }
                    let next_node = new_row_idx as usize * width + new_col_idx as usize;
                    // print!("{} ", next_node);
                    graph.add_edge(node, next_node, *direction);
                }
                println!()
            }
            
        }

        Self { map: graph, height, width, start, end }
    }

    fn index(&self, row: usize, col: usize) -> usize {
        row * self.width + col
    }
}

fn part_1(input: &str) -> u32 {
    let map = Map::new(input);
    let graph = map.map;
    let start = map.start;
    let end = map.end;
    let mut current_direction = Direction::East;

    println!("Start: {}, End: {}", start, end);
    let map = dijkstra(&graph, start, Some(end), |weight: (usize, usize, &Direction)| {
        let edge_weight = *weight.weight();
        let score = match (current_direction, edge_weight) {
            (Direction::North, Direction::North) => 1,
            (Direction::North, Direction::South) => 2000,
            (Direction::North, Direction::East) => 1000,
            (Direction::North, Direction::West) => 1000,
            (Direction::South, Direction::North) => 2000,
            (Direction::South, Direction::South) => 1,
            (Direction::South, Direction::East) => 1000,
            (Direction::South, Direction::West) => 1000,
            (Direction::East, Direction::North) => 1000,
            (Direction::East, Direction::South) => 1000,
            (Direction::East, Direction::East) => 1,
            (Direction::East, Direction::West) => 2000,
            (Direction::West, Direction::North) => 1000,
            (Direction::West, Direction::South) => 1000,
            (Direction::West, Direction::East) => 2000,
            (Direction::West, Direction::West) => 1,
        };
        current_direction = edge_weight;
        score
    });

    println!("{:?}", map);
    *map.get(&end).unwrap() as u32
}

#[cfg(test)]
mod test {
    use crate::day16::*;

    #[test]
    fn test_input1() {
        let input = include_str!("input/day16/test_input1.txt");
        assert_eq!(7036, part_1(input));
    }

    #[test]
    fn test_input2() {
        let input = include_str!("input/day16/test_input2.txt");
        assert_eq!(11048, part_1(input));
    }

    #[test]
    fn run_part1() {
        let input = include_str!("input/day16/input.txt");
        println!("{}", part_1(input))
        
    }
}

