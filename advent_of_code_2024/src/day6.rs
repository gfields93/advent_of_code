use std::collections::HashSet;

use regex::Regex;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq)]
enum Status {
    Available,
    Traveled,
    Blocked,
}

struct Board {
    board: Vec<Status>,
    copy: Vec<Status>,
    direction: Direction,
    start_dir: Direction,
    rows: usize,
    cols: usize,
    start_pos: Position,
    guard_pos: Position,
    path: Vec<Position>,
    // hare: Guard,
    // tortoise: Guard,
}

struct Guard {
    pos: Position,
    direction: Direction,
}

impl Guard {
    fn new(pos: Position, direction: Direction) -> Self {
        Guard { pos, direction }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position(usize, usize);

impl Board {
    fn move_up(&mut self) -> Option<Status> {
        if self.guard_pos.0.checked_sub(1).is_none()
            || self
                .copy
                .get((self.guard_pos.0 - 1) * self.rows + self.guard_pos.1)
                .is_none()
        {
            return None;
        }
        if self.copy[(self.guard_pos.0 - 1) * self.rows + self.guard_pos.1] != Status::Blocked {
            self.guard_pos.0 -= 1;
            self.board[self.guard_pos.0 * self.rows + self.guard_pos.1] = Status::Traveled;
            return Some(Status::Available);
        }
        Some(Status::Blocked)
    }

    fn move_down(&mut self) -> Option<Status> {
        if self
            .copy
            .get((self.guard_pos.0 + 1) * self.rows + self.guard_pos.1)
            .is_none()
        {
            return None;
        }
        if self.copy[(self.guard_pos.0 + 1) * self.rows + self.guard_pos.1] != Status::Blocked {
            self.guard_pos.0 += 1;
            self.copy[self.guard_pos.0 * self.rows + self.guard_pos.1] = Status::Traveled;
            return Some(Status::Available);
        }
        Some(Status::Blocked)
    }

    fn move_left(&mut self) -> Option<Status> {
        if self.guard_pos.1.checked_sub(1).is_none()
            || self
                .copy
                .get(self.guard_pos.0 * self.rows + self.guard_pos.1 - 1)
                .is_none()
        {
            return None;
        }
        if self.copy[self.guard_pos.0 * self.rows + self.guard_pos.1 - 1] != Status::Blocked {
            self.guard_pos.1 -= 1;
            self.copy[self.guard_pos.0 * self.rows + self.guard_pos.1] = Status::Traveled;
            return Some(Status::Available);
        }
        Some(Status::Blocked)
    }

    fn move_right(&mut self) -> Option<Status> {
        if self
            .copy
            .get(self.guard_pos.0 * self.rows + self.guard_pos.1 + 1)
            .is_none()
        {
            return None;
        }
        if self.copy[self.guard_pos.0 * self.rows + self.guard_pos.1 + 1] != Status::Blocked {
            self.guard_pos.1 += 1;
            self.copy[self.guard_pos.0 * self.rows + self.guard_pos.1] = Status::Traveled;
            return Some(Status::Available);
        }
        Some(Status::Blocked)
    }

    fn rotate_direction(&mut self) {
        match self.direction {
            Direction::Up => self.direction = Direction::Right,
            Direction::Right => self.direction = Direction::Down,
            Direction::Down => self.direction = Direction::Left,
            Direction::Left => self.direction = Direction::Up,
        }
    }

    fn _move(&mut self) -> Option<Status> {
        let status = match self.direction {
            Direction::Up => self.move_up(),
            Direction::Down => self.move_down(),
            Direction::Left => self.move_left(),
            Direction::Right => self.move_right(),
        };
        status
    }

    fn move_2(&self, hare: &mut Guard, tortoise: &mut Guard) -> Option<(Position, Position)> {
        let mut step = 0;
        while step < 1 {
            if let Some(status) = self.move_guard(tortoise) {
                match status {
                    Status::Traveled | Status::Available => step += 1,
                    Status::Blocked => rotate_guard(tortoise),
                }
            } else {
                return None;
            }
        }

        step = 0;
        while step < 2 {
            if let Some(status) = self.move_guard(hare) {
                match status {
                    Status::Traveled | Status::Available => step += 1,
                    Status::Blocked => rotate_guard(hare),
                }
            } else {
                return None;
            }
        }
        Some((hare.pos, tortoise.pos))
    }

    fn move_guard(&self, guard: &mut Guard) -> Option<Status> {
        let pos = &mut guard.pos;
        let direction = guard.direction;
        match direction {
            Direction::Up => {
                if pos.0.checked_sub(1).is_none()
                    || self.copy.get((pos.0 - 1) * self.rows + pos.1).is_none()
                {
                    return None;
                }
                if self.copy[(pos.0 - 1) * self.rows + pos.1] != Status::Blocked {
                    pos.0 -= 1;
                    return Some(Status::Traveled);
                }
                return Some(Status::Blocked);
            }
            Direction::Down => {
                if (pos.0 + 1 >= self.rows)
                    || self.copy.get((pos.0 + 1) * self.rows + pos.1).is_none()
                {
                    return None;
                }
                if self.copy[(pos.0 + 1) * self.rows + pos.1] != Status::Blocked {
                    pos.0 += 1;
                    return Some(Status::Traveled);
                }
                return Some(Status::Blocked);
            }
            Direction::Left => {
                if pos.1.checked_sub(1).is_none()
                    || self.copy.get(pos.0 * self.rows + pos.1 - 1).is_none()
                {
                    return None;
                }
                if self.copy[pos.0 * self.rows + pos.1 - 1] != Status::Blocked {
                    pos.1 -= 1;
                    return Some(Status::Traveled);
                }
                return Some(Status::Blocked);
            }
            Direction::Right => {
                if (pos.1 + 1) >= self.cols
                    || self.copy.get(pos.0 * self.rows + pos.1 + 1).is_none()
                {
                    return None;
                }
                if self.copy[pos.0 * self.rows + pos.1 + 1] != Status::Blocked {
                    pos.1 += 1;
                    return Some(Status::Traveled);
                }
                return Some(Status::Blocked);
            }
        }
    }

    pub fn part_2(&mut self) -> u32 {
        self.store_path();
        let set: HashSet<Position> = HashSet::from_iter(self.path.iter().cloned());
        self.copy = self.board.clone();
        let direction = self.start_dir;
        let mut loops_found = 0;

        for pos in set.iter() {
            let mut hare: Guard = Guard::new(self.start_pos, direction);
            let mut tortoise: Guard = Guard::new(self.start_pos, direction);
            self.copy[pos.0 * self.rows + pos.1] = Status::Blocked;
            let mut equals = 0;
            while let Some((pos_a, pos_b)) = self.move_2(&mut hare, &mut tortoise) {
                if pos_a == pos_b {
                    equals += 1;
                    if equals == 20 {
                        loops_found += 1;
                        println!("Position: ({}, {})", pos.0, pos.1);
                        break;
                    }
                }
            }
            self.copy.clone_from_slice(&self.board);
        }

        loops_found
    }
    pub fn find_path(&mut self) {
        self.copy.clone_from_slice(&self.board);
        let mut steps = 0;
        while let Some(status) = self._move() {
            match status {
                Status::Available => {
                    steps += 1;
                }
                Status::Traveled => continue,
                Status::Blocked => self.rotate_direction(),
            }
        }
    }

    pub fn store_path(&mut self) {
        self.copy.clone_from_slice(&self.board);
        // self.path.push(self.start_pos);
        while let Some(status) = self._move() {
            match status {
                Status::Available | Status::Traveled => {
                    self.path.push(self.guard_pos);
                }
                Status::Blocked => self.rotate_direction(),
            }
        }
    }

    fn print_graph_with_tortoise_and_hare(&self, tortoise: &Guard, hare: &Guard) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                match self.copy[row * self.cols + col] {
                    Status::Blocked => print!("#"),
                    Status::Available | Status::Traveled => {
                        if tortoise.pos == Position(row, col) {
                            print!("T")
                        } else if hare.pos == Position(row, col) {
                            print!("H")
                        } else {
                            print!(".")
                        }
                    }
                }
            }
            println!();
        }
        println!(
            "Tortoise: ({}, {}) Hare: ({}, {})",
            tortoise.pos.0, tortoise.pos.1, hare.pos.0, hare.pos.1
        );
    }
}

