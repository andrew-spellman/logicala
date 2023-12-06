#![allow(dead_code)]

use crate::repeats_no_whitespace::*;
use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
enum Literal {
    Z(i32),
    B(bool),
}

// https://logika.v3.sireum.org/doc/03-language/basic/index.html#operators-and-literals
#[derive(Debug, PartialEq)]
enum Operator {
    Or,
    And,
    Less,
    LessEquals,
    Greater,
    GreaterEquals,
    Equals,
    NotEquals,
    Not,
    Plus, // TODO: Prepend and Postpend Plus
    Minus,
    Multiply,
    Modulus,
    Divide,
    // TODO: Figure out if we need Exponent
    Negate,
}

impl Operator {
    const fn precedence(&self) -> usize {
        use Operator::*;
        match self {
            Or => 0,
            And => 1,
            Less => 2,
            LessEquals => 2,
            Greater => 2,
            GreaterEquals => 2,
            Equals => 3,
            NotEquals => 3,
            Not => 3,
            Plus => 4,
            Minus => 4,
            Multiply => 5,
            Divide => 5,
            Modulus => 5,
            Negate => 6,
        }
    }

    fn precedes(&self, other: &Self) -> Ordering {
        match (self.precedence(), other.precedence()) {
            (a, b) if a < b => Ordering::Less,
            (a, b) if a == b => Ordering::Equal,
            (a, b) if a > b => Ordering::Greater,
            _ => unreachable!(),
        }
    }

    fn is_unary(&self) -> bool {
        match self {
            Self::Not => true,
            Self::Negate => true,
            _ => false,
        }
    }

    fn is_double_width(&self) -> bool {
        use Operator::*;
        match self {
            LessEquals => true,
            GreaterEquals => true,
            Equals => true,
            NotEquals => true,
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq)]
enum TokenKind {
    Literal(Literal),
    Operator(Operator),
    LeftParentheses,
    RightParentheses,
}

#[derive(Debug, PartialEq)]
struct Token {
    kind: TokenKind,
    start: usize,
    end: usize,
}

impl Token {
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
            let end = reader.char_index;
            if start == end {
                panic!("next char after tokenizer_integer call was not an integer");
            }
            let slice = &reader.current_line[start..end];
            let z = slice.parse::<i32>().unwrap();
            return Token::new_literal(Literal::Z(z), start, end);
        }
        let _ = reader.next();
    }
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
    let start = reader.char_index;
    match reader.next() {
        Some('(') => Token {
            kind: TokenKind::LeftParentheses,
            start,
            end: start + 1,
        },
        Some(')') => Token {
            kind: TokenKind::RightParentheses,
            start,
            end: start + 1,
        },
        _ => panic!("next char after tokenize_seperator was not a seperator"),
    }
}

fn tokenize(mut reader: RepeatsNoWhiteSpace) -> Vec<Token> {
    let mut tokens = Vec::new();
    while let Some(c) = reader.get() {
        tokens.push(match c {
            ' ' => {
                reader.next();
                continue;
            }
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
