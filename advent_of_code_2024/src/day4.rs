struct WordSearch {
    card: Vec<Vec<char>>,
}

impl From<&str> for WordSearch {
    fn from(card: &str) -> Self {
        let lines: Vec<&str> = card.lines().collect();
        WordSearch {
            card: lines.iter().map(|line| line.chars().collect()).collect(),
        }
    }
}

impl WordSearch {
    /* Methods for part 1   */
    fn check_for_xmas(&self, row: usize, col: usize) -> Option<usize> {
        let mut found = 0;
        if self.check_down_left_diagonal(row, col) {
            found += 1;
        }
        if self.check_down_right_diagonal(row, col) {
            found += 1;
        }
        if self.check_left_horizontal(row, col) {
            found += 1;
        }
        if self.check_right_horizontal(row, col) {
            found += 1;
        }
        if self.check_up_left_diagonal(row, col) {
            found += 1;
        }
        if self.check_up_right_diagonal(row, col) {
            found += 1;
        }
        if self.check_up_vertical(row, col) {
            found += 1;
        }
        if self.check_down_vertical(row, col) {
            found += 1;
        }
        if found > 0 {
            Some(found)
        } else {
            None
        }
    }

    fn check_left_horizontal(&self, row: usize, col: usize) -> bool {
        let m_option = self.get_coordinate((Some(row), col.checked_add(1)));
        let a_option = self.get_coordinate((Some(row), col.checked_add(2)));
        let s_option = self.get_coordinate((Some(row), col.checked_add(3)));
        m_option.is_some_and(|c| c == 'M')
            && a_option.is_some_and(|c| c == 'A')
            && s_option.is_some_and(|c| c == 'S')
    }

    fn check_right_horizontal(&self, row: usize, col: usize) -> bool {
        let m_option = self.get_coordinate((Some(row), col.checked_sub(1)));
        let a_option = self.get_coordinate((Some(row), col.checked_sub(2)));
        let s_option = self.get_coordinate((Some(row), col.checked_sub(3)));
        m_option.is_some_and(|c| c == 'M')
            && a_option.is_some_and(|c| c == 'A')
            && s_option.is_some_and(|c| c == 'S')
    }

    fn check_down_left_diagonal(&self, row: usize, col: usize) -> bool {
        let m_option = self.get_coordinate((row.checked_sub(1), col.checked_sub(1)));
        let a_option = self.get_coordinate((row.checked_sub(2), col.checked_sub(2)));
        let s_option = self.get_coordinate((row.checked_sub(3), col.checked_sub(3)));
        m_option.is_some_and(|c| c == 'M')
            && a_option.is_some_and(|c| c == 'A')
            && s_option.is_some_and(|c| c == 'S')
    }

    fn check_up_vertical(&self, row: usize, col: usize) -> bool {
        let m_option = self.get_coordinate((row.checked_add(1), Some(col)));
        let a_option = self.get_coordinate((row.checked_add(2), Some(col)));
        let s_option = self.get_coordinate((row.checked_add(3), Some(col)));
        m_option.is_some_and(|c| c == 'M')
            && a_option.is_some_and(|c| c == 'A')
            && s_option.is_some_and(|c| c == 'S')
    }

    fn check_down_vertical(&self, row: usize, col: usize) -> bool {
        let m_option = self.get_coordinate((row.checked_sub(1), Some(col)));
        let a_option = self.get_coordinate((row.checked_sub(2), Some(col)));
        let s_option = self.get_coordinate((row.checked_sub(3), Some(col)));
        m_option.is_some_and(|c| c == 'M')
            && a_option.is_some_and(|c| c == 'A')
            && s_option.is_some_and(|c| c == 'S')
    }

    fn check_down_right_diagonal(&self, row: usize, col: usize) -> bool {
        let m = self.get_coordinate((row.checked_sub(1), col.checked_add(1)));
        let a = self.get_coordinate((row.checked_sub(2), col.checked_add(2)));
        let s = self.get_coordinate((row.checked_sub(3), col.checked_add(3)));
        m.is_some_and(|c| c == 'M') && a.is_some_and(|c| c == 'A') && s.is_some_and(|c| c == 'S')
    }

