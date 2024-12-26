pub mod ast;

use crate::tacky;

pub fn compile_program(prog: tacky::ast::Program) -> ast::Program {
    ast::Program { defs: compile_function_definition(prog.def) }
}

pub fn compile_function_definition(
    fundef: tacky::ast::FunctionDefinition,
) -> ast::FunctionDefinition {
    let tacky::ast::FunctionDefinition { name, body } = fundef;
    ast::FunctionDefinition { name, instructions: compile_instructions(body) }
}

pub fn compile_instructions(_instructions: Vec<tacky::ast::Instruction>) -> Vec<ast::Instruction> {
    todo!()
}
