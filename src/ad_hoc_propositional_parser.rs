use crate::propositional;
use crate::repeats_no_whitespace::RepeatsNoWhiteSpace;
use std::fs::File;

fn parse_propositional(file: File) {
    let mut reader = RepeatsNoWhiteSpace::new(Box::new(file));

    match reader.get() {
        None => panic!("file was empty"),
        _ => (),
    }

    while match reader.get() {
        Some(' ') | Some('\n') => true,
        Some(_) => false,
        None => panic!("file was entirely whitespace"),
    } {
        reader.next();
    }

    let sequent = match reader.get() {
        Some('{') => None,
        Some(' ') | Some('\n') | None => unreachable!(),
        Some(_) => parse_sequent(&mut reader),
    };
}

fn parse_sequent(reader: &mut RepeatsNoWhiteSpace) -> Option<propositional::Sequent> {
    let premises: Vec<propositional::Claim> = Vec::new();
    let conclusions: Vec<propositional::Claim> = Vec::new();

    while match reader.get() {
        Some('⊢') => false,
        Some('{') => panic!("expected sequent"),
        _ => true,
    } {
        premises.push(parse_claim(&mut reader));
    }

    todo!("conclusions")

    Some(propositional::Sequent {
        premises,
        conclusions,
    })
}

fn parse_claim(reader: &mut RepeatsNoWhiteSpace) -> Option<propositional::Claim> {
    use propositional::Claim::*;
    None
}

enum ClaimToken {
    TrueLiteral,
    FalseLiteral,
    Negation,
    Conjunction,
    Disjunction,
    Implication,
    LeftParenthese,
    RightParenthese,
    Identifier(String),
}

fn tokenize_claim(reader: &mut RepeatsNoWhiteSpace) -> Vec<ClaimToken> {
    use ClaimToken::*;
    let mut tokens = Vec::new();
    loop {
        match reader.get() {
            Some('⊤') => {
                tokens.push(TrueLiteral);
                reader.next();
            }
            Some('⊥') => {
                tokens.push(FalseLiteral);
                reader.next();
            }
            Some('¬') => {
                tokens.push(Negation);
                reader.next();
            }
            Some('∧') => {
                tokens.push(Conjunction);
                reader.next();
            }
            Some('∨') => {
                tokens.push(Disjunction);
                reader.next();
            }
            Some('→') => {
                tokens.push(Implication);
                reader.next();
            }
            Some('(') => {
                tokens.push(LeftParenthese);
                reader.next();
            }
            Some(')') => {
                tokens.push(RightParenthese);
                reader.next();
            }
            Some(c) if c.is_ascii_alphabetic() => {
                let id = Identifier(parse_ascii_alphabetic(&mut reader));
            }
            todo!("error cases")
        }
    }
}

fn parse_ascii_alphabetic(reader: &mut RepeatsNoWhiteSpace) -> String {
    let mut result = String::new();
    while reader.get().is_some_and(|c| c.is_ascii_alphabetic()) {
        result.push(reader.next().unwrap());
    }
    result
}
