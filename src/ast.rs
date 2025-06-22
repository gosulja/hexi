#[derive(Debug, Clone)]
pub enum Expr {
    Identifier(String),
    Number(f64),
    String(String),
    Call(Call),
}

#[derive(Debug, Clone)]
pub struct Call {
    pub name: String,
    pub args: Vec<Expr>,
}

impl Call {
    pub fn new(name: String, args: Vec<Expr>) -> Self {
        Call { name, args }
    }
}

