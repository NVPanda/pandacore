use crate::ast::Statement;

pub trait Backend {
    // Recebe a lista de comandos e executa um por um
    fn run(&mut self, ast: Vec<Statement>);
}