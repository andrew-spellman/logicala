/*
use crate::justification::Justification;
use crate::literal::Literal;
use crate::tokenizer::Token;
use std::collections::HashMap;

type VarName = String;
type Expression = Vec<Token>;

enum Premise {
    Equal(VarName, Literal),
    NotEqual(VarName, Literal),
    Implies(VarName, VarName),
}

struct Line {
    line_number: i32,
    claim: Expression,
    justification: String,
    proven: bool,
}

impl Line {
    fn prove(&self, premises: Vec<Premise>, proven_lines: Vec<Line>) {}
}

struct Assumption {}

struct ProofBlock {
    premises: Vec<Premise>,
    lines: Vec<Line>,
    claim: Expression,
}
*/
