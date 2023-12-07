use std::cmp::Ordering;

// https://logika.v3.sireum.org/doc/03-language/basic/index.html#operators-and-literals
#[derive(Debug, PartialEq)]
pub enum Operator {
    Or,
    And,
    Less,
    LessEquals,
    Greater,
    GreaterEquals,
    Equals,
    NotEquals,
    Not,
    Plus, // TODO: Prepend and Postpend Plus
    Minus,
    Multiply,
    Modulus,
    Divide,
    // TODO: Figure out if we need Exponent
    Negate,
}

impl Operator {
    pub const fn precedence(&self) -> usize {
        use Operator::*;
        match self {
            Or => 0,
            And => 1,
            Less => 2,
            LessEquals => 2,
            Greater => 2,
            GreaterEquals => 2,
            Equals => 3,
            NotEquals => 3,
            Not => 3,
            Plus => 4,
            Minus => 4,
            Multiply => 5,
            Divide => 5,
            Modulus => 5,
            Negate => 6,
        }
    }

    pub fn precedes(&self, other: &Self) -> Ordering {
        match (self.precedence(), other.precedence()) {
            (a, b) if a < b => Ordering::Less,
            (a, b) if a == b => Ordering::Equal,
            (a, b) if a > b => Ordering::Greater,
            _ => unreachable!(),
        }
    }

    pub fn is_unary(&self) -> bool {
        match self {
            Self::Not => true,
            Self::Negate => true,
            _ => false,
        }
    }

    pub fn is_double_width(&self) -> bool {
        use Operator::*;
        match self {
            LessEquals => true,
            GreaterEquals => true,
            Equals => true,
            NotEquals => true,
            _ => false,
        }
    }
}
