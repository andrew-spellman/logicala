use crate::propositional::Claim;
use chumsky::prelude::*;

pub fn parser() -> impl Parser<char, Claim, Error = Simple<char>> {
    recursive(|claim| {
        let bool_literal = {
            let just_false = just('⊥').map(|_c| Claim::BoolLiteral(false));
            let just_true = just('⊤').map(|_c| Claim::BoolLiteral(true));
            let kw_false = text::keyword("false").map(|()| Claim::BoolLiteral(false));
            let kw_true = text::keyword("true").map(|()| Claim::BoolLiteral(true));
            just_false.or(just_true).or(kw_false).or(kw_true).padded()
        };

        let ident = text::ident()
            .map(|s: String| Claim::Identifier(s))
            .padded()
            .validate(|s, span, emit| {
                impl Claim {
                    fn get_ident_str(&self) -> &str {
                        match self {
                            Self::Identifier(s) => s,
                            _ => unreachable!(),
                        }
                    }
                }
                let id = s.get_ident_str();
                match id {
                    "false" | "true" | "not" | "neg" | "and" | "or" | "implies" => {
                        emit(Simple::custom(
                            span,
                            format!("Expected identifier, found keyword '{}'", id),
                        ))
                    }
                    _ => (),
                }
                s
            });

        let bool_atom = bool_literal
            .or(ident)
            .or(claim.delimited_by(just('('), just(')')))
            .padded();

        let negation = {
            let logic_not = just('¬').map(|_| ());
            let kw_not = text::keyword("not").map(|_| ());
            let kw_neg = text::keyword("neg").map(|_| ());
            let exclamation = just('!').map(|_| ());
            let tilde = just('~').map(|_| ());
            let op_negation = logic_not
                .or(kw_not)
                .or(kw_neg)
                .or(exclamation)
                .or(tilde)
                .padded();
            op_negation
                .repeated()
                .then(bool_atom)
                .foldr(|_op, rhs| Claim::Negation(Box::new(rhs)))
        };

        let conjunction = {
            let logic_conj = just('∧').map(|_| ());
            let ampersand = just('&').map(|_| ());
            let kw_and = text::keyword("and").map(|_| ());
            let caret = just('^').map(|_| ());
            let op_conjunction = logic_conj.or(ampersand).or(kw_and).or(caret).padded();
            negation
                .clone()
                .then(op_conjunction.then(negation).repeated())
                .foldl(|lhs, ((), rhs)| Claim::Conjunction(Box::new(lhs), Box::new(rhs)))
        };

        let disjunction = {
            let logic_disj = just('∨').map(|_| ());
            let pipe = just('|').map(|_| ());
            let kw_or = text::keyword("or").map(|_| ());
            let capital_v = just('V').map(|_| ());
            conjunction
                .clone()
                .then(
                    logic_disj
                        .or(pipe)
                        .or(kw_or)
                        .or(capital_v)
                        //.padded()
                        .then(conjunction)
                        .repeated(),
                )
                .foldl(|lhs, ((), rhs)| Claim::Disjunction(Box::new(lhs), Box::new(rhs)))
        };

        let implies = {
            let logic_implies = just('→').map(|_| ());
            let thin_arrow = text::keyword("->").map(|_| ());
            let kw_implies = text::keyword("implies").map(|_| ());
            disjunction
                .clone()
                .then(
                    logic_implies
                        .or(thin_arrow)
                        .or(kw_implies)
                        //.padded()
                        .then(disjunction)
                        .repeated(),
                )
                .foldl(|lhs, ((), rhs)| Claim::Implication(Box::new(lhs), Box::new(rhs)))
        };

        implies
    })
    .then_ignore(end())
}
