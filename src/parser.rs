#![allow(dead_code)]
use crate::ast::{Statement, Expr, UnsafeStatement};
use crate::lexer::Token;

pub struct TokenParser<'a> {
    tokens: &'a [Token],
    pos: usize,
}

impl<'a> TokenParser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, pos: 0 }
    }

    // Função auxiliar para espiar o próximo token sem consumir
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    // Consome e retorna o próximo token
    fn advance(&mut self) -> Option<&Token> {
        let tok = self.tokens.get(self.pos);
        if tok.is_some() { self.pos += 1; }
        tok
    }

    // O "Cérebro" do Parser: Decide qual regra aplicar
    pub fn parse(&mut self) -> Vec<Statement> {
        let mut statements = Vec::new();
        while let Some(token) = self.peek() {
            match token {
                Token::Let => statements.push(self.parse_let()),
                Token::Audit => statements.push(self.parse_audit()),
                Token::Holy => statements.push(self.parse_holy()),
                Token::EOF => break,
                _ => { self.advance(); } // Ignora tokens desconhecidos por enquanto
            }
        }
        statements
    }

    // Regra: let <ident> = <number>
    fn parse_let(&mut self) -> Statement {
        self.advance(); // Consome 'Let'
        
        let name = if let Some(Token::Ident(n)) = self.advance() {
            n.clone()
        } else {
            panic!("Esperado nome da variável após 'let'");
        };

        // Aqui ignoraríamos o '=' (Token::Illegal ou adicionaríamos o Token::Assign no Lexer)
        self.advance(); 

        let value = if let Some(Token::Number(n)) = self.advance() {
            Expr::Number(*n)
        } else {
            panic!("Esperado valor numérico após '='");
        };

        Statement::LetDecl { name, value }
    }

    // Regra: audit <statement>
    fn parse_audit(&mut self) -> Statement {
        self.advance(); // Consome 'Audit'
        let inner = self.parse_let(); // Por simplicidade, audita apenas declarações 'let'
        Statement::AuditLog("Audit Trail Ativo".to_string(), Box::new(inner))
    }

    // Regra: #holy { ... }
    fn parse_holy(&mut self) -> Statement {
        self.advance(); // Consome 'Holy' (#)
        // Aqui implementaríamos a lógica de ler o bloco { } e os comandos de ponteiro
        Statement::HolyBlock(vec![]) 
    }
}