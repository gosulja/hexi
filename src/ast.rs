#[derive(Debug, Clone)]
pub enum Expr {
    Identifier(String),
    Number(f64),
    String(String),
    Call(Call),
    VarDecl(VarDecl),
    Assignment(Assignment)
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