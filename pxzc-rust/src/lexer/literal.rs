use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub(crate) enum Literal<'a> {
    Number(f64),
    String(&'a str),
    Boolean(bool),
    None,
}

impl fmt::Display for Literal<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Literal::Number(n) => write!(f, "{}", n),
            Literal::String(s) => write!(f, "{}", s),
            Literal::Boolean(b) => write!(f, "{}", b),
            Literal::None => write!(f, "none")
        }
    }
}
