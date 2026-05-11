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

pub fn default_functions() -> HashMap<String, FunctionRepr> {
    HashMap::<String, FunctionRepr>::from([
        ("exp".to_string(), FunctionRepr::Direct(Box::new(|args| {
            if args.len() != 1 { Err(format!("Wrong number of arguments provided for function 'exp' (expected 1, got {}).", args.len())) }
            else { match args[0] {
                Object::Float(x) => Ok(Object::Float(x.exp())),
                _ => Err("Wrong type of argument provided for function 'exp' (expected float).".to_string())
            } }
        }))),
        ("ln".to_string(), FunctionRepr::Direct(Box::new(|args| {
            if args.len() != 1 { Err(format!("Wrong number of arguments provided for function 'ln' (expected 1, got {}).", args.len())) }
            else { match args[0] {
                Object::Float(x) => Ok(Object::Float(x.ln())),
                _ => Err("Wrong type of argument provided for function 'ln' (expected float).".to_string())
            } }
        }))),
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
        ("sqrt".to_string(), FunctionRepr::Direct(Box::new(|args| {
            if args.len() != 1 { Err(format!("Wrong number of arguments provided for function 'sqrt' (expected 1, got {}).", args.len())) }
            else { match args[0] {
                Object::Float(x) => Ok(Object::Float(x.sqrt())),
                _ => Err("Wrong type of argument provided for function 'sqrt' (expected float).".to_string())
            } }
        }))),


        // ---------------------------------------------- TRIGONOMETRIC FUNCTIONS ----------------------------------------------
        ("cos".to_string(), FunctionRepr::Direct(Box::new(|args| {
            if args.len() != 1 { Err(format!("Wrong number of arguments provided for function 'cos' (expected 1, got {}).", args.len())) }
            else { match args[0] {
                Object::Float(x) => Ok(Object::Float(x.cos())),
                _ => Err("Wrong type of argument provided for function 'cos' (expected float).".to_string())
            } }
        }))),
        ("cosh".to_string(), FunctionRepr::Direct(Box::new(|args| {
            if args.len() != 1 { Err(format!("Wrong number of arguments provided for function 'cosh' (expected 1, got {}).", args.len())) }
            else { match args[0] {
                Object::Float(x) => Ok(Object::Float(x.cosh())),
                _ => Err("Wrong type of argument provided for function 'cosh' (expected float).".to_string())
            } }
        }))),
        ("sin".to_string(), FunctionRepr::Direct(Box::new(|args| {
            if args.len() != 1 { Err(format!("Wrong number of arguments provided for function 'sin' (expected 1, got {}).", args.len())) }
            else { match args[0] {
                Object::Float(x) => Ok(Object::Float(x.sin())),
                _ => Err("Wrong type of argument provided for function 'sin' (expected float).".to_string())
            } }
        }))),
        ("sinh".to_string(), FunctionRepr::Direct(Box::new(|args| {
            if args.len() != 1 { Err(format!("Wrong number of arguments provided for function 'sinh' (expected 1, got {}).", args.len())) }
            else { match args[0] {
                Object::Float(x) => Ok(Object::Float(x.sinh())),
                _ => Err("Wrong type of argument provided for function 'sinh' (expected float).".to_string())
            } }
        }))),
        ("tan".to_string(), FunctionRepr::Direct(Box::new(|args| {
            if args.len() != 1 { Err(format!("Wrong number of arguments provided for function 'tan' (expected 1, got {}).", args.len())) }
            else { match args[0] {
                Object::Float(x) => Ok(Object::Float(x.tan())),
                _ => Err("Wrong type of argument provided for function 'tan' (expected float).".to_string())
            } }
        }))),
        ("tanh".to_string(), FunctionRepr::Direct(Box::new(|args| {
            if args.len() != 1 { Err(format!("Wrong number of arguments provided for function 'tanh' (expected 1, got {}).", args.len())) }
            else { match args[0] {
                Object::Float(x) => Ok(Object::Float(x.tanh())),
                _ => Err("Wrong type of argument provided for function 'tanh' (expected float).".to_string())
            } }
        }))),
        ("acos".to_string(), FunctionRepr::Direct(Box::new(|args| {
            if args.len() != 1 { Err(format!("Wrong number of arguments provided for function 'acos' (expected 1, got {}).", args.len())) }
            else { match args[0] {
                Object::Float(x) => Ok(Object::Float(x.acos())),
                _ => Err("Wrong type of argument provided for function 'acos' (expected float).".to_string())
            } }
        }))),
        ("acosh".to_string(), FunctionRepr::Direct(Box::new(|args| {
            if args.len() != 1 { Err(format!("Wrong number of arguments provided for function 'acosh' (expected 1, got {}).", args.len())) }
            else { match args[0] {
                Object::Float(x) => Ok(Object::Float(x.acosh())),
                _ => Err("Wrong type of argument provided for function 'acosh' (expected float).".to_string())
            } }
        }))),
        ("asin".to_string(), FunctionRepr::Direct(Box::new(|args| {
            if args.len() != 1 { Err(format!("Wrong number of arguments provided for function 'asin' (expected 1, got {}).", args.len())) }
            else { match args[0] {
                Object::Float(x) => Ok(Object::Float(x.asin())),
                _ => Err("Wrong type of argument provided for function 'asin' (expected float).".to_string())
            } }
        }))),
        ("asinh".to_string(), FunctionRepr::Direct(Box::new(|args| {
            if args.len() != 1 { Err(format!("Wrong number of arguments provided for function 'asinh' (expected 1, got {}).", args.len())) }
            else { match args[0] {
                Object::Float(x) => Ok(Object::Float(x.asinh())),
                _ => Err("Wrong type of argument provided for function 'asinh' (expected float).".to_string())
            } }
        }))),
        ("atan".to_string(), FunctionRepr::Direct(Box::new(|args| {
            if args.len() != 1 { Err(format!("Wrong number of arguments provided for function 'atan' (expected 1, got {}).", args.len())) }
            else { match args[0] {
                Object::Float(x) => Ok(Object::Float(x.atan())),
                _ => Err("Wrong type of argument provided for function 'atan' (expected float).".to_string())
            } }
        }))),
        ("atanh".to_string(), FunctionRepr::Direct(Box::new(|args| {
            if args.len() != 1 { Err(format!("Wrong number of arguments provided for function 'atanh' (expected 1, got {}).", args.len())) }
            else { match args[0] {
                Object::Float(x) => Ok(Object::Float(x.atanh())),
                _ => Err("Wrong type of argument provided for function 'atanh' (expected float).".to_string())
            } }
        }))),
    ])
}