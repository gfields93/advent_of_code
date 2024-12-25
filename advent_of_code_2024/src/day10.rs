use std::collections::HashMap;

use petgraph::adj::NodeIndex;
use petgraph::algo::{all_simple_paths, has_path_connecting};
use petgraph::graphmap::GraphMap;
use petgraph::prelude::DiGraphMap;
use petgraph::Directed;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Position(usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Node {
    height: u8,
    id: usize,
    position: Position,
}

fn part_1(input: &str) -> usize {
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().chars().count();
    let mut graph = DiGraphMap::<_, ()>::new();
    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            let height = ch.to_digit(10).unwrap();
            let id = rows * row + col;
            // let position = Position(row, col);
            let node = Node {
                height: height.try_into().unwrap(),
                id: id.try_into().unwrap(),
                position: Position(row, col),
            };
            graph.add_node(node);
        }
    }
    let node_list = graph.nodes().collect::<Vec<_>>();
    create_adj_list(rows, cols, &mut graph, &node_list);
    let mut count = 0;
    for node in node_list.iter().filter(|node| node.height == 0) {
        for end in node_list.iter().filter(|node| node.height == 9) {
            count += if has_path_connecting(&graph, node.clone(), end.clone(), None) {
                1
            } else {
                0
            };
        }
    }
    count
}

fn part_2(input: &str) -> usize {
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().chars().count();
    let mut graph = DiGraphMap::<_, ()>::new();
    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            let height = ch.to_digit(10).unwrap();
            let id = rows * row + col;
            // let position = Position(row, col);
            let node = Node {
                height: height.try_into().unwrap(),
                id: id.try_into().unwrap(),
                position: Position(row, col),
            };
            graph.add_node(node);
        }
    }
    let node_list = graph.nodes().collect::<Vec<_>>();
    create_adj_list(rows, cols, &mut graph, &node_list);
    let mut count = 0;
    for node in node_list.iter().filter(|node| node.height == 0) {
        for end in node_list.iter().filter(|node| node.height == 9) {
            count +=
                all_simple_paths::<Vec<_>, _>(&graph, node.clone(), end.clone(), 1, None).count()
        }
    }
    count
}

fn create_adj_list(
    rows: usize,
    cols: usize,
    graph: &mut GraphMap<Node, (), Directed>,
    node_list: &[Node],
) {
    for node in node_list.iter() {
        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0
                    || i == -1 && j == -1
                    || i == 1 && j == -1
                    || i == 1 && j == 1
                    || i == -1 && j == 1
                {
                    continue;
                }
                let neighbor_row = node.position.0 as i32 + i;
                let neighbor_col = node.position.1 as i32 + j;
                if neighbor_row >= 0
                    && neighbor_row < rows as i32
                    && neighbor_col >= 0
                    && neighbor_col < cols as i32
                {
                    let neighbor_id = (neighbor_row as usize) * rows + neighbor_col as usize;
                    let neighbor_node = &node_list[neighbor_id];
                    if node.height + 1 == neighbor_node.height {
                        graph.add_edge(node.clone(), neighbor_node.clone(), ());
                    }
                }
            }
        }
    }
}

fn format_hashmap_to_string(hashmap: &HashMap<usize, Vec<usize>>) -> String {
    let mut result = String::new();
    let mut ordered_keys: Vec<usize> = hashmap.keys().cloned().collect();
    ordered_keys.sort();
    for key in ordered_keys.into_iter() {
        result.push_str(&format!("{}: {:?}\n", key, hashmap.get(&key).unwrap()));
    }
    result
}

fn print_topological_map(input: &str, rows: usize, cols: usize) {
    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            let id = rows * row + col;
            let height: u8 = ch.to_digit(10).unwrap().try_into().unwrap();
            print!("{:2}: {} ", id, height);
        }
        println!();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        let input = include_str!("input/day10/test_input.txt");
        assert_eq!(part_1(input), 1);
    }

    #[test]
    fn test_part_1_2() {
        let input = include_str!("input/day10/test_input2.txt");
        assert_eq!(part_1(input), 36);
    }

    #[test]
    fn run_part_1() {
        let input = include_str!("input/day10/input.txt");
        println!("{}", part_1(input));
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("input/day10/test_input2.txt");
        assert_eq!(part_2(input), 81);
    }

    #[test]
    fn run_part_2() {
        let input = include_str!("input/day10/input.txt");
        println!("{}", part_2(input));
    }
}