fn rotate_guard(guard: &mut Guard) {
    match guard.direction {
        Direction::Up => guard.direction = Direction::Right,
        Direction::Right => guard.direction = Direction::Down,
        Direction::Down => guard.direction = Direction::Left,
        Direction::Left => guard.direction = Direction::Up,
    }
}

impl From<&str> for Board {
    fn from(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        let rows = lines.len();
        let cols = lines[0].chars().count();
        let mut board = vec![Status::Available; rows * cols];
        let mut start_pos = Position(0, 0);
        let mut guard_pos = Position(0, 0);
        let mut tortoise = Position(0, 0);
        let mut hare = Position(0, 0);
        let mut direction = Direction::Up;

        for (row, line) in lines.iter().enumerate() {
            for (col, c) in line.chars().enumerate() {
                match c {
                    '#' => board[rows * row + col] = Status::Blocked,
                    '^' => {
                        board[rows * row + col] = Status::Traveled;
                        direction = Direction::Up;
                        initial_positions(
                            row,
                            col,
                            &mut start_pos,
                            &mut guard_pos,
                            &mut tortoise,
                            &mut hare,
                        );
                    }
                    'v' => {
                        board[rows * row + col] = Status::Traveled;
                        direction = Direction::Down;
                        initial_positions(
                            row,
                            col,
                            &mut start_pos,
                            &mut guard_pos,
                            &mut tortoise,
                            &mut hare,
                        );
                    }
                    '<' => {
                        board[rows * row + col] = Status::Traveled;
                        direction = Direction::Left;
                        initial_positions(
                            row,
                            col,
                            &mut start_pos,
                            &mut guard_pos,
                            &mut tortoise,
                            &mut hare,
                        );
                    }
                    '>' => {
                        board[rows * row + col] = Status::Traveled;
                        direction = Direction::Right;
                        initial_positions(
                            row,
                            col,
                            &mut start_pos,
                            &mut guard_pos,
                            &mut tortoise,
                            &mut hare,
                        );
                    }
                    _ => {}
                }
            }
        }

        Board {
            board: board.clone(),
            direction: direction,
            start_dir: direction,
            rows,
            cols,
            guard_pos: Position(guard_pos.0, guard_pos.1),
            start_pos: Position(start_pos.0, start_pos.1),
            path: Vec::new(),
            copy: board,
            // tortoise: Guard {
            //     direction,
            //     pos: start_pos,
            // },
            // hare: Guard {
            //     direction,
            //     pos: start_pos,
            // },
        }
    }
}

