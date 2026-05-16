use std::collections::HashMap;
use std::f64::consts;
use crate::math::Expression;
use crate::math::expressions;
use crate::math::{Object, FunctionRepr};
use crate::math::operations::{Comparison, UnaryOperation, BinaryOperation};
use crate::{expr_if_else, expr_and, expr_compare, expr_sub, expr_mul, expr_div, expr_square, expr_neg, expr_1arg_func};

/// Wrapped in a function because const hashmaps aren't available yet.
pub fn default_constants() -> HashMap<String, Object> {
    HashMap::<String, Object>::from([
        ("e".to_string(), Object::Float(consts::E)),
        ("pi".to_string(), Object::Float(consts::PI)),
    ])
}

/// Takes a function name `name`, e.g. `exp`, and returns the tuple consisting of
/// 1. Stringified name of the function
/// 2. `FunctionRepr::Direct`: expect exactly one `f64` as argument; if so, return `Ok(x.name())`, otherwise, the appropriate `Err`.
macro_rules! float_1_function {
    ($name:ident) => {
        (
            stringify!($name).to_string(),
            FunctionRepr::Direct(Box::new(|args| {
                if args.len() != 1 {
                    Err(format!(
                        "Wrong number of arguments provided for function '{}' (expected 1, got {}).",
                        stringify!($name),
                        args.len()
                    ))
                } else {
                    match args[0] {
                        Object::Float(x) => Ok(Object::Float(x.$name())),
                        _ => Err(format!(
                            "Wrong type of argument provided for function '{}' (expected float).",
                            stringify!($name)
                        )),
                    }
                }
            })),
        )
    };
}

pub const DEFAULT_FUNCTION_NAMES: [&str; 16] = [
    "exp", "ln", "log",
    "sqrt",
    "cos", "cosh", "acos", "acosh",
    "sin", "sinh", "asin", "asinh",
    "tan", "tanh", "atan", "atanh"
];

/// Wrapped in a function because const hashmaps aren't available yet.
pub fn default_functions() -> HashMap<String, FunctionRepr> {
    HashMap::<String, FunctionRepr>::from([
        float_1_function!(exp),
        float_1_function!(ln),
        ("log".to_string(), FunctionRepr::Direct(Box::new(|args| {
            if args.len() != 2 { Err(format!("Wrong number of arguments provided for function 'log' (expected 2 [value, base], got {}).", args.len())) }
            else {
                if let Object::Float(base) = args[1] {
                    match args[0] {
                        Object::Float(x) => Ok(Object::Float(x.log(base))),
                        _ => Err("Wrong type for first argument (value) of function 'log' (expected float).".to_string())
                    }
                }
                else { Err("Wrong type for second argument (base) of function 'log' (expected float).".to_string()) }
            }
        }))),
        float_1_function!(sqrt),
        float_1_function!(cos), float_1_function!(cosh), float_1_function!(acos), float_1_function!(acosh),
        float_1_function!(sin), float_1_function!(sinh), float_1_function!(asin), float_1_function!(asinh),
        float_1_function!(tan), float_1_function!(tanh), float_1_function!(atan), float_1_function!(atanh),
    ])
}

/// Example: (exp, point) => Ok(Expression::Function("exp", point[0].clone())) if point has length 1 otherwise Err
macro_rules! get_default_derivative_macro1 {
    ($name:ident, $point:expr, $direction:expr) => {
        if $point.len() != 1 {
            Err(format!(
                "Wrong number of arguments provided for derivative of '{}' (expected 1, got {}).",
                stringify!($name),
                $point.len()
            ))
        } else {
            Ok(expressions::simplify_mul(Expression::Function(
                stringify!($name).to_string(),
                vec![$point[0].clone()]
            ), $direction[0].clone()))
        }
    };
}
/// (point) => Ok(point[0].clone()) if point has length 1 otherwise Err
macro_rules! get_default_derivative_macro2 {
    ($point:expr, $direction:expr) => {
        if $point.len() != 1 {
            Err(format!(
                "Wrong number of arguments provided for derivative (expected 1, got {}).",
                $point.len()
            ))
        } else {
            Ok(expressions::simplify_mul($point[0].clone(), $direction[0].clone()))
        }
    };
}

/// If `function_name` is among the default functions, returns its derivative at point `point` in direction `direction` (provided it exists; if it simply doesn't exist, returns `Expression::None`).
/// If there is a greater error, e.g. no such default function or wrong number of arguments given, returns the corresponding `Err`.
/// 
/// Acts like a HashMap, but initializing a hashmap for this would be overkill since we do not need to modify it.
/// 
/// N.b.: we return an expression and not e.g. a `FunctionRepr` for the sake of simplicity in the application.
pub fn get_default_derivative(function_name: &str, point: &[Expression], direction: &[Expression]) -> Result<Expression, String> {
    match function_name {
        "exp" => get_default_derivative_macro1!(exp, point, direction),
        "ln" => {
            let p = get_default_derivative_macro2!(point, direction)?;
            Ok(expr_if_else!(
                expr_compare!(p.clone(), Gt, Expression::Number(0.0)),
                expr_div!(Expression::Number(1.0), p),
                Expression::None
            ))
        }
        "log" => {
            // D log(x, b)[s, t] = s \partial_x log(x, b) + t \partial_b log(x, b) = s/(x*ln(y)) - (t*ln(x))/(b*ln(b)²)     for x, b > 0
            if point.len() != 2 || direction.len() != 2 {
                return Err("Both point and direction for derivative of log must have exactly two arguments.".to_string())
            }
            Ok(expr_if_else!(
                expr_and!(expr_compare!(point[0].clone(), Gt, Expression::Number(0.0)), expr_compare!(point[1].clone(), Gt, Expression::Number(0.0))),
                expr_sub!(
                    expr_div!(
                        direction[0].clone(),
                        expr_mul!(
                            point[0].clone(),
                            expr_1arg_func!("ln", point[1].clone())
                        )
                    ),
                    expr_div!(
                        expr_mul!(
                            direction[1].clone(),
                            expr_1arg_func!("ln", point[0].clone())
                        ),
                        expr_mul!(
                            point[1].clone(),
                            expr_square!(expr_1arg_func!("ln", point[1].clone()))
                        )
                    )
                ),
                Expression::None
            ))
        }
        "sqrt" => {
            let p = get_default_derivative_macro2!(point, direction)?;
            Ok(expr_if_else!(
                expr_compare!(p.clone(), Gt, Expression::Number(0.0)),
                expr_div!(
                    Expression::Number(1.0),
                    expr_mul!(Expression::Number(2.0), expr_1arg_func!("sqrt", p))
                ),
                Expression::None
            ))
        }
        "cos" => Ok(expr_neg!(get_default_derivative_macro1!(sin, point, direction)?)),
        "sin" => get_default_derivative_macro1!(cos, point, direction),
        // TODO: add other functions
        _ => Err(format!("No such default function '{function_name}'."))
    }
}