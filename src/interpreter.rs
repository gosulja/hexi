use std::cmp::{PartialOrd};
use crate::ast::{Expr, Call, VarDecl, Assignment, BinaryOp, UnaryOp, If, Block};
use crate::stdlib::{REGISTRY_STD};
use std::collections::HashMap;
use crate::lexer::TokenType;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Nil => write!(f, "nil"),
            Value::Bool(b) => write!(f, "{}", b),
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

        i.load_std();
        i
    }

    fn load_std(&mut self) {
        for module in REGISTRY_STD {
            for (name, fptr) in module.funcs {
                let realname = format!("{}_{}", module.name, name);
                self.natives.insert(realname.clone(), *fptr);

                // match (module.name, *name) {
                //     ("io", "print") | ("io", "println") | ("io", "input") => {
                //         self.natives.insert(name.to_string(), *fptr);
                //     }
                //     ("math", _) => {
                //         self.natives.insert(name.to_string(), *fptr);
                //     },
                //     ("string", "len") => {
                //         self.natives.insert(name.to_string(), *fptr);
                //     },
                //     _ => {}
                // }
            }
        }
    }

    pub fn list_natives(&self) {
        println!("Available native functions:");
        let mut names: Vec<_> = self.natives.keys().collect();
        names.sort();

        for name in names {
            println!("  {}", name);
        }
    }

    pub fn evaluate(&mut self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Number(n) => Ok(Value::Number(*n)),
            Expr::String(s) => Ok(Value::String(s.to_string())),
            Expr::Identifier(name) => self.vars.get(name).cloned().ok_or_else(|| format!("undefined variable or reference '{}'", name)),
            Expr::Call(c) => self.exec_call(c),
            Expr::VarDecl(v) => self.exec_var_decl(v),
            Expr::Assignment(a) => self.exec_assignment(a),
            Expr::BinaryOp(b) => self.exec_binary_op(b),
            Expr::UnaryOp(u) => self.exec_unary_op(u),
            Expr::If(i) => self.exec_if(i),
            Expr::Block(b) => self.exec_block(b),
        }
    }

    fn exec_if(&mut self, i: &If) -> Result<Value, String> {
        let cond = self.evaluate(&i.cond)?;
        // if statements should only allow conditions which are truthy
        if cond.is_truthy() {
            // execute the main block, so inside if cond { ... }
            self.exec_block(&i.block)
        } else if let Some(else_block) = &i.else_block {
            // Else
            self.exec_block(else_block)
        } else {
            // return nil
            Ok(Value::Nil)
        }
    }

    fn exec_block(&mut self, b: &Block) -> Result<Value, String> {
        let mut last = Value::Nil;
        // these are basically statements but im too lazy to refactor
        for e in &b.exprs {
            last = self.evaluate(e)?;
        }

        Ok(last)
    }

    fn exec_unary_op(&mut self, u: &UnaryOp) -> Result<Value, String> {
        let operand = self.evaluate(&u.operand)?;
        match u.op {
            TokenType::Sub => match operand {
                Value::Number(n) => Ok(Value::Number(-n)),  // negate numbers
                _ => Err("negate unary operator only supported on numbers".to_string())
            },
            _ => Err(format!("unsupported unary operator {:?}", u.op))
        }
    }

    fn exec_call(&mut self, call: &Call) -> Result<Value, String> {
        let mut args = Vec::new();
        for a in &call.args { args.push(self.evaluate(a)?); }

        let sig = call.signature();   // get the signature of the function (full name of the function)
        if let Some(f) = self.natives.get(&sig) {
            f(&args)
        } else {
            if let Some(f) = self.natives.get(&call.name) {
                f(&args)
            } else {
                Err(format!("undefined function '{}'", call.name))
            }
        }
    }

    fn exec_binary_op(&mut self, b: &BinaryOp) -> Result<Value, String> {
        let left = self.evaluate(&b.left)?;
        let right = self.evaluate(&b.right)?;

        match b.op {
            TokenType::DblEquals => {
                Ok(Value::Bool(left == right))
            },

            TokenType::Lt => {
                Ok(Value::Bool(left < right))
            },

            TokenType::Gt => {
                Ok(Value::Bool(left > right))
            },

            TokenType::Lte => {
                Ok(Value::Bool(left <= right))
            },

            TokenType::Gte => {
                Ok(Value::Bool(left >= right))
            },

            TokenType::Neq => {
                Ok(Value::Bool(left != right))
            },

            TokenType::Add | TokenType::Sub | TokenType::Mul | TokenType::Div | TokenType::Mod => {
                match (left, right) {
                    (Value::Number(l), Value::Number(r)) => {
                        let result = match b.op {
                            TokenType::Add => l + r,
                            TokenType::Sub => l - r,
                            TokenType::Mul => l * r,
                            TokenType::Div => {
                                if r == 0.0 {
                                    return Err("division by zero".to_string());
                                }
                                l / r
                            },
                            TokenType::Mod => {
                                if r == 0.0 {
                                    return Err("modulo by zero".to_string());
                                }
                                l % r
                            },
                            _ => unreachable!(), // done
                        };
                        Ok(Value::Number(result))
                    },
                    _ => Err("arithmetic operations can only be performed on numbers".to_string())
                }
            },

            _ => Err(format!("unsupported binary operator {:?}", b.op))
        }
    }

    fn exec_var_decl(&mut self, var: &VarDecl) -> Result<Value, String> {
        if self.vars.contains_key(&var.name) {
            Err(format!("variable '{}' already defined!", var.name))
        } else {
            let value = self.evaluate(var.value.as_ref())?;
            self.vars.insert(var.name.clone(), value);
            Ok(Value::Nil)
        }
    }

    fn exec_assignment(&mut self, assignment: &Assignment) -> Result<Value, String> {
        if self.vars.contains_key(&assignment.name) {
            // referenced https://doc.rust-lang.org/book/ch08-03-hash-maps.html
            let avalue = self.evaluate(assignment.assignee.as_ref())?;
            self.vars.entry(assignment.name.clone()).and_modify(|v| *v = avalue);
            Ok(Value::Nil)
        } else {
            Err(format!("variable '{}' not defined!", assignment.name))
        }
    }
    // not used/
    // fn exec_mul(&mut self, exprs: &[Expr]) -> Result<Vec<Value>, String> {
    //     let mut results = Vec::new();
    //     for e in exprs { results.push(self.evaluate(e)?); }
    //     Ok(results)
    // }

    pub fn dbg_print_variables(&self) {
        for (name, value) in self.vars.clone().into_iter() {
            println!("{} = {}", name, value);
        }
    }
}

// helper implementations
impl Value {
    pub fn as_string(self) -> Result<String, String> {
        match self {
            Value::String(s) => Ok(s),
            _ => Err(format!("{:?} is not a string", self)),
        }
    }

    // pub fn is_bool(&self) -> bool {
    //     matches!(self, Value::Bool(_))
    // }

    pub fn as_bool_ref(&self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn is_true(&self) -> bool {
        matches!(self, Value::Bool(true))
    }

    pub fn is_false(&self) -> bool {
        matches!(self, Value::Bool(false))
    }

    pub fn equals_bool(&self, other: bool) -> bool {
        match self {
            Value::Bool(b) => *b == other,
            _ => false,
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Bool(false) | Value::Nil => false,
            _ => true,
        }
    }
}