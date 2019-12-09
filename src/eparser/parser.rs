#![allow(unused)]

use std::fmt;
use super::lexer::TokenInfo;
use std::marker::PhantomData;

pub enum ExpressionType<T, U> {
    None,
    Token(TokenInfo<T>),
    Binary(BinaryOperatiorn<T, U>),
    Prefix(PrefixOperatiorn<T, U>),
    Suffix(PrefixOperatiorn<T, U>),
    Custom(U),
}

pub struct BinaryOperatiorn<T, U> {
    left: Box<ExpressionType<T, U>>,
    right: Box<ExpressionType<T, U>>,
    operation: TokenInfo<T>
}

pub struct PrefixOperatiorn<T, U> {
    expr: Box<ExpressionType<T, U>>,
    operation: TokenInfo<T>
}

pub struct SuffixOperatiorn<T, U> {
    expr: Box<ExpressionType<T, U>>,
    operation: TokenInfo<T>
}

pub struct Parser<T, U> {
    tokens: Vec<TokenInfo<T>>, 
    phantom: PhantomData<U>
}

impl<T, U> Parser<T, U> where T: std::clone::Clone, 
                              U: std::clone::Clone {
    pub fn new (toks: Vec<TokenInfo<T>>) -> Parser<T, U> {
        return Parser {
            tokens: toks,
            phantom: PhantomData
        }
    }
    pub fn parse_expression(&mut self) -> ExpressionType<T, U> {
        return ExpressionType::None;
    }
}
