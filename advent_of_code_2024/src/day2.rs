pub(crate) mod day2 {
    use std::{iter::from_fn, slice::Windows, str::from_utf8};

    const INPUT: &'static [u8] = include_bytes!("input/day2/input.txt");

    pub fn parse_line() -> std::iter::FromFn<impl FnMut() -> Option<Vec<u32>>> {
        let mut iter = INPUT.split(|&b| b == b'\n');

        from_fn(move || {
            let maybe_line = iter.next();
            let line = match maybe_line {
                Some(line) => line,
                None => return None,
            };
            let mut split = line.split(|&b| b == b' ');
            let mut numbers = Vec::new();
            while let Some(num) = split.next() {
                let convert_digit = from_utf8(num);
                if let Ok(digit) = convert_digit {
                    numbers.push(digit.parse().unwrap());
                } else {
                    return None;
                }
            }
            Some(numbers)
        })
    }

    pub fn part_1() -> u32 {
        const MAX: u32 = 3;
        const MIN: u32 = 1;
        let mut lines = parse_line();
        let mut safe = 0;
        let mut _unsafe = false;
        let mut ordering = Ordering::Increasing;
        while let Some(line) = lines.next() {
            if line[0] < line[1] {
                ordering = Ordering::Increasing;
            } else {
                ordering = Ordering::Decreasing;
            }
            for i in 0..line.len() - 1 {
                let distance = line[i].abs_diff(line[i + 1]);
                let left = line[i];
                let right = line[i + 1];
                match ordering {
                    Ordering::Increasing => {
                        if left >= right || (distance > MAX || distance < MIN) {
                            _unsafe = true;
                            break;
                        }
                    }
                    Ordering::Decreasing => {
                        if left <= right || (distance > MAX || distance < MIN) {
                            _unsafe = true;
                            break;
                        }
                    }
                }
            }
            if !_unsafe {
                println!("Line is safe: {:?}", line);
                safe += 1;
            } else {
                println!("Line is unsafe: {:?}", line);
            }
            _unsafe = false;
        }
        safe
    }

    pub fn part_2() -> u32 {
        let mut lines = parse_line();
        let mut safe = 0;
        while let Some(line) = lines.next() {
            for idx in 0..line.len() {
                let mut list_segment = line.clone();
                list_segment.remove(idx);
                match fun_name(&list_segment[..]) {
                    Safety::Safe => {
                        safe += 1;
                        break;
                    }
                    Safety::Unsafe => {}
                }
            }
        }
        safe
    }

    pub fn fun_name(line: &[u32]) -> Safety {
        if !is_sorted(line) {
            return Safety::Unsafe;
        }

        if !line
            .windows(2)
            .all(|window| within_range(window[0].abs_diff(window[1])))
        {
            return Safety::Unsafe;
        }
        Safety::Safe
    }
    fn is_sorted(list: &[u32]) -> bool {
        list.windows(2).all(|window| window[0] < window[1])
            || list
                .into_iter()
                .rev()
                .collect::<Vec<&u32>>()
                .windows(2)
                .all(|window| window[0] < window[1])
    }

    fn within_range(n: u32) -> bool {
        const MAX: u32 = 3;
        const MIN: u32 = 1;
        n >= MIN && n <= MAX
    }

    enum Ordering {
        Increasing,
        Decreasing,
    }

    enum Trend {
        Ascending,
        Descending,
        Flat,
        Mixed,
    }

    #[derive(Debug, PartialEq)]
    pub enum Safety {
        Safe,
        Unsafe,
    }
}

#[cfg(test)]
mod tests {
    use super::day2::{parse_line, part_2, Safety};

    #[test]
    fn test_parse_line() {
        let mut lines = parse_line();
        let first_line = lines.next().unwrap();
        assert_eq!(first_line, vec![7, 6, 4, 2, 1]);

        let second_line = lines.next().unwrap();
        assert_eq!(second_line, vec![1, 2, 7, 8, 9]);

        let third_line = lines.next().unwrap();
        assert_eq!(third_line, vec![9, 7, 6, 2, 1])
    }

    #[test]
    fn test_part_2() {
        println!("{}", part_2());
    }
}
