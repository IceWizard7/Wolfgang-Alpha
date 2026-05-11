use std::fmt;
use crate::math::expressions::Expression;

#[derive(Clone, Copy, PartialEq)]
pub enum Comparison { Eq, Gt, Ge, Lt, Le }
impl Comparison {
    fn to_string(&self) -> &str {
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
        write!(f, "{}", self.to_string())
    }
}
impl fmt::Debug for Comparison {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
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
    Comp(Comparison, Option<Box<Expression>>)
}
impl BinaryOperation {
    fn to_string(&self) -> &str {
        match self {
            BinaryOperation::Add => "+",
            BinaryOperation::Sub => "-",
            BinaryOperation::Mul => "*",
            BinaryOperation::Div => "/",
            BinaryOperation::Quo => "//",
            BinaryOperation::Rem => "%",
            BinaryOperation::Pow => "^",
            BinaryOperation::Comp(c, _) => c.to_string(),
        }
    }
}
impl fmt::Display for BinaryOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
impl fmt::Debug for BinaryOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum UnaryOperation {
    Neg,
    Abs,
}