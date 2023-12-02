#![allow(dead_code)]

use std::cmp::Ordering;

#[derive(Debug)]
enum Literal {
    Z(i32),
    B(bool),
}

// https://logika.v3.sireum.org/doc/03-language/basic/index.html#operators-and-literals
#[derive(Debug)]
enum ProofOperator {
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

impl ProofOperator {
    const fn precedence(&self) -> usize {
        use ProofOperator::*;
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
}

#[derive(Debug)]
enum ClaimTokenType {
    Literal(Literal),
    Operator(ProofOperator),
    LeftParentheses,
    RightParentheses,
}

#[derive(Debug)]
struct ClaimToken {
    token: ClaimTokenType,
    start: usize,
    end: usize,
}

impl ClaimToken {
    fn new_literal(literal: Literal, start: usize, end: usize) -> Self {
        Self {
            token: ClaimTokenType::Literal(literal),
            start,
            end,
        }
    }

    fn new_operator(operator: Operator, start: usize, end: usize) -> Self {
        Self {
            token: ClaimTokenType::Operator(operator),
            start,
            end,
        }
    }
}

fn tokenize_integer(steam: &mut InputSteam) -> ClaimToken {
    // TODO handle negative numbers
    let start = steam.char_index;
    loop {
        let next = claim.peek();
        if next.is_none() || !next.unwrap().is_digit(10) {
            let end = claim.char_index;
            if start == end {
                panic!("next char after tokenizer_integer call was not an integer");
            }
            let slice = &stream.current_string[start..end];
            let z = slice.parse::<i32>().unwrap();
            return ClaimToken::new_literal(Literal::Z(z), start, end);
        }
        claim.advanced();
    }
}

fn tokenize_operator(steam: &mut InputSteam) -> ClaimToken {
    let start = steam.char_index;
    let first = steam.peek();
    steam.advance();
    let second = steam.peek();
    ClaimToken::new_operator(
        match (first, second) {
            ('∨', _) => Operator::Or,
            ('∧', _) => Operator::And,
            ('^', _) => Operator::Less,
            ('<', '=') => Operator::LessEquals,
            ('<', _) => Operator::Greater,
            ('>', '=') => Operator::GreaterEquals,
            ('=', '=') => Operator::Equals,
            ('=', _) => panic!("assignement is not an operator"),
            ('!', '=') => Operator(Operator::NotEquals),
            ('!', _) => Operator(Operator::Not),
            ('+', _) => Operator(Operator::Plus),
            ('-', _) => Operator(Operator::Minus),
            ('*', _) => Operator(Operator::Multiply),
            ('/', _) => Operator(Operator::Divide),
            ('%', _) => Operator(Operator::Modulus),
            ('-', _) => Operator(Operator::Negate),
            _ => panic!("next char after tokenize_operator was not an operator"),
        },
        start,
        end,
    )
}

fn tokenize_claim(expression: &str) -> Vec<ClaimToken> {
    let stream = InputSteam::new();
    let mut tokens = Vec::new();
    loop {
        match c {
            ' ' => (),
            '(' => tokens.push(ClaimToken::LeftParentheses),
            ')' => tokens.push(ClaimToken::RightParentheses),
            '∨' => tokenize_operator(&mut stream),
            '∧' => tokenize_operator(&mut stream),
            '^' => tokenize_operator(&mut stream),
            '+' => tokenize_operator(&mut stream),
            '-' => tokenize_operator(&mut stream),
            '*' => tokenize_operator(&mut stream),
            '/' => tokenize_operator(&mut stream),
            // this assumes all numbers are single digit and base 10
            _ => tokens.push(ClaimToken::Literal(Literal::Z(
                c.to_digit(10).unwrap() as i32
            ))),
        }
    }
    tokens
}

fn infix_to_posfix(tokens: Vec<ClaimToken>) -> Vec<ClaimToken> {
    let mut stack: Vec<ClaimToken> = Vec::new();
    let mut posfix: Vec<ClaimToken> = Vec::new();

    for token in tokens {
        match token {
            ClaimToken::LeftParentheses => stack.push(token),
            ClaimToken::RightParentheses => {
                loop {
                    assert!(stack.len() > 0, "mismatched parentheses");
                    match stack[stack.len() - 1] {
                        ClaimToken::LeftParentheses => break,
                        _ => posfix.push(stack.pop().unwrap()),
                    }
                    /* if there is a function token at the top of the operator stack, then:
                    pop the function from the operator stack into the output queue */
                }
                stack.pop();
            }
            ClaimToken::Literal(_) => posfix.push(token),
            ClaimToken::Operator(ref op) => {
                if op.is_unary() {
                    panic!("unary ops not yet implemented")
                }
                while stack.len() > 0 {
                    match &stack[stack.len() - 1] {
                        ClaimToken::Operator(stack_op)
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
        match stack.pop().unwrap() {
            ClaimToken::LeftParentheses => panic!("mismatched parentheses"),
            t => posfix.push(t),
        }
    }
    posfix
}
