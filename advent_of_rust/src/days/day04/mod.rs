use super::Day;

#[derive(PartialEq)]
enum Token {
    X,
    M,
    A,
    S,
}

impl Token {
    fn next_token(&self) -> Option<Token> {
        match &self {
            Token::X => Some(Token::M),
            Token::M => Some(Token::A),
            Token::A => Some(Token::S),
            Token::S => None,
        }
    }

    fn from_char(ch: char) -> Option<Token> {
        match ch {
            'X' => Some(Token::X),
            'M' => Some(Token::M),
            'A' => Some(Token::A),
            'S' => Some(Token::S),
            _ => None,
        }
    }
}

pub struct DayStruct;
impl Day for DayStruct {
    type Output = i32;

    fn part_one(&self, input: &str) -> Self::Output {
        let mut output = 0;
        let token_matrix: Vec<Vec<Option<Token>>> = input
            .lines()
            .map(|line| line.chars().map(Token::from_char).collect())
            .collect();

        const ALL_DIRECTIONS: [(i32, i32); 8] = [
            (1, 0),
            (-1, 0),
            (0, 1),
            (0, -1),
            (1, 1),
            (-1, -1),
            (-1, 1),
            (1, -1),
        ];

        for i in 0..token_matrix.len() {
            for j in 0..token_matrix[i].len() {
                if token_matrix[i][j] == Some(Token::X) {
                    for dir in ALL_DIRECTIONS.iter() {
                        if check_sequence_in_direction(
                            &token_matrix,
                            &Token::X,
                            i as i32,
                            j as i32,
                            dir.0,
                            dir.1,
                        ) {
                            output += 1;
                        }
                    }
                }
            }
        }

        output
    }

    fn part_two(&self, input: &str) -> Self::Output {
        let mut output = 0;
        let token_matrix: Vec<Vec<Option<Token>>> = input
            .lines()
            .map(|line| line.chars().map(Token::from_char).collect())
            .collect();

        for i in 0..token_matrix.len() {
            for j in 0..token_matrix[i].len() {
                if token_matrix[i][j] == Some(Token::A) && check_diagonals(&token_matrix, i, j) {
                    output += 1;
                }
            }
        }

        output
    }
}

fn check_sequence_in_direction(
    matrix: &Vec<Vec<Option<Token>>>,
    cur_token: &Token,
    cur_i: i32,
    cur_j: i32,
    i_increment: i32,
    j_increment: i32,
) -> bool {
    let i = cur_i + i_increment;
    let j = cur_j + j_increment;

    if i < 0 || j < 0 {
        return false;
    }

    match matrix.get(i as usize) {
        Some(tokens) => match tokens.get(j as usize) {
            Some(token) => match cur_token.next_token() == *token {
                true => match token == &Some(Token::S) {
                    true => true,
                    false => check_sequence_in_direction(
                        matrix,
                        token.as_ref().unwrap(),
                        i,
                        j,
                        i_increment,
                        j_increment,
                    ),
                },
                false => false,
            },
            None => false,
        },
        None => false,
    }
}

fn check_diagonals(matrix: &[Vec<Option<Token>>], i: usize, j: usize) -> bool {
    match get_diagonal_pairs(matrix, i, j) {
        Some(pairs) => {
            check_diagonal_pair(pairs.0 .0, pairs.0 .1)
                && check_diagonal_pair(pairs.1 .0, pairs.1 .1)
        }
        None => false,
    }
}

fn check_diagonal_pair(t1: &Token, t2: &Token) -> bool {
    t1 != t2 && is_m_or_s(t1) && is_m_or_s(t2)
}

fn is_m_or_s(t: &Token) -> bool {
    *t == Token::M || *t == Token::S
}

// APPEASE CLIPPY..
type TokenDiagonal<'a> = (&'a Token, &'a Token);

fn get_diagonal_pairs(
    matrix: &[Vec<Option<Token>>],
    i: usize,
    j: usize,
) -> Option<(TokenDiagonal, TokenDiagonal)> {
    let get_token = |di: i32, dj: i32| -> Option<&Token> {
        let new_i = i as i32 + di;
        let new_j = j as i32 + dj;

        if new_i >= 0 && new_j >= 0 {
            matrix.get(new_i as usize)?.get(new_j as usize)?.as_ref()
        } else {
            None
        }
    };

    Some((
        (
            get_token(1, 1)?,   // NE
            get_token(-1, -1)?, // SW
        ),
        (
            get_token(-1, 1)?, // NW
            get_token(1, -1)?, // SE
        ),
    ))
}

#[cfg(test)]
mod day4_tests {
    use super::*;

    const EXAMPLE: &str = include_str!("./example.txt");

    #[test]
    fn test_part_one_example() {
        let expected = 18;

        assert_eq!(DayStruct.part_one(EXAMPLE), expected);
    }

    #[test]
    fn test_part_two_example() {
        let expected = 9;

        assert_eq!(DayStruct.part_two(EXAMPLE), expected);
    }
}
