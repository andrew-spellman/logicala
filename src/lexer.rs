#![allow(dead_code)]

use crate::repeats_no_whitespace::*;
use std::cmp::Ordering;
use std::fs::File;

#[derive(Debug)]
enum Literal {
    Z(i32),
    B(bool),
}

// https://logika.v3.sireum.org/doc/03-language/basic/index.html#operators-and-literals
#[derive(Debug)]
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

#[derive(Debug)]
enum TokenType {
    Literal(Literal),
    Operator(Operator),
    LeftParentheses,
    RightParentheses,
}

#[derive(Debug)]
struct Token {
    token_type: TokenType,
    start: usize,
    end: usize,
}

impl Token {
    fn new_literal(literal: Literal, start: usize, end: usize) -> Self {
        Self {
            token_type: TokenType::Literal(literal),
            start,
            end,
        }
    }

    fn new_operator(operator: Operator, start: usize, end: usize) -> Self {
        Self {
            token_type: TokenType::Operator(operator),
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
        (Some('-'), _) => Minus,
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
    let seperator = match reader.next() {
        Some('(') => TokenType::LeftParentheses,
        Some(')') => TokenType::RightParentheses,
        _ => panic!("next char after tokenize_seperator was not a seperator"),
    };
    Token {
        token_type: seperator,
        start,
        end: start + 1,
    }
}

fn tokenize(file: File) -> Vec<Token> {
    let mut reader = RepeatsNoWhiteSpace::new(file);
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

fn infix_to_posfix(tokens: Vec<Token>) -> Vec<Token> {
    let mut stack: Vec<Token> = Vec::new();
    let mut posfix: Vec<Token> = Vec::new();

    for token in tokens {
        match token.token_type {
            TokenType::LeftParentheses => stack.push(token),
            TokenType::RightParentheses => {
                loop {
                    assert!(stack.len() > 0, "mismatched parentheses");
                    match stack[stack.len() - 1].token_type {
                        TokenType::LeftParentheses => break,
                        _ => posfix.push(stack.pop().unwrap()),
                    }
                    /* if there is a function token at the top of the operator stack, then:
                    pop the function from the operator stack into the output queue */
                }
                stack.pop();
            }
            TokenType::Literal(_) => posfix.push(tkoken),
            TokenType::Operator(ref op) => {
                if op.is_unary() {
                    panic!("unary ops not yet implemented")
                }
                while stack.len() > 0 {
                    match &stack[stack.len() - 1].token_type {
                        TokenType::Operator(stack_op)
                            if stack_op.precedes(op) == Ordering::Greater =>
                        {
                            posfix.push(stack.pop().unwrap())
                        }
                        _ => break,
                    }
                }
                stack.push(token);
            }
        }
    }
    while stack.len() > 0 {
        let stack_token = stack.pop().unwrap();
        match stack_token.token_type {
            TokenType::LeftParentheses => panic!("mismatched parentheses"),
            _ => (),
        }
        posfix.push(stack_token)
    }
    posfix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
