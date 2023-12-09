#![allow(dead_code)]

use crate::justification::Justification;
use crate::parser::{expression_in_vec, Expression};
use crate::tokenizer::TokenKind;

fn find_step_by_number<'a>(steps: &'a Vec<ProofStep>, step_number: usize) -> &'a ProofStep {
    steps
        .iter()
        .find(|step| step.line_number == step_number)
        .expect(&format!("step {} didn't exist", step_number))
}

struct ProofStep {
    line_number: usize,
    claim: Expression,
    justification: Justification,
    proven: bool,
}

impl ProofStep {
    fn prove(&mut self, premises: &Vec<Expression>, previous_steps: &Vec<ProofStep>) {
        use Justification::*;
        self.proven = match self.justification {
            Premise => expression_in_vec(&premises, &self.claim),
            AndIntroduction(first_step_number, second_step_number) => {
                let and_index = &self
                    .claim
                    .last_position_of_token_kind_in_expression(&TokenKind::And);
                let first_step = find_step_by_number(&previous_steps, first_step_number);
                let second_step = find_step_by_number(&previous_steps, second_step_number);
                // TODO slice claim into two arguements using and_index and compare to first_step and second_step
                false
            }
            _ => false, // TODO
        };
    }
}

struct SubProof {
    assume: Expression,
}

struct Proof {
    claim: Expression,
    premises: Vec<Expression>,
    lines: Vec<ProofStep>, // must all have unique line_numbers
}

impl Proof {
    fn prove(&mut self) -> bool {
        return false;
    }
}

// TODO sequents
