#![allow(dead_code)]

use crate::repeats_no_whitespace::*;
use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
enum TokenKind {
    // Literals
    Z(isize),
    B(bool),
    // Operators
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
    // Seperators
    Newline,
    LeftParentheses,
    RightParentheses,
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

    fn is_operator(&self) -> bool {
        use TokenKind::*;
        match self.kind {
            Or => true,
            And => true,
            Less => true,
            LessEquals => true,
            Greater => true,
            GreaterEquals => true,
            Equals => true,
            NotEquals => true,
            Not => true,
            Plus => true,
            Minus => true,
            Multiply => true,
            Divide => true,
            Modulus => true,
            Negate => true,
            _ => false,
        }
    }

    pub const fn operator_precedence(&self) -> usize {
        use TokenKind::*;
        match self.kind {
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
            _ => panic!("cannot call operator_precedence on non-operator"),
        }
    }

    pub fn precedes(&self, other: &Self) -> Ordering {
        match (self.operator_precedence(), other.operator_precedence()) {
            (a, b) if a < b => Ordering::Less,
            (a, b) if a == b => Ordering::Equal,
            (a, b) if a > b => Ordering::Greater,
            _ => unreachable!(),
        }
    }

    pub fn is_unary(&self) -> bool {
        use TokenKind::*;
        match self.kind {
            Not => true,
            Negate => true,
            _ => false,
        }
    }

    pub fn is_double_width(&self) -> bool {
        use TokenKind::*;
        match self.kind {
            LessEquals => true,
            GreaterEquals => true,
            Equals => true,
            NotEquals => true,
            _ => false,
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
    let z = slice.parse::<isize>().unwrap();
    Token::new(TokenKind::Z(z), start, end)
}

fn tokenize_operator(reader: &mut RepeatsNoWhiteSpace) -> Token {
    use TokenKind::*;
    let start = reader.char_index;
    let first = reader.next();
    let second = reader.get();
    let (operator, end) = match (first, second) {
        (Some('∨'), _) => (Or, start + 1),
        (Some('∧'), _) => (And, start + 1),
        (Some('^'), _) => (Less, start + 1),
        (Some('<'), Some('=')) => (LessEquals, start + 2),
        (Some('<'), _) => (Greater, start + 1),
        (Some('>'), Some('=')) => (GreaterEquals, start + 2),
        (Some('='), Some('=')) => (Equals, start + 2),
        (Some('='), _) => panic!("assignement is not an operator"),
        (Some('!'), Some('=')) => (NotEquals, start + 2),
        (Some('!'), _) => (Not, start + 1),
        (Some('+'), _) => (Plus, start + 1),
        //(Some('-'), _) => Minus, TODO tokenizer should be dumb, the parser will apply negations to integer
        (Some('*'), _) => (Multiply, start + 1),
        (Some('/'), _) => (Divide, start + 1),
        (Some('%'), _) => (Modulus, start + 1),
        (Some('-'), _) => (Negate, start + 1),
        _ => panic!("next char after tokenize_operator was not an operator"),
    };
    Token::new(operator, start, end)
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
        assert_eq!(tokenize(reader), [Token::new(TokenKind::Z(420), 0, 3)])
    }

    #[test]
    fn multiple_integers() {
        let file = file_from_str("420 69");
        let reader = RepeatsNoWhiteSpace::new(file);
        assert_eq!(
            tokenize(reader),
            [
                Token::new(TokenKind::Z(420), 0, 3),
                Token::new(TokenKind::Z(69), 4, 6)
            ]
        )
    }

    #[test]
    fn operator() {
        let file = file_from_str("+");
        let reader = RepeatsNoWhiteSpace::new(file);
        assert_eq!(tokenize(reader), [Token::new(TokenKind::Plus, 0, 1)])
    }

    #[test]
    fn integers_and_operators() {
        let file = file_from_str("420+69");
        let reader = RepeatsNoWhiteSpace::new(file);
        assert_eq!(
            tokenize(reader),
            [
                Token::new(TokenKind::Z(420), 0, 3),
                Token::new(TokenKind::Plus, 3, 4),
                Token::new(TokenKind::Z(69), 4, 6)
            ]
        )
    }
}
