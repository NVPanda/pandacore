impl Interpreter {
    // Resolve expressões matemáticas recursivamente
    fn eval_expr(&self, expr: &Expr) -> f64 {
        match expr {
            Expr::Number(n) => *n,
            Expr::Identifier(name) => {
                if let Some(Expr::Number(n)) = self.context.resolve(name) {
                    *n
                } else {
                    panic!("Variável '{}' não encontrada ou não é um número", name);
                }
            },
            Expr::BinaryExpr { left, op, right } => {
                let l = self.eval_expr(left);
                let r = self.eval_expr(right);
                match op {
                    BinaryOp::Add => l + r,
                    BinaryOp::Sub => l - r,
                    BinaryOp::Mul => l * r,
                    BinaryOp::Div => l / r,
                }
            }
        }
    }
}