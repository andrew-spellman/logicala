/*

use std::collections::HashMap;

enum Literal {
    B(bool),
    Z(i32),
}

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
    fn prove(&self, premises: Vec<Premise>, proven_lines: Vec<Line>) {
    }
}

struct Assumption {
    assume:
}

struct ProofBlock {
    premises: Vec<Premise>,
    lines: Vec<Line>,
}

*/
