use crate::ast::{Expr, Call};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Nil,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Nil => write!(f, "nil"),
        }
    }
}

type Native = fn(&[Value]) -> Result<Value, String>;

pub struct Interpreter {
    natives: HashMap<String, Native>,
    vars: HashMap<String, Value>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let mut i = Interpreter {
            natives: HashMap::new(),
            vars: HashMap::new(),
        };

        i.setup();
        i
    }

    fn setup(&mut self) {
        self.natives.insert("print".to_string(), nprint);
    }

    pub fn evaluate(&mut self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Number(n) => Ok(Value::Number(*n)),
            Expr::Identifier(name) => self.vars.get(name).cloned().ok_or_else(|| format!("undefined variable or reference '{}'", name)),
            Expr::Call(c) => self.exec_call(c),
            Expr::String(s) => Ok(Value::String(s.to_string()))
        }
    }

    fn exec_call(&mut self, call: &Call) -> Result<Value, String> {
        let mut args = Vec::new();
        for a in &call.args { args.push(self.evaluate(a)?); }

        // find native
        if let Some(f) = self.natives.get(&call.name) {
            f(&args)
        } else {
            Err(format!("undefined function '{}'", call.name))
        }
    }

    fn exec_mul(&mut self, exprs: &[Expr]) -> Result<Vec<Value>, String> {
        let mut results = Vec::new();
        for e in exprs { results.push(self.evaluate(e)?); }
        Ok(results)
    }
}

fn nprint(args: &[Value]) -> Result<Value, String> {
    for (i, a) in args.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }

        print!("{}", a);
    }

    println!();
    Ok(Value::Nil)
}