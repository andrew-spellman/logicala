pub enum ValueKind {
    Bool,
}

pub struct Identifier {
    id: String,
    kind: ValueKind,
}

pub enum ClaimToken {
    BoolLiteral(bool),
    Identifier(Identifier),
    LeftParenthese,
    RightParenthese,
    Implication,
    Disjunction,
    Conjunction,
    Negation,
}

impl ClaimToken {
    fn is_value(&self) -> bool {
        use ClaimToken::*;
        match self {
            BoolLiteral(_) | Identifier(_) => true,
            LeftParenthese | RightParenthese | Implication | Disjunction | Conjunction
            | Negation => false,
        }
    }

    fn is_operator(&self) -> bool {
        use ClaimToken::*;
        match self {
            Implication | Disjunction | Conjunction | Negation => true,
            BoolLiteral(_) | Identifier(_) | LeftParenthese | RightParenthese => false,
        }
    }

    fn is_unary_operator(&self) -> bool {
        use ClaimToken::*;
        match self {
            Negation => true,
            BoolLiteral(_) | Identifier(_) | LeftParenthese | RightParenthese | Implication
            | Disjunction | Conjunction => false,
        }
    }

    fn is_binary_operator(&self) -> bool {
        use ClaimToken::*;
        match self {
            Implication | Disjunction | Conjunction => true,
            BoolLiteral(_) | Identifier(_) | LeftParenthese | RightParenthese | Negation => false,
        }
    }

    fn precedence(&self) -> usize {
        use ClaimToken::*;
        match self {
            BoolLiteral(_) | Identifier(_) | LeftParenthese | RightParenthese => 0,
            Implication => 1,
            Disjunction => 2,
            Conjunction => 3,
            Negation => 4,
        }
    }
}

pub struct Claim {
    tokens: Vec<ClaimToken>,
}

impl Claim {
    pub fn from_tokens(tokens: Vec<ClaimToken>) -> Self {
        use ClaimToken::*;
        let mut posfix = Vec::new();
        let mut operator_stack: Vec<ClaimToken> = Vec::new();
        for token in tokens {
            match token {
                token if token.is_value() => posfix.push(token),

                token if token.is_operator() => {
                    while match operator_stack.last() {
                        Some(operator) => operator.precedence() > token.precedence(),
                        None => false,
                    } {
                        posfix.push(operator_stack.pop().unwrap());
                    }
                    operator_stack.push(token);
                }

                LeftParenthese => operator_stack.push(token),
                RightParenthese => loop {
                    match operator_stack.last() {
                        Some(LeftParenthese) => {
                            _ = operator_stack.pop().unwrap();
                            break;
                        }
                        Some(_) => posfix.push(operator_stack.pop().unwrap()),
                        None => panic!("unmatched right parethese"),
                    }
                },

                BoolLiteral(_) | Identifier(_) | Negation | Disjunction | Conjunction
                | Implication => unreachable!(),
            }
        }

        // TODO: make certain posfix evaluates to a bool

        Self { tokens: posfix }
    }
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
