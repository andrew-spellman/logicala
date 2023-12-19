#![allow(dead_code)]

struct Propositional {
    sequent: Sequent,
    proof: Proof,
}

struct Sequent {
    premises: ClaimList,
    conclusions: ClaimList,
}

struct ClaimList {
    claims: Vec<Claim>,
}

struct Proof {
    steps: Vec<ProofStep>,
}

struct RegularStepNumber(usize);
struct SubProofNumber(usize);

enum ProofStep {
    RegularStep {
        number: RegularStepNumber,
        claim: Claim,
        just: Justification,
    },
    SubProof {
        number: SubProofNumber,
        assume: AssumeStep,
        steps: Box<Vec<ProofStep>>,
    },
}

struct AssumeStep {
    number: RegularStepNumber,
    claim: Claim,
}

#[derive(Debug)]
pub enum Claim {
    BoolLiteral(bool),
    Identifier(String),
    Negation(Box<Claim>),
    Conjunction(Box<Claim>, Box<Claim>),
    Disjunction(Box<Claim>, Box<Claim>),
    Implication(Box<Claim>, Box<Claim>),
}

enum Justification {
    AndIntroduction(RegularStepNumber, RegularStepNumber),
    AndElimination1(RegularStepNumber),
    AndElimination2(RegularStepNumber),
    OrIntroduction1(RegularStepNumber),
    OrIntroduction2(RegularStepNumber),
    OrElimination(RegularStepNumber, SubProofNumber, SubProofNumber),
    ImplicationIntroduction(SubProofNumber),
    ImplicationElimination(RegularStepNumber, RegularStepNumber),
    NegationIntroduction(SubProofNumber),
    NegationElimination(RegularStepNumber, RegularStepNumber),
    ContradictionElimination(RegularStepNumber),
    ProofByContradiction(SubProofNumber),
}
