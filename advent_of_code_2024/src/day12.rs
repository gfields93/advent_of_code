use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs, isize,
    ops::Add,
};

// struct Region {
//     plant_type: char,
//     area: u32,
//     locations: Vec<(usize, usize)>,
// }

type Garden = Vec<char>;

type Vertex = (usize, usize);

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn part_1(input: &str) -> u32 {
    let mut regions: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().chars().count();
    let garden: Garden = input.lines().flat_map(|s| s.chars()).collect();
    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            regions.entry(ch).or_insert_with(Vec::new).push((row, col));
        }
    }

    let mut total = 0;
    for plant in regions.keys() {
        total += flood_fill(rows, cols, &garden, plant);
    }
    total
}

fn part_2(input: &str) -> u32 {
    let mut regions: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().chars().count();
    let garden: Garden = input.lines().flat_map(|s| s.chars()).collect();
    // let cardinals: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            regions.entry(ch).or_insert_with(Vec::new).push((row, col));
        }
    }
    let mut total_cost = 0;
    for plant in regions.keys() {
        let mut new_garden: Vec<i8> = garden
            .iter()
            .map(|&ch| if ch == *plant { 1 } else { 0 })
            .collect();
        total_cost += calulate_cost(&mut new_garden[..], rows, cols);
    }
    total_cost
}

fn calulate_cost(new_garden: &mut [i8], rows: usize, cols: usize) -> u32 {
    let mut cost = 0;
    // let mut visited = HashSet::new();
    for row in 0..rows {
        for col in 0..cols {
            if new_garden[get_index(row, col, rows)] == 1 {
                let (corners, area) = bfs(row, col, rows, cols, new_garden);
                cost += corners * area;
            }
        }
    }
    cost
}

// fn find_sides(row: usize, col: usize, rows: usize, new_garden: &mut [i8]) -> u16 {
//     let mut sides = 0;
//     sides
// }

fn bfs(row: usize, col: usize, rows: usize, cols: usize, new_garden: &mut [i8]) -> (u32, u32) {
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    queue.push_back((row, col));
    new_garden[get_index(row, col, rows)] = -1;
    let mut corners = 0;
    let mut area = 0;
    let cardinals: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    while let Some((r, c)) = queue.pop_front() {
        area += 1;
        corners += count_corners(r, c, rows, cols, new_garden);
        for angle in cardinals {
            let neighbor = (r as isize + angle.0, c as isize + angle.1);
            if get_index_2(neighbor.0, neighbor.1, rows, cols)
                .is_some_and(|idx| new_garden[idx] == 1)
            {
                let idx = get_index_2(neighbor.0, neighbor.1, rows, cols).unwrap();
                queue.push_back((neighbor.0 as usize, neighbor.1 as usize));
                new_garden[idx] = -1;
            }
        }
    }
    dbg!((corners, area))
}

fn count_corners(row: usize, col: usize, rows: usize, cols: usize, new_garden: &[i8]) -> u32 {
    let mut corners = 0;
    let cardinals: [(isize, isize); 5] = [(-1, 0), (0, 1), (1, 0), (0, -1), (-1, 0)];
    for angle in cardinals.windows(2) {
        let neighbor1 = (row as isize + angle[0].0, col as isize + angle[0].1);
        let neighbor2 = (row as isize + angle[1].0, col as isize + angle[1].1);
        let neignbor1_idx = get_index_2(neighbor1.0, neighbor1.1, rows, cols);
        let neignbor2_idx = get_index_2(neighbor2.0, neighbor2.1, rows, cols);
        if (neignbor1_idx.is_none() || *new_garden.get(neignbor1_idx.unwrap()).unwrap() == 0)
            && (neignbor2_idx.is_none() || *new_garden.get(neignbor2_idx.unwrap()).unwrap() == 0)
        {
            corners += 1;
        } else if neignbor1_idx.is_some()
            && (*new_garden.get(neignbor1_idx.unwrap()).unwrap() == 1
                || *new_garden.get(neignbor1_idx.unwrap()).unwrap() == -1)
            && neignbor2_idx.is_some()
            && (*new_garden.get(neignbor2_idx.unwrap()).unwrap() == 1
                || *new_garden.get(neignbor2_idx.unwrap()).unwrap() == -1)
        {
            match (angle[0].0, angle[0].1, angle[1].0, angle[1].1) {
                (-1, 0, 0, 1) => {
                    let neighbor = (row as isize - 1, col as isize + 1);
                    if get_index_2(neighbor.0, neighbor.1, rows, cols).is_some()
                        && *new_garden
                            .get(get_index_2(neighbor.0, neighbor.1, rows, cols).unwrap())
                            .unwrap()
                            == 0
                    {
                        corners += 1;
                    }
                }
                (0, 1, 1, 0) => {
                    let neighbor = (row as isize + 1, col as isize + 1);
                    if get_index_2(neighbor.0, neighbor.1, rows, cols).is_some()
                        && *new_garden
                            .get(get_index_2(neighbor.0, neighbor.1, rows, cols).unwrap())
                            .unwrap()
                            == 0
                    {
                        corners += 1;
                    }
                }
                (1, 0, 0, -1) => {
                    let neighbor = (row as isize + 1, col as isize - 1);
                    if get_index_2(neighbor.0, neighbor.1, rows, cols).is_some()
                        && *new_garden
                            .get(get_index_2(neighbor.0, neighbor.1, rows, cols).unwrap())
                            .unwrap()
                            == 0
                    {
                        corners += 1;
                    }
                }
                (0, -1, -1, 0) => {
                    let neighbor = (row as isize - 1, col as isize - 1);
                    if get_index_2(neighbor.0, neighbor.1, rows, cols).is_some()
                        && *new_garden
                            .get(get_index_2(neighbor.0, neighbor.1, rows, cols).unwrap())
                            .unwrap()
                            == 0
                    {
                        corners += 1;
                    }
                }
                _ => {}
            }
        }
    }
    corners
}

