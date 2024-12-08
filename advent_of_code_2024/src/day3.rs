use regex::Regex;
use std::array::from_fn;

const INPUT: &'static str = include_str!("input/day3/input.txt");

fn part_1() -> u32 {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    re.captures_iter(INPUT)
        .map(|cap| {
            println!("Match: {:?}", cap);
            let a = cap.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let b = cap.get(2).unwrap().as_str().parse::<u32>().unwrap();
            a * b
        })
        .sum()
}

fn part_2() -> u32 {
    let re =
        Regex::new(r"(?P<mul>mul\([0-9]{1,3},[0-9]{1,3}\))|(?P<do>do\(\))|(?P<dont>don't\(\))")
            .unwrap();
    re.captures_iter(INPUT)
        .map(|c| {
            println!("Match: {:?}", c);
            let mul_cap = c.name("mul");
            let do_cap = c.name("do");
            let dont_cap = c.name("dont");
            if let Some(mul_cap) = mul_cap {
                let re_1 = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
                let matches = re_1
                    .captures(mul_cap.as_str())
                    .unwrap()
                    .iter()
                    .map(|c| c.unwrap().as_str().parse::<u32>())
                    .collect::<Vec<_>>();
                Node::Mul(matches[1].clone().unwrap(), matches[2].clone().unwrap())
            } else if let Some(_) = do_cap {
                Node::Do
            } else if let Some(_) = dont_cap {
                Node::Dont
            } else {
                panic!("No match found for {:?}", c)
            }
        })
        .fold((0, true), |acc: (u32, bool), node| {
            let (value, enabled) = acc;
            match (enabled, node) {
                (true, Node::Do) => (value, true),
                (true, Node::Dont) => (value, false),
                (true, Node::Mul(a, b)) => (value + a * b, enabled),
                (false, Node::Do) => (value, true),
                (false, _) => (value, false),
            }
        })
        .0
}

enum Node {
    Do,
    Dont,
    Mul(u32, u32),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        // 104049191 too low
        println!("{}", part_1());
    }

    #[test]
    fn test_part_2() {
        println!("{}", part_2());
    }
}
