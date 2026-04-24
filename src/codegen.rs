use cranelift::prelude::*;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::Module;
use crate::ast::Statement;
use crate::backend::Backend;

pub struct JitBackend {
    builder_context: FunctionBuilderContext,
    ctx: codegen::Context,
    module: JITModule,
}

impl JitBackend {
    pub fn translate_expr(&mut self, builder: &mut FunctionBuilder, expr: &Expr) -> Value {
        match expr {
            Expr::Number(n) => {
                // Traduz um número para uma constante de 64 bits no código nativo
                builder.ins().f64const(*n)
            },
            Expr::BinaryExpr { left, op, right } => {
                let lhs = self.translate_expr(builder, left);
                let rhs = self.translate_expr(builder, right);
                
                match op {
                    // Aqui o Cranelift gera a instrução binária real (ex: addss, addsd)
                    BinaryOp::Add => builder.ins().fadd(lhs, rhs),
                    BinaryOp::Sub => builder.ins().fsub(lhs, rhs),
                    BinaryOp::Mul => builder.ins().fmul(lhs, rhs),
                    BinaryOp::Div => builder.ins().fdiv(lhs, rhs),
                }
            },
            _ => todo!("Implementar identificadores no JIT"),
        }
    }
}

impl Backend for JitBackend {
    fn run(&mut self, ast: Vec<Statement>) {
        println!("[JIT] Compilando AST para código de máquina nativo...");
        // A lógica do Cranelift envolve criar blocos básicos e traduzir 
        // cada Statement para instruções de baixo nível (ex: iadd, store).
        // Focaremos na implementação detalhada do JIT na próxima etapa.
    }
}

