//https://logika.v3.sireum.org/doc/03-language/propositional/index.html#justification-and-inference-rules
pub enum Justification {
    Premise,
    AndIntroduction(usize, usize),
    AndElimination1(usize),
    AndElimination2(usize),
    OrIntroduction1(usize),
    OrIntroduction2(usize),
    OrElimination(usize, usize, usize),
    ImplicationIntroduction(usize),
    ImplicationElimination(usize, usize),
    NegationIntroduction(usize),
    NegationElimination(usize, usize),
    ContraditionElimination(usize),
    ProofByContradiction(usize),
}
