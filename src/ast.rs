#[derive(Debug, Clone)]
pub enum Expr {
    Identifier(String),
    Number(f64),
    String(String),
    Call(Call),
    VarDecl(VarDecl)
}

#[derive(Debug, Clone)]
pub struct Call {
    pub name: String,
    pub args: Vec<Expr>,
}

#[derive(Debug, Clone)]
pub struct VarDecl {
    pub name: String,
    pub value: Box<Expr>,   // so we dont recursively set spaces
}

impl Call {
    pub fn new(name: String, args: Vec<Expr>) -> Self {
        Call { name, args }
    }
}

impl VarDecl {
    pub fn new(name: String, value: Expr) -> Self {
        VarDecl { name, value: Box::new(value) }
    }
}