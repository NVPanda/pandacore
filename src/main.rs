mod ast;
mod lexer;
mod parser;
mod translation;
mod backend;
mod interpreter;
mod codegen;
mod context;

use translation::Translator;
use lexer::Lexer;
use parser::TokenParser;
use backend::Backend;
use interpreter::Interpreter;
use codegen::JitBackend;

fn main() {
    let translator = Translator::load("pt-BR");
    let input = "auditar permita panda = 500";

    // 1. Lexing & Parsing
    let mut lexer = Lexer::new(input, &translator);
    let mut tokens = Vec::new();
    loop {
        let tok = lexer.next_token();
        tokens.push(tok.clone());
        if tok == lexer::Token::EOF { break; }
    }

    let mut parser = TokenParser::new(&tokens);
    let ast = parser.parse();

    // 2. Escolha do Motor (Exemplo: via flag ou variável)
    let usar_jit = false; // Mude para true para ativar o poder do Cranelift

    let mut motor: Box<dyn Backend> = if usar_jit {
        Box::new(JitBackend::new())
    } else {
        Box::new(Interpreter::new())
    };

    // 3. Execução Final
    motor.run(ast);
}