fn initial_positions(
    row: usize,
    col: usize,
    start_pos: &mut Position,
    guard_pos: &mut Position,
    tortoise: &mut Position,
    hare: &mut Position,
) {
    *start_pos = Position(row, col);
    *guard_pos = Position(row, col);
    *tortoise = Position(row, col);
    *hare = Position(row, col);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        let input = include_str!("input/day6/test_input.txt");
        let mut board = Board::from(input);
        board.find_path();
        assert_eq!(
            41,
            board.board.iter().fold(0, |acc, w| {
                if *w == Status::Traveled {
                    acc + 1
                } else {
                    acc
                }
            })
        ); // 1055
    }

    #[test]
    fn test_part_1_input() {
        let input = include_str!("input/day6/input.txt");
        let mut board = Board::from(input);
        board.find_path();
        println!(
            "{}",
            board.board.iter().fold(0, |acc, w| {
                if *w == Status::Traveled {
                    acc + 1
                } else {
                    acc
                }
            })
        ); // 18045
    }

    #[test]
    fn test_print_path_list() {
        let input = include_str!("input/day6/test_input.txt");
        let mut board = Board::from(input);
        let loops = board.part_2();
        println!("Part 2: {}", loops);
    }

    #[test]
    fn part_2_input() {
        let input = include_str!("input/day6/input.txt");
        let mut board = Board::from(input);
        let loops = board.part_2();
        println!("Part 2: {}", loops);
    }
}