    fn check_up_left_diagonal(&self, row: usize, col: usize) -> bool {
        let m = self.get_coordinate((row.checked_add(1), col.checked_sub(1)));
        let a = self.get_coordinate((row.checked_add(2), col.checked_sub(2)));
        let s = self.get_coordinate((row.checked_add(3), col.checked_sub(3)));
        m.is_some_and(|c| c == 'M') && a.is_some_and(|c| c == 'A') && s.is_some_and(|c| c == 'S')
    }

    fn check_up_right_diagonal(&self, row: usize, col: usize) -> bool {
        let m = self.get_coordinate((row.checked_add(1), col.checked_add(1)));
        let a = self.get_coordinate((row.checked_add(2), col.checked_add(2)));
        let s = self.get_coordinate((row.checked_add(3), col.checked_add(3)));
        m.is_some_and(|c| c == 'M') && a.is_some_and(|c| c == 'A') && s.is_some_and(|c| c == 'S')
    }

    /* End methods for part 1 */

    fn check_top(&self, row: usize, col: usize, expected: char) -> bool {
        if let (Some(c1), Some(c2)) = (
            self.get_coordinate((row.checked_add(1), col.checked_sub(1))),
            self.get_coordinate((row.checked_add(1), col.checked_add(1))),
        ) {
            c1 == expected && c2 == expected
        } else {
            false
        }
    }

    fn check_bottom(&self, row: usize, col: usize, expected: char) -> bool {
        if let (Some(c1), Some(c2)) = (
            self.get_coordinate((row.checked_sub(1), col.checked_sub(1))),
            self.get_coordinate((row.checked_sub(1), col.checked_add(1))),
        ) {
            c1 == expected && c2 == expected
        } else {
            false
        }
    }

    fn check_left(&self, row: usize, col: usize, expected: char) -> bool {
        if let (Some(c1), Some(c2)) = (
            self.get_coordinate((row.checked_sub(1), col.checked_sub(1))),
            self.get_coordinate((row.checked_add(1), col.checked_sub(1))),
        ) {
            c1 == expected && c2 == expected
        } else {
            false
        }
    }

    fn check_right(&self, row: usize, col: usize, expected: char) -> bool {
        if let (Some(c1), Some(c2)) = (
            self.get_coordinate((row.checked_sub(1), col.checked_add(1))),
            self.get_coordinate((row.checked_add(1), col.checked_add(1))),
        ) {
            c1 == expected && c2 == expected
        } else {
            false
        }
    }

    fn check_for_mas(&self, row: usize, col: usize) -> bool {
        self.check_top(row, col, 'M') && self.check_bottom(row, col, 'S')
            || self.check_top(row, col, 'S') && self.check_bottom(row, col, 'M')
            || self.check_left(row, col, 'M') && self.check_right(row, col, 'S')
            || self.check_left(row, col, 'S') && self.check_right(row, col, 'M')
    }

    fn get_coordinate(&self, (y_maybe, x_maybe): (Option<usize>, Option<usize>)) -> Option<char> {
        if x_maybe.is_some() && y_maybe.is_some() {
            let x = x_maybe.unwrap();
            let y = y_maybe.unwrap();
            if let Some(row) = self.card.get(x) {
                if let Some(c) = row.get(y) {
                    Some(*c)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}

fn part_1(card: &str) -> usize {
    let search = WordSearch::from(card);
    let mut count = 0;
    for row in 0..search.card.len() {
        for col in 0..search.card[row].len() {
            if search.get_coordinate((Some(row), Some(col))).unwrap() == 'X' {
                if let Some(found) = search.check_for_xmas(row, col) {
                    count += found;
                }
            }
        }
    }
    count
}

fn part_2(card: &str) -> usize {
    let search = WordSearch::from(card);
    let mut count = 0;
    for row in 0..search.card.len() {
        for col in 0..search.card[row].len() {
            if search.get_coordinate((Some(row), Some(col))).unwrap() == 'A'
                && search.check_for_mas(row, col)
            {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        print!("{}", part_1(include_str!("input/day4/input.txt")));
    }

    #[test]
    fn test_part_2() {
        print!("{}", part_2(include_str!("input/day4/input.txt")));
    }
}
