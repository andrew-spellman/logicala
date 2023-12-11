pub enum Claim {
    TrueLiteral,
    FalseLiteral,
    Identifier(String),
    Parenthesized(Box<Claim>),
    Negation(Box<Claim>),
    Conjunction(Box<Claim>),
    DisJunction(Box<Claim>),
    Implication(Box<Claim>),
}

struct RegularStepNumber(usize);
struct SubProofNumber(usize);

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

struct RegularStep {
    number: RegularStepNumber,
    claim: Claim,
    justification: Justification,
}

struct AssumeStep {
    number: RegularStepNumber,
    claim: Claim,
}

struct SubProof {
    number: SubProofNumber,
    assume: AssumeStep,
    steps: Vec<ProofStep>,
}

enum ProofStep {
    Regular(RegularStep),
    Sub(SubProof),
}

struct Proof {
    steps: Vec<ProofStep>,
}

pub struct Sequent {
    pub premises: Vec<Claim>,
    pub conclusions: Vec<Claim>,
}

struct Propositional {
    sequent: Option<Sequent>,
    proof: Proof,
}
