use crate::frontend;

pub trait EmitTacky {
    type Target;

    fn emit_tacky(&self, gen: &mut u64) -> Self::Target;
}

fn fresh_var(gen: &mut u64) -> super::Var {
    let var = super::Var { name: format!("tmp{}", gen) };
    *gen += 1;
    var
}

impl EmitTacky for frontend::ast::Program {
    type Target = super::Program;

    fn emit_tacky(&self, gen: &mut u64) -> Self::Target {
        let frontend::ast::Program { def } = self;
        let def = def.emit_tacky(gen);
        super::Program { def }
    }
}

impl EmitTacky for frontend::ast::FunctionDefinition {
    type Target = super::FunctionDefinition;

    fn emit_tacky(&self, gen: &mut u64) -> Self::Target {
        let frontend::ast::FunctionDefinition { name, body } = self;
        let instructions = body.emit_tacky(gen);
        super::FunctionDefinition { name: name.clone(), body: instructions }
    }
}

impl EmitTacky for frontend::ast::Statement {
    type Target = Vec<super::Instruction>;

    fn emit_tacky(&self, gen: &mut u64) -> Self::Target {
        match self {
            frontend::ast::Statement::Return(exp) => {
                let (mut instructions, val) = exp.emit_tacky(gen);
                instructions.push(super::Instruction::Return(val));
                instructions
            }
        }
    }
}

impl EmitTacky for frontend::ast::Exp {
    type Target = (Vec<super::Instruction>, super::Val);

    fn emit_tacky(&self, gen: &mut u64) -> Self::Target {
        match self {
            frontend::ast::Exp::Constant(c) => (Vec::new(), super::Val::Constant(*c)),
            frontend::ast::Exp::Unary(unary_op, exp) => {
                let (mut instructions, val) = exp.emit_tacky(gen);
                let tmp = fresh_var(gen);
                instructions.push(super::Instruction::Unary {
                    op: unary_op.emit_tacky(gen),
                    src: val,
                    dst: tmp.clone(),
                });
                (instructions, super::Val::Var(tmp))
            }
        }
    }
}

impl EmitTacky for frontend::ast::UnaryOp {
    type Target = super::UnaryOp;

    fn emit_tacky(&self, _gen: &mut u64) -> Self::Target {
        match self {
            frontend::ast::UnaryOp::Complement => super::UnaryOp::Complement,
            frontend::ast::UnaryOp::Negate => super::UnaryOp::Negate,
        }
    }
}
