#![allow(dead_code)]
use crate::translation::Translator;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Func, Contract, Fixed, Holy, Audit, Let,
    Ident(String), Number(f64), Illegal, EOF,
}

pub struct Lexer<'a> {
    pub input: Vec<char>,
    pub position: usize,
    pub translator: &'a Translator, // Injeção de dependência
}

impl<'a> Lexer<'a> {
    pub fn new(input: &str, translator: &'a Translator) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
            translator,
        }
    }

    fn lookup_ident(&self, ident: &str) -> Token {
        // Primeiro, traduzimos a palavra usando o dicionário injetado
        let internal_keyword = self.translator.translate(ident);
        
        match internal_keyword.as_str() {
            "func" => Token::Func,
            "contract" => Token::Contract,
            "fixed" => Token::Fixed,
            "audit" => Token::Audit,
            "let" => Token::Let,
            _ => Token::Ident(ident.to_string()),
        }
    }

    pub fn next_token(&mut self) -> Token {
        if self.position >= self.input.len() { return Token::EOF; }
        let ch = self.input[self.position];
        self.position += 1;

        match ch {
            ' ' | '\n' | '\t' | '\r' => self.next_token(),
            '#' => Token::Holy,
            _ if ch.is_alphabetic() => {
                let mut word = ch.to_string();
                while self.position < self.input.len() && self.input[self.position].is_alphanumeric() {
                    word.push(self.input[self.position]);
                    self.position += 1;
                }
                self.lookup_ident(&word)
            }
            _ if ch.is_numeric() => {
                let mut num_str = ch.to_string();
                while self.position < self.input.len() && self.input[self.position].is_numeric() {
                    num_str.push(self.input[self.position]);
                    self.position += 1;
                }
                Token::Number(num_str.parse().unwrap_or(0.0))
            }
            _ => Token::Illegal,
        }
    }
}