use super::Day;

#[derive(PartialEq)]
enum Token {
    Wall,
    Floor,
    Seen,
}

#[derive(Clone, Copy, PartialEq, Hash, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_char(ch: char) -> Option<Direction> {
        match ch {
            '^' => Some(Direction::Up),
            '>' => Some(Direction::Right),
            'v' => Some(Direction::Down),
            '<' => Some(Direction::Left),
            _ => None,
        }
    }

    fn to_bit(self) -> u8 {
        match self {
            Direction::Up => 0b0001,
            Direction::Right => 0b0010,
            Direction::Down => 0b0100,
            Direction::Left => 0b1000,
        }
    }

    fn next_tile(&self, i: i32, j: i32) -> (i32, i32) {
        match self {
            Direction::Up => (i - 1, j),
            Direction::Down => (i + 1, j),
            Direction::Left => (i, j - 1),
            Direction::Right => (i, j + 1),
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

impl Token {
    fn from_char(ch: char) -> Token {
        match ch {
            '#' => Token::Wall,
            '^' | '>' | 'v' | '<' => Token::Seen,
            _ => Token::Floor,
        }
    }
}

pub struct DayStruct;
impl Day for DayStruct {
    type Output = i32;

    fn part_one(&self, input: &str) -> Self::Output {
        let (dir, row_start, col_start) = get_start(input);
        let mut tokens: Vec<Vec<Token>> = input
            .lines()
            .map(|line| line.chars().map(Token::from_char).collect())
            .collect();

        walk_tiles(&mut tokens, row_start, col_start, dir);
        tokens
            .iter()
            .flat_map(|line| line.iter().filter(|token| matches!(**token, Token::Seen)))
            .count() as i32
    }

    fn part_two(&self, input: &str) -> Self::Output {
        let (dir, row_start, col_start) = get_start(input);
        let mut tokens: Vec<Vec<Token>> = input
            .lines()
            .map(|line| line.chars().map(Token::from_char).collect())
            .collect();
        let rows = tokens.len();
        let cols = tokens[0].len();

        walk_tiles(&mut tokens, row_start, col_start, dir);
        let visited: Vec<(usize, usize)> = tokens
            .iter()
            .enumerate()
            .flat_map(|(r, row)| {
                row.iter().enumerate().filter_map(move |(c, token)| {
                    if matches!(token, Token::Seen) {
                        Some((r, c))
                    } else {
                        None
                    }
                })
            })
            .collect();

        let mut result = 0;
        for &(row, col) in &visited {
            if (row, col) == (row_start, col_start) {
                continue;
            }

            let original = std::mem::replace(&mut tokens[row][col], Token::Wall);
            if !can_escape(&tokens, rows, cols, row_start, col_start, dir) {
                result += 1;
            }

            tokens[row][col] = original;
        }

        result
    }
}

fn can_escape(
    matrix: &[Vec<Token>],
    rows: usize,
    cols: usize,
    row: usize,
    col: usize,
    dir: Direction,
) -> bool {
    let mut visited = vec![0u8; rows * cols];
    visited[row * cols + col] |= dir.to_bit();

    let mut current_row = row;
    let mut current_col = col;
    let mut current_dir = dir;

    loop {
        let next = current_dir.next_tile(current_row as i32, current_col as i32);
        let (next_row, next_col) = match conv_usize(next.0, next.1, matrix.len(), matrix[0].len()) {
            Some(pos) => pos,
            None => {
                return true;
            }
        };

        match matrix[next_row][next_col] {
            Token::Wall => {
                current_dir = current_dir.turn_right();
            }
            _ => {
                current_row = next_row;
                current_col = next_col;
            }
        }

        let next_index = next_row * cols + next_col;
        let bit = current_dir.to_bit();
        if visited[next_index] & bit != 0 {
            return false;
        }

        visited[next_index] |= bit;
    }
}

fn walk_tiles(matrix: &mut [Vec<Token>], cur_row: usize, cur_col: usize, cur_dir: Direction) {
    let next = cur_dir.next_tile(cur_row as i32, cur_col as i32);

    if let Some((next_row, next_col)) = conv_usize(next.0, next.1, matrix.len(), matrix[0].len()) {
        match matrix[next_row][next_col] {
            Token::Wall => walk_tiles(matrix, cur_row, cur_col, cur_dir.turn_right()),
            _ => {
                matrix[next_row][next_col] = Token::Seen;
                walk_tiles(matrix, next_row, next_col, cur_dir)
            }
        }
    }
}

fn get_start(input: &str) -> (Direction, usize, usize) {
    let (row_start, col_start, ch) = input
        .lines()
        .enumerate()
        .flat_map(|(row_idx, row)| {
            row.chars()
                .enumerate()
                .map(move |(col_idx, ch)| (row_idx, col_idx, ch))
        })
        .find(|&(_, _, c)| Direction::from_char(c).is_some())
        .unwrap();

    (Direction::from_char(ch).unwrap(), row_start, col_start)
}

fn conv_usize(row: i32, col: i32, row_max: usize, col_max: usize) -> Option<(usize, usize)> {
    if row >= 0 && col >= 0 && row < row_max as i32 && col < col_max as i32 {
        Some((row as usize, col as usize))
    } else {
        None
    }
}

#[cfg(test)]
mod day6_tests {
    use super::*;

    const EXAMPLE: &str = include_str!("./example.txt");

    #[test]
    fn test_part_one_example() {
        let expected = 41;

        assert_eq!(DayStruct.part_one(EXAMPLE), expected);
    }

    #[test]
    fn test_part_two_example() {
        let expected = 6;

        assert_eq!(DayStruct.part_two(EXAMPLE), expected);
    }
}
