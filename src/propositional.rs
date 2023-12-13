#[derive(Debug, Clone, PartialEq)]
pub enum Kind {
    Bool,
}

#[derive(Clone)]
pub struct Identifier {
    pub id: String,
    pub kind: Kind,
}

#[derive(Clone)]
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

    fn operand_kinds(&self) -> Vec<Kind> {
        use ClaimToken::*;
        match self {
            BoolLiteral(_) | Identifier(_) | LeftParenthese | RightParenthese => vec![],
            Negation => vec![Kind::Bool],
            Implication | Disjunction | Conjunction => {
                vec![Kind::Bool, Kind::Bool]
            }
        }
    }

    fn operator_result_kind(&self) -> Option<Kind> {
        use ClaimToken::*;
        match self {
            BoolLiteral(_) | Identifier(_) | LeftParenthese | RightParenthese => None,
            Negation | Implication | Disjunction | Conjunction => Some(Kind::Bool),
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

struct Posfix {
    tokens: Vec<ClaimToken>,
}

impl Posfix {
    fn from_tokens(tokens: Vec<ClaimToken>) -> Vec<ClaimToken> {
        use ClaimToken::*;
        let mut posfix: Vec<ClaimToken> = Vec::new();
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
        posfix
    }
}

pub struct Claim {
    tokens: Vec<ClaimToken>,
    kind: Kind,
}

impl Claim {
    pub fn from_posfix(posfix: Posfix) -> Self {
        assert!(Self::evaluate_kind(&posfix));
        Self {
            tokens: posfix.tokens,
            kind: Kind::Bool,
        }
    }

    fn evaluate_kind(posfix: &Posfix) -> bool {
        use ClaimToken::*;

        enum TokenOrExpression<'a> {
            Token(&'a ClaimToken),
            Expression(Expression<'a>),
        }

        struct Expression<'a> {
            tokens: Vec<TokenOrExpression<'a>>,
            kind: Kind,
        }

        let mut tokens_or_expressions: Vec<TokenOrExpression> = posfix
            .tokens
            .iter()
            .map(|token| TokenOrExpression::Token(token))
            .collect();

        loop {
            let Some((first_operator, operator_index)) = tokens_or_expressions
                .iter()
                .enumerate()
                .find_map(|x| match x {
                    (index, TokenOrExpression::Token(token)) if token.is_operator() => {
                        Some((token.clone(), index))
                    }
                    _ => None,
                })
            else {
                break;
            };

            let expected_operand_kinds = first_operator.operand_kinds();

            let operand_count = expected_operand_kinds.len();
            if operator_index < operand_count - 1 {
                panic!("operator missing operands");
            }

            let start_index = operator_index - operand_count;

            for (index, expected_kind) in expected_operand_kinds.iter().enumerate() {
                let found_kind = match &tokens_or_expressions[start_index + index] {
                    TokenOrExpression::Token(token) if token.is_value() => match token {
                        BoolLiteral(_) => &Kind::Bool,
                        Identifier(identifier) => &identifier.kind,
                        LeftParenthese | RightParenthese | Implication | Disjunction
                        | Conjunction | Negation => unreachable!(),
                    },
                    TokenOrExpression::Expression(exp) => &exp.kind,
                    TokenOrExpression::Token(_) => unreachable!(),
                };
                if found_kind != expected_kind {
                    panic!(
                        "found kind {:?}, expected kind {:?}",
                        found_kind, expected_kind
                    );
                }
            }
            let new_expression_tokens_or_expressions = (0..operand_count + 1)
                .map(|_| tokens_or_expressions.remove(start_index))
                .collect();
            let Some(result_kind) = first_operator.operator_result_kind() else {
                panic!();
            };
            let new_expression = Expression {
                tokens: new_expression_tokens_or_expressions,
                kind: result_kind,
            };
            tokens_or_expressions
                .insert(start_index, TokenOrExpression::Expression(new_expression));
        }

        false
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
