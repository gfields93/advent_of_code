use std::collections::HashMap;
use petgraph::prelude::*;
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
    start: NodeIndex<usize,
    end: NodeIndex<usize>,
    current: usize,
}
impl Map {
    pub fn new(input: &str) -> Self {
        todo!()
    }
}

fn part_1(input: &str) -> u32 {
    todo!()
}

