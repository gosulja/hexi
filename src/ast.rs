use crate::lexer::TokenType;

#[derive(Debug, Clone)]
pub enum Expr {
    Identifier(String),
    Number(f64),
    String(String),
    Call(Call),
    VarDecl(VarDecl),
    Assignment(Assignment),
    BinaryOp(BinaryOp),
    UnaryOp(UnaryOp),
    Block(Block),
    If(If),
    Array(Array),
    IndexAccess(IndexAccess),
    MethodCall(MethodCall),
}

#[derive(Debug, Clone)]
pub struct Block {
    pub exprs: Vec<Expr>,
}

#[derive(Debug, Clone)]
pub struct If {
    pub cond: Box<Expr>,
    pub block: Block,
    pub else_block: Option<Block>
}

#[derive(Debug, Clone)]
pub struct Call {
    pub module: Option<String>,     // acesses from a module? io?
    pub name: String,
    pub args: Vec<Expr>,
}

#[derive(Debug, Clone)]
pub struct VarDecl {
    pub name: String,
    pub value: Box<Expr>,   // so we dont recursively set spaces
}

#[derive(Debug, Clone)]
pub struct Assignment {
    pub name: String,
    pub assignee: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct UnaryOp {
    pub operand: Box<Expr>,
    pub op: TokenType, // negate, or other shit idk
}

#[derive(Debug, Clone)]
pub struct BinaryOp {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    pub op: TokenType,
}

#[derive(Debug, Clone)]
pub struct Array {
    pub values: Vec<Expr>,
}

#[derive(Debug, Clone)]
pub struct IndexAccess {
    pub object: Box<Expr>,
    pub index: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct MethodCall {
    pub object: Box<Expr>,
    pub method: String,
    pub args: Vec<Expr>,
}

impl Block {
    pub fn new(exprs: Vec<Expr>) -> Block {
        Block { exprs }
    }
}

impl If {
    pub fn new(cond: Expr, block: Block, else_block: Option<Block>) -> Self {
        If {
            cond: Box::new(cond),
            block,
            else_block,
        }
    }
}

impl UnaryOp {
    pub fn new(operand: Expr, op: TokenType) -> Self {
        UnaryOp { operand: Box::new(operand), op }
    }
}

impl Call {
    pub fn new(name: String, args: Vec<Expr>) -> Self {
        Call { module: None, name, args }
    }

    pub fn new_from_module(module: String, name: String, args: Vec<Expr>) -> Self {
        Call { module: Some(module), name, args }
    }

    // Return the signature name for the function if it's in a module
    pub fn signature(&self) -> String {
        match &self.module {
            Some(m) => format!("{}_{}", m, self.name),
            None => self.name.clone(),
        }
    }
}

impl VarDecl {
    pub fn new(name: String, value: Expr) -> Self {
        VarDecl { name, value: Box::new(value) }
    }
}

impl Assignment {
    pub fn new(name: String, assignee: Expr) -> Self {
        Assignment { name, assignee: Box::new(assignee) }
    }
}

impl BinaryOp {
    pub fn new(left: Expr, right: Expr, op: TokenType) -> Self {
        BinaryOp { left: Box::new(left), right: Box::new(right), op }
    }
}

impl Array {
    pub fn new(values: Vec<Expr>) -> Self {
        Array { values }
    }
}

impl IndexAccess {
    pub fn new(object: Expr, index: Expr) -> Self {
        IndexAccess {
            object: Box::new(object),
            index: Box::new(index),
        }
    }
}

impl MethodCall {
    pub fn new(object: Expr, method: String, args: Vec<Expr>) -> Self {
        MethodCall {
            object: Box::new(object),
            method,
            args,
        }
    }
}