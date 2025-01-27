use crate::day15::Direction;
use crate::day15::Position;
pub struct Map {
    robot: (usize, usize),
    pub(crate) map: Vec<NodeType>,
    pub(crate) rows: usize,
    pub(crate) columns: usize,
    pub(crate) moves: Vec<Direction>,
}

impl Map {
    pub fn from(input: &str) -> Self {
        let (map_str, moves_str) = input.split_once("\n\n").unwrap();
        let map = map_str
            .chars()
            .filter_map(|c| NodeType::try_from(c).ok())
            .collect::<Vec<NodeType>>();
        let rows = map_str.lines().count();
        let columns = map_str.lines().next().unwrap().chars().count();
        let robot_row = map_str.lines().position(|line| line.contains('@')).unwrap();
        let robot_column = map_str.lines().nth(robot_row).unwrap().find('@').unwrap();

        let moves = moves_str
            .chars()
            .filter_map(|c| Direction::try_from(c).ok())
            .collect::<Vec<Direction>>();

        Map {
            robot: (robot_row, robot_column),
            map,
            rows,
            columns,
            moves,
        }
    }

    pub(crate) fn position_to_index(&self, position: &Position) -> usize {
        self.rows * position.row + position.column
    }

    pub(crate) fn move_robot(&mut self, direction: Direction) {
        let (row, column) = self.robot;
        let (dx, dy): (isize, isize) = match direction {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        };
        let next_position = &Position {
            row: (row as isize + dx) as usize,
            column: (column as isize + dy) as usize,
        };
        let current_idx = self.position_to_index(&Position { row, column });
        let next_idx = self.position_to_index(next_position);
        match self.map[self.position_to_index(next_position)] {
            NodeType::Wall => return,
            NodeType::Box => {
                if self.move_boxes(next_position, direction) {
                    self.map[next_idx] = NodeType::Robot;
                    self.map[current_idx] = NodeType::Empty;
                    self.robot = (next_position.row, next_position.column);
                }
            }
            NodeType::Empty => {
                self.map[next_idx] = NodeType::Robot;
                self.map[current_idx] = NodeType::Empty;
                self.robot = (next_position.row, next_position.column);
            }
            _ => unreachable!(),
        }
    }

    fn move_boxes(&mut self, position: &Position, direction: Direction) -> bool {
        let (dx, dy): (isize, isize) = match direction {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        };
        let next_position = &Position {
            row: (position.row as isize + dx) as usize,
            column: (position.column as isize + dy) as usize,
        };
        let current_idx = self.position_to_index(position);
        let next_idx = self.position_to_index(next_position);

        match self.map[self.position_to_index(next_position)] {
            NodeType::Box => {
                if self.move_boxes(next_position, direction) {
                    self.map[next_idx] = NodeType::Box;
                    self.map[current_idx] = NodeType::Empty;
                    true
                } else {
                    false
                }
            }
            NodeType::Wall => false,
            NodeType::Empty => {
                self.map[next_idx] = NodeType::Box;
                self.map[current_idx] = NodeType::Empty;
                true
            }
            _ => unreachable!(),
        }
    }

    pub fn print_map(&self) {
        for row in 0..self.rows {
            for column in 0..self.columns {
                match self.map[self.position_to_index(&Position { row, column })] {
                    NodeType::Robot => print!("@"),
                    NodeType::Box => print!("O"),
                    NodeType::Wall => print!("#"),
                    NodeType::Empty => print!("."),
                }
            }
            println!();
        }
        println!();
    }
}

#[derive(PartialEq)]
pub enum NodeType {
    Robot,
    Box,
    Wall,
    Empty,
}

impl TryFrom<char> for NodeType {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(NodeType::Wall),
            '.' => Ok(NodeType::Empty),
            '@' => Ok(NodeType::Robot),
            'O' => Ok(NodeType::Box),
            _ => Err(()),
        }
    }
}