#[derive(Debug, Clone)]
pub struct Program {
    pub def: FunctionDefinition,
}

#[derive(Debug, Clone)]
pub struct FunctionDefinition {
    pub name: String,
    pub body: Vec<Instruction>,
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Return(Val),
    Unary { op: UnaryOp, src: Val, dst: Var },
}

#[derive(Debug, Clone)]
pub enum Val {
    Constant(i64),
    Var(Var),
}

#[derive(Debug, Clone)]
pub struct Var {
    pub name: String,
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Complement,
    Negate,
}
