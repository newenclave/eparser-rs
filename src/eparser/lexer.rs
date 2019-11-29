#![allow(unused)]

use super::trie::Trie as Trie;
use super::scanner::Scanner as StrScanner;

pub enum TokenType<T> {
    None, 
    Ident(String),
    Integer(i64),
    Floating(f64),
    String(String),
    Custom(T),
}

pub struct Lexer<T> {
    trie: Trie<TokenType<T>>
} 

impl<T> Lexer<T> {
    pub fn new () -> Lexer<T> {
        Lexer {
            trie: Trie::new()
        }
    }
}

