use super::Day;

#[derive(PartialEq, Clone, Copy)]
enum Token {
    M,
    U,
    L,
    Num,
    Comma,
    LParen,
    RParen,
}

impl Token {
    fn valid_next_tokens(&self) -> &[Token] {
        match &self {
            Token::M => &[Token::U],
            Token::U => &[Token::L],
            Token::L => &[Token::LParen],
            Token::LParen => &[Token::Num],
            Token::Num => &[Token::Num, Token::RParen, Token::Comma],
            Token::Comma => &[Token::Num],
            Token::RParen => &[],
        }
    }

    fn from_char(c: char) -> Option<Token> {
        match c {
            'm' => Some(Token::M),
            'u' => Some(Token::U),
            'l' => Some(Token::L),
            '(' => Some(Token::LParen),
            ')' => Some(Token::RParen),
            ',' => Some(Token::Comma),
            _ if c.is_numeric() => Some(Token::Num),
            _ => None,
        }
    }
}

pub struct DayStruct;
impl Day for DayStruct {
    type Output = i32;

    fn part_one(&self, input: &str) -> Self::Output {
        let mut output = 0;
        let mut prev_token: Option<Token> = None;
        let mut operand_seq: Vec<char> = vec![];

        for ch in input.trim().chars() {
            match Token::from_char(ch) {
                Some(token) => {
                    if token == Token::M {
                        prev_token = Some(token);
                        operand_seq.clear();
                    } else if let Some(prev) = prev_token {
                        if prev.valid_next_tokens().contains(&token) {
                            match token {
                                Token::RParen => {
                                    output += multiply_chars(&operand_seq);
                                    prev_token = None;
                                    operand_seq.clear();
                                }
                                Token::Num | Token::Comma => {
                                    operand_seq.push(ch);
                                    prev_token = Some(token);
                                }
                                _ => prev_token = Some(token),
                            }
                        } else {
                            prev_token = None;
                            operand_seq.clear();
                        }
                    }
                }
                None => {
                    prev_token = None;
                    operand_seq.clear();
                }
            }
        }

        output
    }

    fn part_two(&self, input: &str) -> Self::Output {
        let mut output = 0;
        let mut prev_token: Option<Token> = None;
        let mut operand_seq: Vec<char> = vec![];

        for ch in input
            .trim()
            .split("do()")
            .flat_map(|s| s.split("don't()").next().unwrap_or("").chars())
        {
            match Token::from_char(ch) {
                Some(token) => {
                    if token == Token::M {
                        prev_token = Some(token);
                        operand_seq.clear();
                    } else if let Some(prev) = prev_token {
                        if prev.valid_next_tokens().contains(&token) {
                            match token {
                                Token::RParen => {
                                    output += multiply_chars(&operand_seq);
                                    prev_token = None;
                                    operand_seq.clear();
                                }
                                Token::Num | Token::Comma => {
                                    operand_seq.push(ch);
                                    prev_token = Some(token);
                                }
                                _ => prev_token = Some(token),
                            }
                        } else {
                            prev_token = None;
                            operand_seq.clear();
                        }
                    }
                }
                None => {
                    prev_token = None;
                    operand_seq.clear();
                }
            }
        }

        output
    }
}

fn multiply_chars(chars: &[char]) -> i32 {
    let mut left: Vec<char> = Vec::with_capacity(3);
    let mut right: Vec<char> = Vec::with_capacity(3);
    let mut comma_found = false;

    for &ch in chars {
        match ch {
            ',' if !comma_found => comma_found = true,
            ',' => return 0,
            _ if ch.is_numeric() => {
                if comma_found {
                    right.push(ch);
                } else {
                    left.push(ch);
                }
            }
            _ => {}
        }
    }

    let l: i32 = left.into_iter().collect::<String>().parse().unwrap_or(0);
    let r: i32 = right.into_iter().collect::<String>().parse().unwrap_or(0);

    l * r
}

#[cfg(test)]
mod day3_tests {
    use super::*;

    const EXAMPLE: &str = include_str!("./example.txt");

    #[test]
    fn test_part_one_example() {
        let expected = 161;

        assert_eq!(DayStruct.part_one(EXAMPLE), expected);
    }

    #[test]
    fn test_part_two_example() {
        let expected = 48;

        assert_eq!(DayStruct.part_two(EXAMPLE), expected);
    }
}
