#![allow(dead_code)]
use std::collections::HashMap;
use crate::ast::Expr;

pub struct SymbolTable {
    // Mapeia o nome da variável para o seu valor/tipo
    variables: HashMap<String, Expr>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self { variables: HashMap::new() }
    }

    pub fn define(&mut self, name: String, value: Expr) {
        self.variables.insert(name, value);
    }

    pub fn resolve(&self, name: &str) -> Option<&Expr> {
        self.variables.get(name)
    }
}