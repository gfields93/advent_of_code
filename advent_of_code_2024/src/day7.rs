fn check_equation(target: u128, current: u128, slice: &[u128]) -> bool {
    if slice.len() == 0 && current == target {
        return true;
    }
    if slice.len() == 0 && current != target {
        return false;
    }

    let (top, rest) = slice.split_first().unwrap();
    check_equation(target, current * *top, rest) || check_equation(target, current + *top, rest)
}

fn check_equation_2(target: u128, current: u128, slice: &[u128]) -> bool {
    if slice.len() == 0 && current == target {
        return true;
    }
    if slice.len() == 0 && current != target {
        return false;
    }

    let (top, rest) = slice.split_first().unwrap();
    check_equation_2(target, current * *top, rest)
        || check_equation_2(target, current + *top, rest)
        || check_equation_2(target, concat(current, *top), rest)
}

fn concat(a: u128, b: u128) -> u128 {
    format!("{}{}", a, b).parse().unwrap()
}
fn part_1(input: &str) -> u128 {
    input
        .lines()
        .map(|line| {
            let (result, test_values) = line.split_once(": ").unwrap();
            let target: u128 = result.parse().unwrap();
            let test_values: Vec<u128> =
                test_values.split(" ").map(|s| s.parse().unwrap()).collect();
            if check_equation(target, 0, &test_values) {
                target
            } else {
                0
            }
        })
        .sum()
}

fn part_2(input: &str) -> u128 {
    input
        .lines()
        .map(|line| {
            let (result, test_values) = line.split_once(": ").unwrap();
            let target: u128 = result.parse().unwrap();
            let test_values: Vec<u128> =
                test_values.split(" ").map(|s| s.parse().unwrap()).collect();
            if check_equation_2(target, 0, &test_values) {
                target
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let content = include_str!("input/day7/test_input.txt");
        assert_eq!(3749, part_1(content));
    }

    #[test]
    fn run_part_1() {
        let content = include_str!("input/day7/input.txt");
        println!("{}", part_1(content));
    }

    #[test]
    fn test_part_2() {
        let content = include_str!("input/day7/test_input.txt");
        assert_eq!(11387, part_2(content));
    }

    #[test]
    fn run_part_2() {
        let content = include_str!("input/day7/input.txt");
        println!("{}", part_2(content));
    }
}
