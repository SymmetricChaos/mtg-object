use std::fmt::{self, Display};

/// A symbol representing a number. Most numbers are integer but a *, X, or Y can also appear in a number position.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Number {
    N(i32),
    Star,
    X,
    Y,
}

impl Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Number::N(n) => write!(f, "{}", n),
            Number::Star => write!(f, "*"),
            Number::X => write!(f, "X"),
            Number::Y => write!(f, "Y"),
        }
    }
}
