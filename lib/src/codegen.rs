pub struct Program {
    pub defs: FunctionDefinition,
}

pub struct FunctionDefinition {
    pub name: String,
    pub instructions: Vec<Instruction>,
}

pub enum Instruction {
    Ret,
    Mov { src: Operand, dst: Operand },
}

pub enum Operand {
    Immediate(i64),
    Register,
}
