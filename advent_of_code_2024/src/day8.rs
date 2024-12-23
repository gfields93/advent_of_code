use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    ops::{Add, Mul, Sub},
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]

struct Antenna(i32, i32);
type Antinode = Antenna;

struct Data {
    antennas: HashMap<char, Vec<Antenna>>,
    antinodes: HashMap<char, HashSet<Antinode>>,
    rows: i32,
    cols: i32,
}

impl Data {
    pub fn check_bounds(&self, antenna: &Antenna) -> bool {
        antenna.0 >= 0 && antenna.0 < self.rows && antenna.1 >= 0 && antenna.1 < self.cols
    }
}

impl Antenna {
    pub fn from(antenna: Antenna) -> Antenna {
        antenna
    }
}
impl Add for Antenna {
    type Output = Antenna;
    fn add(self, other: Antenna) -> Self::Output {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl Sub for Antenna {
    type Output = Antenna;
    fn sub(self, other: Antenna) -> Antenna {
        Self(self.0 - other.0, self.1 - other.1)
    }
}

impl Mul<i32> for Antenna {
    type Output = Antenna;
    fn mul(self, scalar: i32) -> Antenna {
        Antenna(self.0 * scalar, self.1 * scalar)
    }
}

fn parse_input(input: &str) -> Data {
    let mut map = HashMap::new();
    let mut antinodes: HashMap<char, HashSet<Antinode>> = HashMap::new();
    let rows = input.lines().count() as i32;
    let cols = input.lines().next().unwrap().chars().count() as i32;

    for (row, line) in input.lines().enumerate() {
        for (column, c) in line.chars().enumerate() {
            if c != '.' {
                map.entry(c)
                    .or_insert_with(Vec::new)
                    .push(Antenna(row as i32, column as i32));
                if !antinodes.contains_key(&c) {
                    antinodes.insert(c, HashSet::new());
                }
            }
        }
    }
    Data {
        antennas: map,
        antinodes,
        rows,
        cols,
    }
}

fn part_1(input: &str) -> u32 {
    let mut data = parse_input(input);
    let filled_positions: HashSet<Antenna> =
        data.antennas
            .values()
            .fold(HashSet::new(), |mut acc, antenna| {
                acc.extend(antenna.clone());
                acc
            });
    let mut antinodes = data.antinodes.clone();

    for (key, locations) in data.antennas.iter() {
        for pos_a in locations {
            for pos_b in locations {
                if pos_a == pos_b {
                    continue;
                }
                let dist = *pos_a - *pos_b;
                antinodes
                    .get_mut(key)
                    .and_then(|set| set.insert(Antinode::from(*pos_a + dist)).then_some(()));
            }
        }
    }
    println!("{:?}", &antinodes);
    antinodes
        .values()
        .map(|set| {
            let mut count = 0;
            for &pos_b in set {
                if data.check_bounds(&pos_b) && !filled_positions.contains(&pos_b) {
                    count += 1;
                }
            }
            count as u32
        })
        .sum()
}

fn part_2(input: &str) -> i32 {
    let mut data = parse_input(input);
    let mut filled_positions: HashSet<Antenna> =
        data.antennas
            .values()
            .fold(HashSet::new(), |mut acc, antenna| {
                acc.extend(antenna.clone());
                acc
            });
    let mut antinodes = data.antinodes.clone();
    for (location, positions) in data.antennas.iter() {
        for antenna_a in positions {
            for antenna_b in positions {
                if antenna_a == antenna_b {
                    continue;
                }
                let dist = *antenna_a - *antenna_b;
                let mut scalar = 1;
                while data.check_bounds(&Antinode::from(*antenna_a + dist * scalar)) {
                    antinodes.get_mut(location).and_then(|set| {
                        set.insert(Antinode::from(*antenna_a + dist * scalar))
                            .then_some(())
                    });
                    scalar += 1;
                }
            }
        }
    }
    println!("{:?}", &antinodes);
    for set in antinodes.values() {
        filled_positions.extend(set);
    }
    println!("Filled positions: {:?}", &filled_positions);
    filled_positions.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_with_input1() {
        let input = include_str!("input/day8/test_input1.txt");
        println!("{}", part_1(input));
    }

    #[test]
    fn test_part_1_with_input2() {
        let input = include_str!("input/day8/test_input2.txt");
        assert_eq!(4, part_1(input));
    }

    #[test]
    fn run_part_1_with_input3() {
        let input = include_str!("input/day8/test_input3.txt");
        assert_eq!(3, part_1(input));
    }

    #[test]
    fn run_part_1_with_input4() {
        let input = include_str!("input/day8/test_input4.txt");
        assert_eq!(14, part_1(input));
    }

    #[test]
    fn run_part_1_with_real_input() {
        let input = include_str!("input/day8/input.txt");
        println!("{}", part_1(input));
    }

    #[test]
    fn test_part_2_with_input1() {
        let input = include_str!("input/day8/test_input1.txt");
        println!("{}", part_2(input));
    }

    #[test]
    fn test_part_2_with_real_input() {
        let input = include_str!("input/day8/input.txt");
        println!("{}", part_2(input));
    }
}
