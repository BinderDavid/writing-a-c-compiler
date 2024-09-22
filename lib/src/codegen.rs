use crate::ast;

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

pub fn compile_program(prog: ast::Program) -> Program {
    Program { defs: compile_function_definition(prog.ef) }
}

pub fn compile_function_definition(fundef: ast::FunctionDefinition) -> FunctionDefinition {
    let ast::FunctionDefinition { name, body } = fundef;
    FunctionDefinition { name, instructions: compile_statement(body) }
}

pub fn compile_statement(stmt: ast::Statement) -> Vec<Instruction> {
    match stmt {
        ast::Statement::Return(exp) => {
            let op = compile_exp(exp);
            vec![Instruction::Mov { src: op, dst: Operand::Register }, Instruction::Ret]
        }
    }
}

pub fn compile_exp(exp: ast::Exp) -> Operand {
    match exp {
        ast::Exp::Constant(i) => Operand::Immediate(i),
    }
}
