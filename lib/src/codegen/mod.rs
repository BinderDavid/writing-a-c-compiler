use core::fmt;

use crate::tacky;

pub struct Program {
    pub defs: FunctionDefinition,
}

#[cfg(target_os = "linux")]
impl fmt::Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n.section .note.GNU-stack, \"\",@progbits\n", self.defs)
    }
}

#[cfg(target_os = "macos")]
impl fmt::Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n", self.defs)
    }
}

pub struct FunctionDefinition {
    pub name: String,
    pub instructions: Vec<Instruction>,
}

#[cfg(target_os = "linux")]
impl fmt::Display for FunctionDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let instructions: Vec<String> =
            self.instructions.iter().map(|i| format!("{}", i)).collect();
        write!(f, "    .globl {}\n{}:\n{}", self.name, self.name, instructions.join("\n"))
    }
}

#[cfg(target_os = "macos")]
impl fmt::Display for FunctionDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let instructions: Vec<String> =
            self.instructions.iter().map(|i| format!("{}", i)).collect();
        write!(f, "    .globl _{}\n_{}:\n{}", self.name, self.name, instructions.join("\n"))
    }
}

pub enum Instruction {
    Ret,
    Mov { src: Operand, dst: Operand },
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Ret => write!(f, "    ret"),
            Instruction::Mov { src, dst } => write!(f, "    movl {}, {}", src, dst),
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

pub fn compile_program(prog: tacky::ast::Program) -> Program {
    Program { defs: compile_function_definition(prog.def) }
}

pub fn compile_function_definition(fundef: tacky::ast::FunctionDefinition) -> FunctionDefinition {
    let tacky::ast::FunctionDefinition { name, body } = fundef;
    FunctionDefinition { name, instructions: compile_instructions(body) }
}

pub fn compile_instructions(_instructions: Vec<tacky::ast::Instruction>) -> Vec<Instruction> {
    todo!()
}
