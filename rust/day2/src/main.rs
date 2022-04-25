use std::{fs::File, path::Path, io::{BufReader, BufRead}};
fn main() {
    let input = "input.txt";
    let file: File = File::open(Path::new(input)).expect("Error opening file...");

    let buff_read = BufReader::new(file).lines();
    let data: Vec<(String, i32)> = buff_read.map(|x| {
        let x_copy = x.as_ref();
        let input_string: Vec<&str> = x_copy.unwrap().split_whitespace()
            .into_iter()
            .collect();
        
        (input_string[0].to_owned(), input_string[1].parse::<i32>().unwrap())
    }).into_iter().collect();

    println!("Final position for part 1, {}", part1(data.to_owned()));
    println!("Final position for part 2, {}", part2(data))

}

fn part1(path: Vec<(String, i32)>) -> i32 {
    struct Position {horizontal: i32, depth: i32}
    let mut pos: Position = Position { horizontal: 0, depth: 0 };

    for (direction, distance) in path.into_iter() {
        match direction.as_str() {
            "forward" => pos.horizontal += distance,
            "up" => pos.depth -= distance,
            "down" => pos.depth += distance,
            _ => panic!("Invalid direction found!")
        }
    }
    pos.depth * pos.horizontal
}

fn part2(path: Vec<(String, i32)>) -> i32 {
    struct Position {horizontal: i32, depth: i32, aim: i32}
    let mut pos: Position = Position { horizontal: 0, depth: 0, aim: 0 };

    for (direction, distance) in path.into_iter() {
        match direction.as_str() {  
            "forward" => {
                pos.horizontal += distance;
                pos.depth += pos.aim * distance
            },
            "up" => pos.aim -= distance,
            "down" => pos.aim += distance,
            _ => panic!("Invalid direction found!")
        }
    }
    pos.depth * pos.horizontal
}
