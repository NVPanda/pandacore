#![allow(dead_code)]

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int32,
    Float64,
    Fixed, // Para precisão estilo COBOL
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add, Sub, Mul, Div,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    Identifier(String),
    BinaryExpr {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
}

#[derive(Debug, Clone)]
pub enum Statement {
    LetDecl { 
        name: String, 
        value: Expr, 
        value_type: Type // Tipagem explícita ou inferida
    },
    AuditLog(String, Box<Statement>),
    HolyBlock(Vec<UnsafeStatement>),
}