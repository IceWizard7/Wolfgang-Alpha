use std::collections::HashMap;
use std::f64::consts;
pub use crate::math::{Object, FunctionRepr};

// Wrapped in a function because const hashmaps aren't available yet
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