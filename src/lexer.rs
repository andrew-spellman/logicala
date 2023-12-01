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
enum ClaimToken {
    Literal(Literal),
    Operator(ProofOperator),
    LeftParentheses,
    RightParentheses,
}

#[derive(Debug)]
struct ClaimTokenWithLocation {
    token: ClaimToken,
    start: usize,
}

struct ClaimConsumable {
    claim: String,
    index: usize,
}

impl ClaimConsumable {
    fn get_next(&self) -> Option<char> {
        if self.index < self.claim.len() - 1 {
            return Some(self.claim.chars().nth(self.index).unwrap());
        }
        None
    }

    fn consume(&mut self) {
        if self.index >= self.claim.len() {
            panic!("attempted to next claim char after last char already consumed");
        }
        self.index += 1;
    }
}

fn tokenize_claim_integer(claim: &mut ClaimConsumable) -> ClaimTokenWithLocation {
    let start = claim.index;
    loop {
        let next = claim.get_next();
        if next.is_none() || !next.unwrap().is_digit(10) {
            let end = claim.index;
            if start == end {
                panic!("next char after tokenizer_claim_integer call was not an integer");
            }
            let slice = &claim.claim[start..end];
            let z = slice.parse::<i32>().unwrap();
            return ClaimTokenWithLocation {
                token: ClaimToken::Literal(Literal::Z(z)),
                start,
            };
        }
        claim.consume();
    }
}

fn tokenize_claim_operator(claim: &mut ClaimConsumable) -> ClaimTokenWithLocation {}

fn tokenize_claim_operator(claim: &mut ClaimConsumable) -> ClaimTokenWithLocation {}

fn tokenize_claim(expression: &str) -> Vec<ClaimToken> {
    // all needs to be reimplemented to support multi-character tokens
    let mut tokens = Vec::new();
    for c in expression.chars() {
        match c {
            ' ' => (),
            '(' => tokens.push(ClaimToken::LeftParentheses),
            ')' => tokens.push(ClaimToken::RightParentheses),
            '∨' => tokens.push(ClaimToken::Operator(ProofOperator::Or)),
            '∧' => tokens.push(ClaimToken::Operator(ProofOperator::And)),
            '+' => tokens.push(ClaimToken::Operator(ProofOperator::Plus)),
            '-' => tokens.push(ClaimToken::Operator(ProofOperator::Minus)),
            '*' => tokens.push(ClaimToken::Operator(ProofOperator::Multiply)),
            '/' => tokens.push(ClaimToken::Operator(ProofOperator::Divide)),
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
