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

enum Op {
    Conjunction,
    Disjunction,
    Implication,
}

enum Claim {
    TrueLiteral,
    FalseLiteral,
    Identifier,
    Negation(Box<Claim>),
    BinOp {
        lhs: Box<Claim>,
        op: Op,
        rhs: Box<Claim>,
    },
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
