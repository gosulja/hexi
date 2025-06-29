use crate::ast::{Array, Assignment, BinaryOp, Block, Call, Expr, FieldAccess, If, IndexAccess, MethodCall, Collection, UnaryOp, VarDecl, CEntry};
use crate::stdlib::{REGISTRY_OPTIONAL, REGISTRY_STD};
use std::collections::{HashMap, HashSet};
use crate::lexer::TokenType;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Collection(CValue),
    Nil,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CValue {
    pub entries: HashMap<CKey, Value>,
    pub size: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CKey {
    Index(usize),
    String(String),
    Number(String),
}

pub trait Method {
    fn call_method(&mut self, method: &str, args: &[Value]) -> Result<Value, String>;
    fn got_method(&self, method: &str) -> bool;
}

impl Method for Value {
    fn call_method(&mut self, method: &str, args: &[Value]) -> Result<Value, String> {
        match self {
            Value::Collection(c) => {
                match method {
                    "push" => {
                        if args.len() != 1 {
                            return Err(format!("push method on array expects 1 argument, got {}", args.len()));
                        }

                        c.push(args[0].clone());
                        Ok(Value::Nil)
                    },

                    "pop" => {
                        if args.len() != 0 {
                            return Err(format!("pop method on array expects no argument, got {}", args.len()));
                        }

                        Ok(c.pop().unwrap_or(Value::Nil))
                    },

                    "size" => {
                        if args.len() != 0 {
                            return Err(format!("size method on array expects no argument, got {}", args.len()));
                        }

                        Ok(Value::Number(c.len() as f64))
                    },

                    "get" => {
                        if args.len() != 1 {
                            return Err(format!("get method expects 1 argument, got {}", args.len()));
                        }

                        let key = match &args[0] {
                            Value::Number(n) => CKey::Index(*n as usize),
                            Value::String(s) => CKey::String(s.clone()),
                            _ => return Err("collection key must be a number or string".to_string()),
                        };

                        Ok(c.get(&key).cloned().unwrap_or(Value::Nil))
                    },

                    "insert" => {
                        if args.len() != 2 {
                            return Err(format!("insert method expects 2 arguments, got {}", args.len()));
                        }

                        let key = match &args[0] {
                            Value::Number(n) => {
                                let idx = *n as usize;
                                if c.is_array_like() && idx > c.size {
                                    return Err(format!("index {} is out of bounds", idx));
                                }
                                CKey::Index(idx)
                            },

                            Value::String(s) => CKey::String(s.clone()),
                            _ => return Err("insert key must be a number or string".to_string()),
                        };

                        c.insert(key, args[1].clone());
                        Ok(Value::Nil)
                    },

                    _ => Err(format!("unknown method '{}' for array.", method))
                }
            },

            Value::String(s) => {
                match method {
                    "len" => {
                        if args.len() != 0 {
                            return Err(format!("len method on string expects no arguments, got {}", args.len()));
                        }

                        Ok(Value::Number(s.len() as f64))
                    },

                    _ => Err(format!("unknown method '{}' for string.", method))
                }
            },

            _ => Err(format!("cannot call method '{}' on {:?}", method, self.type_name())),
        }
    }

    fn got_method(&self, method: &str) -> bool {
        match self {
            Value::Collection(_) => matches!(method, "push" | "pop" | "size" | "get" | "insert"),
            Value::String(_) => matches!(method, "len"),
            _ => false
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Nil => write!(f, "nil"),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Collection(c) => {
                if c.is_array_like() {
                    write!(f, "[")?;
                    let mut first = true;
                    for i in 0..c.size {
                        if !first { write!(f, ", ")?; }
                        if let Some(val) = c.get_by_index(i) {
                            write!(f, "{}", val)?;
                        } else {
                            write!(f, "nil")?;
                        }
                        first = false;
                    }
                    write!(f, "]")
                } else {
                    write!(f, "[")?;
                    let mut first = true;
                    for (key, value) in &c.entries {
                        if !first { write!(f, ", ")?; }
                        match key {
                            CKey::String(s) => write!(f, "{} = {}", s, value)?,
                            CKey::Number(n) => write!(f, "{} = {}", n, value)?,
                            CKey::Index(i) => write!(f, "{} = {}", i, value)?,
                        }
                        first = false;
                    }
                    write!(f, "]")
                }
            }
        }
    }
}

type Native = fn(&[Value]) -> Result<Value, String>;

pub struct Interpreter {
    natives: HashMap<String, Native>,
    vars: HashMap<String, Value>,
    loaded_modules: HashSet<String>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let mut i = Interpreter {
            natives: HashMap::new(),
            vars: HashMap::new(),
            loaded_modules: HashSet::new(),
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

    fn load_module(&mut self, mod_name: &str) -> Result<Value, String> {
        if self.loaded_modules.contains(mod_name) {
            return Ok(Value::Nil);
        }

        if let Some(module) = REGISTRY_OPTIONAL.iter().find(|m| m.name == mod_name) {
            for (name, fptr) in module.funcs {
                let realname = format!("{}_{}", module.name, name);
                self.natives.insert(realname.clone(), *fptr);
            }

            self.loaded_modules.insert(mod_name.to_string());
            Ok(Value::Nil)
        } else {
            Err(format!("module '{}' not found", mod_name))
        }
    }

    pub fn evaluate(&mut self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Number(n) => Ok(Value::Number(*n)),
            Expr::String(s) => Ok(Value::String(s.to_string())),
            Expr::Identifier(name) => self.vars.get(name).cloned().ok_or_else(|| format!("undefined variable or reference '{}'", name)),
            Expr::Call(c) => self.exec_call(c),
            Expr::Collection(c) => self.exec_collection(c),
            Expr::IndexAccess(ia) => self.exec_idx_access(ia),
            Expr::MethodCall(mc) => self.exec_method_call(mc),
            Expr::VarDecl(v) => self.exec_var_decl(v),
            Expr::Assignment(a) => self.exec_assignment(a),
            Expr::BinaryOp(b) => self.exec_binary_op(b),
            Expr::UnaryOp(u) => self.exec_unary_op(u),
            Expr::If(i) => self.exec_if(i),
            Expr::Block(b) => self.exec_block(b),
            Expr::Include(i) => self.load_module(&i.module),
            Expr::FieldAccess(fa) => self.exec_fa(fa),
        }
    }

    fn exec_collection(&mut self, co: &Collection) -> Result<Value, String> {
        let mut c = CValue::new();
        let mut idx = 0;

        for e in &co.entries {
            match e {
                CEntry::Indexed(ex) => {
                    let v = self.evaluate(ex)?;
                    c.insert(CKey::Index(idx), v);
                    idx += 1;
                },

                CEntry::Keyed(k, ex) => {
                    let v = self.evaluate(ex)?;
                    c.insert(CKey::String(k.clone()), v);
                },

                CEntry::NumKeyed(n, ex) => {
                    let v = self.evaluate(ex)?;
                    c.insert(CKey::Number(n.to_string()), v);
                },
            }
        }

        if idx > 0 { c.size = idx; }

        Ok(Value::Collection(c))
    }

    fn exec_fa(&mut self, fa: &FieldAccess) -> Result<Value, String> {
        let ovalue = self.evaluate(&fa.object)?;
        match ovalue {
            Value::Collection(c) => {
                c.get_by_string(&fa.field).cloned().ok_or_else(|| format!("undefined field '{}'", fa.field))
            },
            _ => Err(format!("cannot access field '{}' on non object", fa.field))
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

    fn exec_array(&mut self, a: &Array) -> Result<Value, String> {
        let mut values = Vec::new();
        for v in &a.values { values.push(self.evaluate(v)?); }
        Ok(Value::Collection(CValue::from_array(values)))
    }

    fn exec_idx_access(&mut self, ia: &IndexAccess) -> Result<Value, String> {
        // let arr = self.evaluate(&ia.object)?;
        // let idx = self.evaluate(&ia.index)?;
        // match (arr, idx) {
        //     (Value::Array(a), Value::Number(n)) => {
        //         let i = n as usize;
        //         if i < a.len() {
        //             Ok(a[i].clone())
        //         } else {
        //             Err(format!("array index {} is out of bounds.", i))
        //         }
        //     }
        //     (arr, _) => Err(format!("cannot index into {:?}",arr))
        // }

        // changed to adapt to collection changes
        let col = self.evaluate(&ia.object)?;
        let idx = self.evaluate(&ia.index)?;

        match col {
            Value::Collection(c) => {
                let key = match idx {
                    Value::Number(n) => CKey::Index(n as usize),
                    Value::String(s) => CKey::String(s),
                    _ => return Err("collection index must be a number or string".to_string()),
                };

                Ok(c.get(&key).cloned().unwrap_or(Value::Nil))
            }
            _ => Err(format!("cannot index into {}", col.type_name()))
        }
    }

    fn exec_method_call(&mut self, mc: &MethodCall) -> Result<Value, String> {
        // let obj = self.evaluate(&mc.object);
        // let mut args = Vec::new();
        // for a in &mc.args { args.push(self.evaluate(a)?); }
        //
        // match obj {
        //     Ok(Value::Array(mut arr)) => {
        //         match mc.method.as_str() {
        //             "push" => {
        //                 if args.len() != 1 {
        //                     return Err(format!("push() expects 1 argument, got {}", args.len()))
        //                 }
        //
        //                 arr.push(args[0].clone());
        //
        //                 // method call on identifier? update variable
        //                 // val nums = [1, 2, 3]
        //                 // nums.push(4)
        //                 if let Expr::Identifier(n) = &*mc.object {
        //                     self.vars.insert(n.clone(), Value::Array(arr));
        //                 }
        //
        //                 Ok(Value::Nil)
        //             },
        //             _ => Err(format!("unknown method '{}' for array", mc.method)),
        //         }
        //     },
        //     _ => Err(format!("cannot call method '{}' on {:?}", mc.method, obj))
        // }

        let mut args = Vec::new();
        for a in &mc.args { args.push(self.evaluate(a)?); }

        // calling method on an identifier?
        // some_arr.size()
        if let Expr::Identifier(id) = &*mc.object {
            if let Some(mut val) = self.vars.get(id).cloned() {
                let meth_result = val.call_method(&mc.method, &args)?;
                self.vars.insert(id.clone(), val);  // we wanna update incase the method mutates the obj
                return Ok(meth_result);
            } else {
                return Err(format!("undefined variable '{}'", id));
            }
        }

        // and then handle method calls on exprs
        // val v = [ 1, 2, 3, 4 ].size()
        let mut o = self.evaluate(&mc.object)?;
        o.call_method(&mc.method, &args)
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

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a.partial_cmp(b),
            (Value::String(a), Value::String(b)) => a.partial_cmp(b),
            (Value::Bool(a), Value::Bool(b)) => a.partial_cmp(b),
            _ => None,
        }
    }
}

// helper implementations
impl Value {
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Number(_) => "number",
            Value::String(_) => "string",
            Value::Bool(_) => "bool",
            Value::Collection(_) => "collection",
            Value::Nil => "nil",
        }
    }

    pub fn new_collection() -> Value {
        Value::Collection(CValue::new())
    }

    pub fn from_pairs(pairs: Vec<(String, Value)>) -> Value {
        let mut obj = HashMap::new();
        for (key, value) in pairs {
            obj.insert(key, value);
        }

        Value::Collection(CValue::from_object(obj))
    }

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
            Value::Collection(c) if c.entries.is_empty() => false,
            _ => true,
        }
    }
}

