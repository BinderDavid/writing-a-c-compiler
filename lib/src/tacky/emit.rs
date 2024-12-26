use crate::frontend;

pub trait EmitTacky {
    type Target;

    fn emit_tacky(&self, gen: &mut u64) -> Self::Target;
}

fn fresh_var(gen: &mut u64) -> super::ast::Var {
    let var = super::ast::Var { name: format!("tmp{}", gen) };
    *gen += 1;
    var
}

impl EmitTacky for frontend::ast::Program {
    type Target = super::ast::Program;

    fn emit_tacky(&self, gen: &mut u64) -> Self::Target {
        let frontend::ast::Program { def } = self;
        let def = def.emit_tacky(gen);
        super::ast::Program { def }
    }
}

impl EmitTacky for frontend::ast::FunctionDefinition {
    type Target = super::ast::FunctionDefinition;

    fn emit_tacky(&self, gen: &mut u64) -> Self::Target {
        let frontend::ast::FunctionDefinition { name, body } = self;
        let instructions = body.emit_tacky(gen);
        super::ast::FunctionDefinition { name: name.clone(), body: instructions }
    }
}

impl EmitTacky for frontend::ast::Statement {
    type Target = Vec<super::ast::Instruction>;

    fn emit_tacky(&self, gen: &mut u64) -> Self::Target {
        match self {
            frontend::ast::Statement::Return(exp) => {
                let (mut instructions, val) = exp.emit_tacky(gen);
                instructions.push(super::ast::Instruction::Return(val));
                instructions
            }
        }
    }
}

impl EmitTacky for frontend::ast::Exp {
    type Target = (Vec<super::ast::Instruction>, super::ast::Val);

    fn emit_tacky(&self, gen: &mut u64) -> Self::Target {
        match self {
            frontend::ast::Exp::Constant(c) => (Vec::new(), super::ast::Val::Constant(*c)),
            frontend::ast::Exp::Unary(unary_op, exp) => {
                let (mut instructions, val) = exp.emit_tacky(gen);
                let tmp = fresh_var(gen);
                instructions.push(super::ast::Instruction::Unary {
                    op: unary_op.emit_tacky(gen),
                    src: val,
                    dst: tmp.clone(),
                });
                (instructions, super::ast::Val::Var(tmp))
            }
        }
    }
}

impl EmitTacky for frontend::ast::UnaryOp {
    type Target = super::ast::UnaryOp;

    fn emit_tacky(&self, _gen: &mut u64) -> Self::Target {
        match self {
            frontend::ast::UnaryOp::Complement => super::ast::UnaryOp::Complement,
            frontend::ast::UnaryOp::Negate => super::ast::UnaryOp::Negate,
        }
    }
}
