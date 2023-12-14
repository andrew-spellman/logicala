use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
enum Token {
    #[regex(" +")]
    WhiteSpace,

    #[token("\n")]
    Newline,

    #[regex("[0-9]+")]
    Integer,

    #[regex("[a-zA-Z]+")]
    Identifier,

    #[token(":")]
    Colon,
    #[token(",")]
    Comma,
    #[token(".")]
    Period,

    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,

    #[token("assume")]
    Assume,
    #[token("premise")]
    Premise,

    #[token("⊤")]
    TrueLiteral,
    #[token("⊥")]
    FalseLiteral,
    #[token("¬")]
    Negation,
    #[token("∧")]
    Conjunction,
    #[token("∨")]
    Disjunction,
    #[token("→")]
    Implication,
    #[regex("(⊢|---+)")]
    Turnstile,

    #[token("∧i")]
    AndIntroduction,
    #[token("∧e1")]
    AndElimination1,
    #[token("∧e2")]
    AndElimination2,
    #[token("∨i1")]
    OrIntroduction1,
    #[token("∨i2")]
    OrIntroduction2,
    #[token("∨e")]
    OrElimination,
    #[token("→i")]
    ImplicationIntroduction,
    #[token("→e")]
    ImplicationElimination,
    #[token("¬i")]
    NegationIntroduction,
    #[token("¬e")]
    NegationElimination,
    #[token("⊥e")]
    ContradictionElimination,
    #[token("pbc")]
    ProofByContradiction,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_token(
        lex: &mut logos::Lexer<Token>,
        token: Token,
        span: std::ops::Range<usize>,
        slice: &str,
    ) {
        assert_eq!(lex.next(), Some(Ok(token)));
        assert_eq!(lex.span(), span);
        assert_eq!(lex.slice(), slice);
    }

    #[test]
    fn propositional() {
        let mut lex = Token::lexer("p, q ⊢ q\n{\n  13. q   premise\n}");

        assert_token(&mut lex, Token::Identifier, 0..1, "p");
        assert_token(&mut lex, Token::Comma, 1..2, ",");
        assert_token(&mut lex, Token::WhiteSpace, 2..3, " ");
        assert_token(&mut lex, Token::Identifier, 3..4, "q");
        assert_token(&mut lex, Token::WhiteSpace, 4..5, " ");
        assert_token(&mut lex, Token::Turnstile, 5..8, "⊢");
        assert_token(&mut lex, Token::WhiteSpace, 8..9, " ");
        assert_token(&mut lex, Token::Identifier, 9..10, "q");
        assert_token(&mut lex, Token::Newline, 10..11, "\n");
        assert_token(&mut lex, Token::LeftBrace, 11..12, "{");
        assert_token(&mut lex, Token::Newline, 12..13, "\n");
        assert_token(&mut lex, Token::WhiteSpace, 13..15, "  ");
        assert_token(&mut lex, Token::Integer, 15..17, "13");
        assert_token(&mut lex, Token::Period, 17..18, ".");
        assert_token(&mut lex, Token::WhiteSpace, 18..19, " ");
        assert_token(&mut lex, Token::Identifier, 19..20, "q");
        assert_token(&mut lex, Token::WhiteSpace, 20..23, "   ");
        assert_token(&mut lex, Token::Premise, 23..30, "premise");
        assert_token(&mut lex, Token::Newline, 30..31, "\n");
        assert_token(&mut lex, Token::RightBrace, 31..32, "}");
        assert_eq!(lex.next(), None);
        assert_eq!(lex.span(), 32..32);
        assert_eq!(lex.slice(), "");
    }
}
