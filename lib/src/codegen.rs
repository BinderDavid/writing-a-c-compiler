use core::fmt;

use crate::ast;

pub struct Program {
    pub defs: FunctionDefinition,
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n.section .note.GNU-stack, \"\",@progbits", self.defs)
    }
}

pub struct FunctionDefinition {
    pub name: String,
    pub instructions: Vec<Instruction>,
}

impl fmt::Display for FunctionDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let instructions: Vec<String> =
            self.instructions.iter().map(|i| format!("{}", i)).collect();
        write!(f, "    .globl {}\n{}:\n{}", self.name, self.name, instructions.join("\n"))
    }
}

pub enum Instruction {
    Ret,
    Mov { src: Operand, dst: Operand },
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Ret => write!(f, "ret"),
            Instruction::Mov { src, dst } => write!(f, "movl {}, {}", src, dst),
        }
    }
}

pub enum Operand {
    Immediate(i64),
    Register,
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::Immediate(i) => write!(f, "${}", i),
            Operand::Register => write!(f, "%eax"),
        }
    }
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
