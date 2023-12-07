#![allow(dead_code)]

use crate::literal::Literal;
use crate::operator::Operator;
use crate::repeats_no_whitespace::*;

#[derive(Debug, PartialEq)]
enum TokenKind {
    Literal(Literal),
    Operator(Operator),
    LeftParentheses,
    RightParentheses,
    Newline,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    kind: TokenKind,
    start: usize,
    end: usize,
}

impl Token {
    fn new(kind: TokenKind, start: usize, end: usize) -> Self {
        Self { kind, start, end }
    }

    fn new_literal(literal: Literal, start: usize, end: usize) -> Self {
        Self {
            kind: TokenKind::Literal(literal),
            start,
            end,
        }
    }

    fn new_operator(operator: Operator, start: usize, end: usize) -> Self {
        Self {
            kind: TokenKind::Operator(operator),
            start,
            end,
        }
    }
}

fn tokenize_integer(reader: &mut RepeatsNoWhiteSpace) -> Token {
    // TODO handle negative numbers
    let start = reader.char_index;
    loop {
        let next = reader.get();
        if next.is_none() || !next.unwrap().is_digit(10) {
            break;
        }
        let _ = reader.next();
    }
    let end = reader.char_index;
    assert_ne!(
        start, end,
        "next char after tokenizer_integer call was not an integer"
    );
    let slice = &reader.current_line[start..end];
    let z = slice.parse::<i32>().unwrap();
    Token::new_literal(Literal::Z(z), start, end)
}

fn tokenize_operator(reader: &mut RepeatsNoWhiteSpace) -> Token {
    use Operator::*;
    let start = reader.char_index;
    let first = reader.next();
    let second = reader.get();
    let operator = match (first, second) {
        (Some('∨'), _) => Or,
        (Some('∧'), _) => And,
        (Some('^'), _) => Less,
        (Some('<'), Some('=')) => LessEquals,
        (Some('<'), _) => Greater,
        (Some('>'), Some('=')) => GreaterEquals,
        (Some('='), Some('=')) => Equals,
        (Some('='), _) => panic!("assignement is not an operator"),
        (Some('!'), Some('=')) => NotEquals,
        (Some('!'), _) => Not,
        (Some('+'), _) => Plus,
        //(Some('-'), _) => Minus, TODO tokenizer should be dumb, the parser will apply negations to integers
        (Some('*'), _) => Multiply,
        (Some('/'), _) => Divide,
        (Some('%'), _) => Modulus,
        (Some('-'), _) => Negate,
        _ => panic!("next char after tokenize_operator was not an operator"),
    };
    let end = match operator.is_double_width() {
        false => start + 1,
        true => start + 2,
    };
    Token::new_operator(operator, start, end)
}

fn tokenize_seperator(reader: &mut RepeatsNoWhiteSpace) -> Token {
    use TokenKind::*;

    let start = reader.char_index;
    Token::new(
        match reader.next() {
            Some('(') => LeftParentheses,
            Some(')') => RightParentheses,
            Some('\n') => Newline,
            _ => panic!("next char after tokenize_seperator was not a seperator"),
        },
        start,
        start + 1,
    )
}

fn tokenize(mut reader: RepeatsNoWhiteSpace) -> Vec<Token> {
    let mut tokens = Vec::new();
    while let Some(c) = reader.get() {
        tokens.push(match c {
            ' ' => {
                reader.next();
                continue;
            }
            '\n' => tokenize_seperator(&mut reader),
            '(' => tokenize_seperator(&mut reader),
            ')' => tokenize_seperator(&mut reader),
            '∨' => tokenize_operator(&mut reader),
            '∧' => tokenize_operator(&mut reader),
            '^' => tokenize_operator(&mut reader),
            '+' => tokenize_operator(&mut reader),
            '-' => tokenize_operator(&mut reader),
            '*' => tokenize_operator(&mut reader),
            '/' => tokenize_operator(&mut reader),
            c if c.is_digit(10) => tokenize_integer(&mut reader),
            _ => panic!("unrecognized character"),
        });
    }
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repeats_no_whitespace::RepeatsNoWhiteSpace;
    use crate::test_helpers::file_from_str;

    #[test]
    fn integer() {
        let file = file_from_str("420");
        let reader = RepeatsNoWhiteSpace::new(file);
        assert_eq!(
            tokenize(reader),
            [Token::new_literal(Literal::Z(420), 0, 3)]
        )
    }

    #[test]
    fn multiple_integers() {
        let file = file_from_str("420 69");
        let reader = RepeatsNoWhiteSpace::new(file);
        assert_eq!(
            tokenize(reader),
            [
                Token::new_literal(Literal::Z(420), 0, 3),
                Token::new_literal(Literal::Z(69), 4, 6)
            ]
        )
    }

    #[test]
    fn operator() {
        let file = file_from_str("+");
        let reader = RepeatsNoWhiteSpace::new(file);
        assert_eq!(
            tokenize(reader),
            [Token::new_operator(Operator::Plus, 0, 1)]
        )
    }

    #[test]
    fn integers_and_operators() {
        let file = file_from_str("420+69");
        let reader = RepeatsNoWhiteSpace::new(file);
        assert_eq!(
            tokenize(reader),
            [
                Token::new_literal(Literal::Z(420), 0, 3),
                Token::new_operator(Operator::Plus, 3, 4),
                Token::new_literal(Literal::Z(69), 4, 6)
            ]
        )
    }
}
