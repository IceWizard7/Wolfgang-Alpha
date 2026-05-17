use std::fmt;
use crate::math::expressions::Expression;

#[derive(Clone, Copy, PartialEq)]
pub enum Comparison { Eq, Gt, Ge, Lt, Le }
impl Comparison {
    fn as_str(&self) -> &str {
        match self {
            Comparison::Eq => "=",
            Comparison::Gt => ">",
            Comparison::Ge => ">=",
            Comparison::Lt => "<",
            Comparison::Le => "<=",
        }
    }
}
impl fmt::Display for Comparison {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
impl fmt::Debug for Comparison {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Clone, PartialEq)]
pub enum BinaryOperation {
    Add,
    Sub,
    Mul,
    Div,
    Quo,
    Rem,
    Pow,
    And,
    Or,
    Comp(Comparison, Option<Box<Expression>>)
}
impl BinaryOperation {
    pub fn as_str(&self) -> &str {
        match self {
            BinaryOperation::Add => "+",
            BinaryOperation::Sub => "-",
            BinaryOperation::Mul => "*",
            BinaryOperation::Div => "/",
            BinaryOperation::Quo => "//",
            BinaryOperation::Rem => "%",
            BinaryOperation::Pow => "^",
            BinaryOperation::And => "&&",
            BinaryOperation::Or => "||",
            BinaryOperation::Comp(c, _) => c.as_str(),
        }
    }
    pub fn priority(&self) -> u8 {
        match self {
            BinaryOperation::Add => 5,
            BinaryOperation::Sub => 5,
            BinaryOperation::Mul => 6,
            BinaryOperation::Div => 6,
            BinaryOperation::Quo => 6,
            BinaryOperation::Rem => 6,
            BinaryOperation::Pow => 7,
            BinaryOperation::And => 2,
            BinaryOperation::Or => 1,
            BinaryOperation::Comp(..) => 4,
        }
    }
}
impl fmt::Display for BinaryOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
impl fmt::Debug for BinaryOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum UnaryOperation {
    Neg,
    Not,
    Factorial,
    Abs,
}

impl fmt::Display for UnaryOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnaryOperation::Neg => write!(f, "-"),
            UnaryOperation::Not => write!(f, "!"),
            UnaryOperation::Factorial => write!(f, "!"),
            UnaryOperation::Abs => write!(f, "|_|"),
        }
    }
}
impl UnaryOperation {
    /// Example: applied to `UnaryOperation::Neg` and some vector `v`,
    /// adds '-' at the beginning of `v[0]`.
    pub fn format_with_multline_expr(&self, expr: &mut [String]) {
        match self {
            UnaryOperation::Neg => expr[0].insert(0, '-'),
            UnaryOperation::Not => expr[0].insert(0, '!'),
            UnaryOperation::Factorial => expr.last_mut().unwrap().push('!'),
            UnaryOperation::Abs => {expr[0].insert(0, '|'); expr.last_mut().unwrap().push('|');},
        }
    }
}