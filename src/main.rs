use std::collections::HashMap;

enum Literal {
    B(bool)
    Z(i32)
}

type VarName = String;

enum Premise {
    Equal(VarName, Literal),
    NotEqual(VarName, Literal),
    Implies(VarName, VarName),
}

struct Line {
    line_number: i32,
    claim: VarName,
    justification: String,
    proven: bool,
}

impl Line {
    fn prove(&self, premises: Vec<Premise>, justifying_lines: Vec<Line>) {
    }
}

fn main() {

}