impl CValue {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
            size: 0
        }
    }

    pub fn from_array(values: Vec<Value>) -> Self {
        let mut entries = HashMap::new();
        for (i, val) in values.iter().enumerate() {
            entries.insert(CKey::Index(i), val.clone());
        }

        Self {
            entries,
            size: values.len()
        }
    }

    pub fn from_object(obj: HashMap<String, Value>) -> Self {
        let mut entries = HashMap::new();
        for (k, val) in obj {
            entries.insert(CKey::String(k), val.clone());
        }

        Self {
            entries,
            size: 0
        }
    }

    pub fn get(&self, key: &CKey) -> Option<&Value> {
        self.entries.get(key)
    }

    pub fn insert(&mut self, key: CKey, value: Value) {
        if let CKey::Index(i) = &key {
            if *i >= self.size {
                self.size = *i + 1;
            }
        }

        self.entries.insert(key, value);
    }

    pub fn push(&mut self, value: Value) {
        self.entries.insert(CKey::Index(self.size), value);
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<Value> {
        if self.size == 0 {
            return None;
        }

        self.size -= 1;
        self.entries.remove(&CKey::Index(self.size))
    }

    pub fn len(&self) -> usize {
        if self.is_array_like() {
            self.size
        } else {
            self.entries.len()
        }
    }

    pub fn is_array_like(&self) -> bool {
        self.size > 0 || self.entries.keys().all(|k| matches!(k, CKey::Index(_)))
    }

    pub fn get_by_index(&self, index: usize) -> Option<&Value> {
        self.entries.get(&CKey::Index(index))
    }

    pub fn get_by_string(&self, key: &str) -> Option<&Value> {
        self.entries.get(&CKey::String(key.to_string()))
    }
}