#[derive(Debug, Clone)]
pub struct Program {
    pub ef: FunctionDefinition,
}

#[derive(Debug, Clone)]
pub struct FunctionDefinition {
    pub name: String,
    pub body: Statement,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Return(Exp),
}

#[derive(Debug, Clone)]
pub enum Exp {
    Constant(i64),
    Unary(UnaryOp, Box<Exp>),
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Complement,
    Negate,
}
