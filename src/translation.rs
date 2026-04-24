#![allow(dead_code)]
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct LanguageConfig {
    pub keywords: HashMap<String, String>,
}

pub struct Translator {
    mapping: HashMap<String, String>,
}

impl Translator {
    pub fn load(lang_code: &str) -> Self {
        let path = format!("locales/{}.json", lang_code);
        let content = fs::read_to_string(path).expect("Erro ao carregar arquivo de idioma");
        let config: LanguageConfig = serde_json::from_str(&content).expect("Erro ao parsear JSON");
        
        Self { mapping: config.keywords }
    }

    // Traduz a palavra do código (ex: "permita") para a keyword interna (ex: "let")
    pub fn translate(&self, word: &str) -> String {
        self.mapping.get(word).cloned().unwrap_or_else(|| word.to_string())
    }
}