use std::{cell::RefCell, collections::HashMap, rc::Rc};

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
type Stone = (u64, u16);

fn part_1(input: &str) -> u64 {
    let mut numbers = input
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u64>>();
    for blink in 0..25 {
        let mut i = 0;
        let mut len = numbers.len();
        while i < len {
            let current = numbers.get_mut(i).unwrap();
            if *current == 0 {
                *current = 1;
                i += 1;
            } else if format!("{}", current).chars().count() % 2 == 0 {
                let current_str = format!("{}", current.clone());
                // let copy = current_str.clone();
                let (front, back) = current_str.split_at(current_str.len() / 2);
                *current = front.parse().unwrap();
                numbers.insert(i + 1, back.parse().unwrap());
                i += 2;
                len += 1;
            } else {
                *current *= 2024;
                i += 1;
            }
        }
    }
    numbers.len() as u64
}

fn part_2(input: &str) -> u64 {
    let numbers = input
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u64>>();

    let memo: Rc<RefCell<HashMap<Stone, u64>>> = Rc::new(RefCell::new(HashMap::new()));
    numbers
        .into_iter()
        .map(|n| count_stones((n, 75), memo.clone()))
        .sum()
}

fn count_stones((n, b): Stone, memo: Rc<RefCell<HashMap<Stone, u64>>>) -> u64 {
    let memo = Rc::clone(&memo);
    if b == 0 {
        return 1;
    } else if memo.borrow().contains_key(&(n, b)) {
        return *memo.borrow().get(&(n, b)).unwrap();
    }

    if n == 0 {
        return count_stones((1, b - 1), memo);
    } else if count_digits(n) % 2 == 0 {
        let (front, back) = split_into_two(n, (count_digits(n) / 2) as usize);
        let stone_count_1 = count_stones((front, b - 1), Rc::clone(&memo));
        let stone_count_2 = count_stones((back, b - 1), Rc::clone(&memo));
        memo.borrow_mut()
            .insert((n, b), stone_count_1 + stone_count_2);
        return stone_count_1 + stone_count_2;
    } else {
        return count_stones((n * 2024, b - 1), memo);
    }
}

// Function that counts the number of digits in a number without using a string method
fn count_digits(n: u64) -> u32 {
    if n == 0 {
        1
    } else {
        n.ilog10() as u32 + 1
    }
}

fn split_into_two(n: u64, length: usize) -> (u64, u64) {
    let n_str = n.to_string();
    let (front, back) = n_str.split_at(length);
    (front.parse().unwrap(), back.parse().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        let input = include_str!("input/day11/test_input.txt");
        assert_eq!(part_1(input), 55312);
    }

    #[test]
    fn run_part_1() {
        let input = include_str!("input/day11/input.txt");
        println!("{}", part_1(input));
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("input/day11/test_input.txt");
        assert_eq!(part_2(input), 55312);
    }

    #[test]
    fn run_part_2() {
        let input = include_str!("input/day11/input.txt");
        println!("{}", part_2(input));
    }

    #[test]
    fn test_count_digits() {
        assert_eq!(count_digits(0), 1);
        assert_eq!(count_digits(1), 1);
        assert_eq!(count_digits(10), 2);
        assert_eq!(count_digits(100), 3);
        assert_eq!(count_digits(1000), 4);
    }
}
