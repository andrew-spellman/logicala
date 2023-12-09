use crate::tokenizer::{Token, TokenKind};
use std::{cmp::Ordering, slice::Iter};

#[derive(Debug, PartialEq)]
pub struct Expression {
    tokens: Vec<Token>,
}

impl Expression {
    fn iter(&self) -> Iter<'_, Token> {
        self.tokens.iter()
    }

    pub fn last_position_of_token_kind_in_expression(&self, kind: &TokenKind) -> usize {
        self.tokens.len()
            - 1
            - self
                .tokens
                .iter()
                .rev()
                .position(|token| &token.kind == kind)
                .expect("cannot justify claim without any And operators using And Intoduction")
    }

    pub fn operands_of_operator_at_index(&self, operator_index: usize) {
        if operator_index > self.tokens.len() - 1 {
            panic!("operator_index out of bounds of token vec");
        }
        let operand = self.tokens.get(operator_index).unwrap();
        if !operand.kind.is_operator() {
            panic!("token at operator_index as not an operator");
        }
        let before_operator = &self.tokens[0..operator_index];
        let sub_expression: Vec<Expression>;
    }
}

pub fn expression_in_vec(expressions: &Vec<Expression>, expression: &Expression) -> bool {
    expressions.iter().find(|exp| exp == &expression).is_some()
}

fn infix_to_posfix(tokens: Vec<Token>) -> Vec<Token> {
    let mut stack: Vec<Token> = Vec::new();
    let mut posfix: Vec<Token> = Vec::new();

    for token in tokens {
        match token.kind {
            TokenKind::LeftParentheses => stack.push(token),
            TokenKind::RightParentheses => {
                loop {
                    assert!(stack.len() > 0, "mismatched parentheses");
                    match stack[stack.len() - 1].kind {
                        TokenKind::LeftParentheses => break,
                        _ => posfix.push(stack.pop().unwrap()),
                    }
                    /* if there is a function token at the top of the operator stack, then:
                    pop the function from the operator stack into the output queue */
                }
                stack.pop();
            }
            TokenKind::Z(_) => posfix.push(token),
            TokenKind::B(_) => posfix.push(token),
            ref kind if kind.is_operator() => {
                if kind.is_unary() {
                    panic!("unary ops not yet implemented")
                }
                while stack.len() > 0 {
                    match &stack[stack.len() - 1].kind {
                        stack_op_kind
                            if stack_op_kind.is_operator()
                                && stack_op_kind.precedes(&kind) == Ordering::Greater =>
                        {
                            posfix.push(stack.pop().unwrap())
                        }
                        _ => break,
                    }
                }
                stack.push(token);
            }
            _ => panic!("not done yet"), // TODO: infix to posfix other types
        }
    }
    while stack.len() > 0 {
        let stack_token = stack.pop().unwrap();
        match stack_token.kind {
            TokenKind::LeftParentheses => panic!("mismatched parentheses"),
            _ => (),
        }
        posfix.push(stack_token)
    }
    posfix
}
