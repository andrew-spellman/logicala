/// # Proof
/// A propositional logic block that contains proofs backed by claims. Each `ProofStep` contains a
/// unique line number, and is either a regular non-subblock proof or a sunblock proof consisting
/// of one or more other proof items. This logic block contains all needed information to solve
/// (prove) a statement is correct, and provide adequate error handling in the case of failure.
#[derive(Debug)]
pub struct Proof {
    pub sequent: Sequent,
    pub proof_steps: Vec<ProofStep>,
}

pub type LineNumber = usize;
pub type SubLineNumber = usize;
pub type Ident = String;

/// # Proof Step
/// A step in the propositional claim consisting of types of possible claims. Each claim step
/// is used to prove a overall proof.
#[derive(Debug, Clone)]
pub enum ProofStep {
    /// ## Regular Step
    /// A step that is just simply one step. Consiting of a line number, claim, and just.
    RegularStep {
        line_number: LineNumber,
        claim: Claim,
        just: Justification,
    },
    /// ## Sub Proof
    /// A proof block inside of a proof block. Used for verifing other steps that require
    /// all possible cases to be verified. Consists of a sub-line number, assume, and finally
    /// the inner proof block. The inner proof block can contain one or more sub-proofs of
    /// its own.
    SubProof {
        sub_line_number: LineNumber,
        assume_step: (LineNumber, AssumeStep),
        inner_proof: Box<ProofStep>,
    },
}

/// # Sequent
/// A Sequent is the outer scope items of a proof step. Sequents contain some `n` number of knowns
/// which will aid in proving the `to_verify` (final claim) of your proof.
#[derive(Debug, Clone)]
pub struct Sequent {
    /// ## Known
    /// Knowns are the claims you know hold to be true.
    pub known: Vec<Claim>,
    /// ## To Verify
    /// `to_verify` is the final goal or outcome of a logic block.
    pub to_verify: Claim,
}

/// # Assume Step
/// A type of assume that a sub-poof can handle. Assumes are used for verifying that
/// all cases withing a logic step are correct.
#[derive(Debug, Clone)]
pub enum AssumeStep {
    /// ## Assume Claim
    /// Assume that this claim is valid
    AssumeClaim(Claim),
    /// ## Assume Fresh
    /// Create a new fresh used for single member manipulation.
    AssumeFresh(Ident),
}

/// # Literal Container
/// A type of literal that also contains the literal
#[derive(Debug, Clone, PartialEq)]
pub enum LiteralContainer {
    Bool(bool),
    Int(i64),
    Float(f64),
    Seq(Vec<LiteralContainer>),
}

/// # Claim
/// A Claim is something that can be proven by logic. Claims are statements that
/// express some logical intention.
#[derive(Debug, Clone, PartialEq)]
pub enum Claim {
    Literal(LiteralContainer),
    Identifier(Ident),
    InnerClaim(Box<Claim>),
    Negate(Box<Claim>),
    Plus(Box<Claim>, Box<Claim>),
    Minus(Box<Claim>, Box<Claim>),
    Mult(Box<Claim>, Box<Claim>),
    And(Box<Claim>, Box<Claim>),
    Or(Box<Claim>, Box<Claim>),
    Imply(Box<Claim>, Box<Claim>),
}

/// # Claim Part
/// The part in-which this justification is acting upon.
#[derive(Debug, Clone, PartialEq)]
pub enum ClaimPart {
    /// ## First Part
    /// The first part of the claim.
    First,
    /// ## Second Part
    /// THe second part of the claim.
    Second,
}

/// # Justification
/// The proof (reason) why something is true. A justification for which a proof is
/// known to be true.
// TODO: Finish Docs
#[derive(Debug, Clone, PartialEq)]
pub enum Justification {
    /// # Premise
    /// Something you know is true at the start.
    Premise,
    AndI(LineNumber, LineNumber),
    AndE(ClaimPart, LineNumber),
    OrI(ClaimPart, LineNumber),
    OrE(LineNumber, SubLineNumber, SubLineNumber),
    ImpI(SubLineNumber),
    ImpE(LineNumber, SubLineNumber),
    NegI(SubLineNumber),
    NegE(LineNumber, SubLineNumber),
    ContE(LineNumber),
    ProofByContradiction(SubLineNumber),
    ForallI(SubLineNumber),
    ForallE(LineNumber, Box<Claim>),
    ExistI(LineNumber, Box<Claim>),
    ExistE(LineNumber, SubLineNumber),
    Fact(Ident),
    Invarient,
    Subset(ClaimPart, LineNumber, LineNumber),
}