fn get_index_2(row: isize, col: isize, rows: usize, cols: usize) -> Option<usize> {
    if row >= 0 && row < rows as isize && col >= 0 && col < cols as isize {
        Some((row * rows as isize + col) as usize)
    } else {
        None
    }
}

// fn is_border(row: usize, col: usize, rows: usize, new_garden: &[i8]) -> bool {
//     if new_garden.get(get_index(row, col, rows)).is_none()
//         || *new_garden.get(get_index(row, col, rows)).unwrap() == 0
//     {
//         return true;
//     }
//     false
// }

fn flood_fill(rows: usize, cols: usize, garden: &Vec<char>, plant: &char) -> u32 {
    let mut filled = vec![false; rows * cols];
    let cardinals: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut regions_val = 0;
    let mut region_id = 1;
    let mut region_map = HashMap::new();
    for row in 0..rows {
        for col in 0..cols {
            if garden[get_index(row, col, rows)] != *plant || filled[get_index(row, col, rows)] {
                continue;
            }

            let mut queue = VecDeque::new();
            let mut border_found = 0;
            let mut area = 1;
            filled[get_index(row, col, rows)] = true;

            queue.push_back((row, col));
            while let Some((r, c)) = queue.pop_front() {
                region_map.insert(get_index(r, c, rows), region_id);
                for (dr, dc) in cardinals.iter() {
                    let nr = r as isize + dr;
                    let nc = c as isize + dc;
                    if nr < 0
                        || nr >= rows.try_into().unwrap()
                        || nc < 0
                        || nc >= cols.try_into().unwrap()
                    {
                        border_found += 1;
                        continue;
                    }
                    let idx = get_index(nr as usize, nc as usize, rows);
                    if garden.get(idx).is_some_and(|f| f != plant) {
                        border_found += 1;
                        continue;
                    }
                    let filled_space = filled.get(idx);
                    if !filled_space.is_some_and(|f| *f) {
                        filled[idx] = true;
                        area += 1;
                        queue.push_back((nr as usize, nc as usize));
                    }
                }
            }
            println!(
                "Region {} for plant {} found: {} square tiles, {} border tiles, total cost {}",
                region_id,
                *plant,
                area,
                border_found,
                area * border_found
            );
            region_id += 1;
            regions_val += area * border_found;
        }
    }
    print_map(rows, cols, &filled[..], garden, plant, &region_map).unwrap();
    regions_val
}

fn print_map(
    rows: usize,
    cols: usize,
    filled: &[bool],
    garden: &[char],
    plant: &char,
    region_map: &HashMap<usize, i32>,
) -> std::io::Result<()> {
    let mut map_str = String::new();
    for row in 0..rows {
        for col in 0..cols {
            let idx = get_index(row, col, rows);
            if region_map.contains_key(&idx) {
                map_str.push_str(&format!("{:?}", region_map.get(&idx).unwrap()));
            } else {
                map_str.push(' ');
            }
        }
        map_str.push('\n');
    }
    let file_path = format!("src/output/day12/{}_plant_map", *plant);
    fs::write(file_path, map_str)
}

fn get_index(row: usize, col: usize, rows: usize) -> usize {
    row * rows + col
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_with_input1() {
        let input = include_str!("input/day12/test_input1.txt");
        assert_eq!(part_1(input), 140);
    }

    #[test]
    fn test_part_1_with_input2() {
        let input = include_str!("input/day12/test_input2.txt");
        assert_eq!(part_1(input), 772);
    }

    #[test]
    fn test_part_1_with_input3() {
        let input = include_str!("input/day12/test_input3.txt");
        assert_eq!(part_1(input), 1930);
    }

    #[test]
    fn test_part_1_with_input4() {
        let input = include_str!("input/day12/test_input4.txt");
        assert_eq!(part_1(input), 16);
    }

    #[test]
    fn run_part_1() {
        let input = include_str!("input/day12/input.txt");
        println!("{}", part_1(input));
    }

    #[test]
    fn test_part_2_with_input1() {
        let input = include_str!("input/day12/test_input1.txt");
        println!("{}", part_2(input));
    }

    #[test]
    fn test_part_2_with_input2() {
        let input = include_str!("input/day12/test_input2.txt");
        println!("{}", part_2(input));
    }

    #[test]
    fn run_part_2_with_input() {
        let input = include_str!("input/day12/input.txt");
        println!("{}", part_2(input));
    }
}
