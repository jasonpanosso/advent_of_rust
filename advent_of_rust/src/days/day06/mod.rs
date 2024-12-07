use std::collections::HashSet;

use super::Day;

#[derive(PartialEq, Clone)]
enum Token {
    Wall,
    Floor,
    Seen(Vec<Direction>),
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
        tokens[row_start][col_start] = Token::Seen(vec![dir]);

        walk_tiles(&mut tokens, row_start, col_start, dir);
        tokens
            .iter()
            .flat_map(|line| {
                line.iter()
                    .filter(|token| matches!(**token, Token::Seen(_)))
            })
            .count() as i32
    }

    fn part_two(&self, input: &str) -> Self::Output {
        let (dir, row_start, col_start) = get_start(input);
        let matrix: Vec<Vec<Token>> = input
            .lines()
            .map(|line| line.chars().map(Token::from_char).collect())
            .collect();

        let mut count = 0;
        for (r, line) in matrix.iter().enumerate() {
            for (c, token) in line.iter().enumerate() {
                if (r == row_start && c == col_start) || matches!(token, Token::Wall) {
                    continue;
                }

                if !can_escape(&matrix, row_start, col_start, dir, (r, c)) {
                    count += 1;
                }
            }
        }

        count
    }
}

fn can_escape(
    matrix: &[Vec<Token>],
    start_row: usize,
    start_col: usize,
    start_dir: Direction,
    new_wall: (usize, usize),
) -> bool {
    let mut visited = HashSet::new();
    let (mut row, mut col, mut dir) = (start_row, start_col, start_dir);

    loop {
        let (next_row, next_col) = dir.next_tile(row as i32, col as i32);
        if next_row < 0
            || next_col < 0
            || next_row as usize >= matrix.len()
            || next_col as usize >= matrix[next_row as usize].len()
        {
            return true;
        }

        if (next_row as usize, next_col as usize) == new_wall
            || matches!(matrix[next_row as usize][next_col as usize], Token::Wall)
        {
            dir = dir.turn_right();
        } else {
            row = next_row as usize;
            col = next_col as usize;

            if !visited.insert((row, col, dir)) {
                return false;
            }
        }
    }
}

fn walk_tiles(matrix: &mut [Vec<Token>], cur_row: usize, cur_col: usize, cur_dir: Direction) {
    let next = cur_dir.next_tile(cur_row as i32, cur_col as i32);

    if next.0 >= 0
        && next.1 >= 0
        && next.0 < matrix.len() as i32
        && next.1 < matrix[next.0 as usize].len() as i32
    {
        match matrix[next.0 as usize][next.1 as usize] {
            Token::Wall => walk_tiles(matrix, cur_row, cur_col, cur_dir.turn_right()),
            _ => {
                matrix[next.0 as usize][next.1 as usize] = Token::Seen(vec![cur_dir]);
                walk_tiles(matrix, next.0 as usize, next.1 as usize, cur_dir)
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
