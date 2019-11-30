#![allow(unused)]

use std::fmt;
use super::trie::Trie as Trie;
use super::scanner::Scanner as StrScanner;

pub enum Token<T> {
    None, 
    Ident(String),
    Integer(i64),
    Floating(f64),
    String(String),
    Custom(T),
}

pub struct TokenInfo<T> {
    token: Token<T>,
    position: (usize, usize)
}

impl<T> TokenInfo<T> {
    pub fn new(value: Token<T>, pos: (usize, usize)) -> TokenInfo<T> {
        TokenInfo {
            token: value,
            position: pos
        }
    } 

    pub fn position(&self) -> (usize, usize) {
        self.position
    }

    pub fn to_string(&self) -> String {
        match &self.token {
            Token::None => format!("None"),
            Token::Ident(ident) => format!("ident({})", ident),
            Token::Integer(i) => format!("int({})", i),
            Token::Floating(f) => format!("float({})", f),
            Token::String(s) => format!("str({})", s),
            Token::Custom(_) => format!("custom(...)"),
        }
    }
}

struct TokenPack<T> {
    value: T,
    possible_ident: bool 
}

impl<T> TokenPack<T> {
    fn new(val: T, ident: bool) -> TokenPack<T> {
        TokenPack {
            value: val,
            possible_ident: ident,
        }
    }
}

pub struct Lexer<T> {
    trie: Trie<TokenPack<T>>
} 

enum Number {
    Integer(i64),
    Floating(f64),
}

fn scan_number(scan: &mut StrScanner) -> Number {

    let mut d: i64 = 0;
    let mut a: f64 = 0.0;
    let mut e: i32 = 0;
    
    while !scan.eol() && scan.top().is_digit(10) {
        let value = scan.top().to_digit(10).unwrap();
        
        d *= 10;
        d += value as i64;

        a *= 10.0;
        a += value as f64;
        scan.advance();
    }

    let mut found: bool = false;
    let scan_bu = scan.backup(); 

    if scan.top() == '.' {
        scan.advance();
        while !scan.eol() && scan.top().is_digit(10) {
            found = true;
            a *= 10.0;
            a += scan.top().to_digit(10).unwrap() as f64;
            e -= 1;
            scan.advance();
        }
    }

    if scan.top() == 'e' || scan.top() == 'E' {
        let mut sign: i32 = 1;
        let mut i: i32 = 0;
        scan.advance();
        match scan.top() {
            '+' => { scan.advance(); },
            '-' => { scan.advance(); sign = -1 },
             _  => { }
        }
        while !scan.eol() && scan.top().is_digit(10) {
            found = true;
            i *= 10;
            i += scan.top().to_digit(10).unwrap() as i32;
            scan.advance();
        }
        e += i * sign;
    }

    while e > 0 {
        a *= 10.0;
        e -= 1;
    }

    while e < 0 {
        a *= 0.1;
        e += 1;
    }

    return if found {
        Number::Floating(a)
    } else {
        scan.restore(&scan_bu);
        Number::Integer(d)
    }
}

fn str_head_tail(data: &str) -> (char, &str) {
    match data.chars().next() {
        Some(c) => (c, &data[c.len_utf8()..]),
        None => ('\0', data),
    }
}

fn is_ident(c: char) -> bool {
    return c.is_ascii_alphabetic() || c == '_';
}

fn scan_ident(scanner: &mut StrScanner) -> String {
    let base = scanner.backup();
    let shift = scanner.advance_while(|c| { is_ident(c) || c.is_digit(10) });
    return String::from(&base.get()[0..shift])
}

fn scan_string(scan: &mut StrScanner, ending: &str) -> String {

    let mut result = String::new();
    let ec = ending.chars().next().unwrap_or('\0');

    while !scan.eol() {
        if scan.get().starts_with(ending) {
            scan.jump(ending.len());
            break;
        }
        match scan.top() {
            '\\' => {
                scan.advance();
                match scan.top() {
                    'n' => result.push('\n'),
                    'r' => result.push('\r'),
                    't' => result.push('\t'),
                    '\\' => result.push('\\'),
                    '\0' => result.push('\\'),
                    ec => result.push(ec),
                    val => { result.push('\\'); result.push(val) },
                }
            },
            val => result.push(val),
        }
        scan.advance();
    }

    return result;
}

fn is_ident_string_rest(data: &str) -> bool {
    for c in data.chars() {
        if !(c.is_digit(10) || is_ident(c)) {
            return false;
        } 
    } 
    return true;
}

fn is_ident_string(data: &str) -> bool {
    let (head, tail) = str_head_tail(data);
    return is_ident(head) && is_ident_string_rest(tail);
}

fn skip_spaces(scanner: &mut StrScanner) {
    scanner.advance_while(|c| { c.is_whitespace() });
}

impl<T> Lexer<T> where T: std::clone::Clone {
    pub fn new() -> Lexer<T> {
        Lexer {
            trie: Trie::new()
        }
    }

    pub fn add(&mut self, key: &str, value: T) {
        let ident = is_ident_string(key);
        self.trie.set(key, TokenPack::new(value, ident));
    }

    pub fn run(&self, data: &str) -> Result<Vec<TokenInfo<T>>, String> {
        let mut result = Vec::new();
        let mut scanner = StrScanner::new(data);
        
        while !scanner.eol() {
            skip_spaces(&mut scanner);
            let backup = scanner.backup();
            let pos = scanner.position();
            let next = self.trie.get(&mut scanner);
    
            match next {
                Some(expr) => {
                    let top = scanner.top();
                    if expr.0.possible_ident && (is_ident(top) || top.is_digit(10)) {
                        let mut ival = String::from(&backup.get()[0..expr.1]);
                        ival.push_str(&scan_ident(&mut scanner));
                        result.push(TokenInfo::new(Token::Ident(ival), pos));
                    } else {
                        result.push(TokenInfo::new(Token::Custom(expr.0.value.clone()), pos));
                    }
                },
                None => {
                    if scanner.top().is_digit(10) {
                        let num = scan_number(&mut scanner);
                        match num {
                            Number::Integer(i) => result.push(TokenInfo::new(Token::Integer(i), pos)),
                            Number::Floating(f) => result.push(TokenInfo::new(Token::Floating(f), pos)),
                        }
                    } else if is_ident(scanner.top()) {
                        let ident = scan_ident(&mut scanner);
                        result.push(TokenInfo::new(Token::Ident(ident), pos));
                    } else if scanner.top() == '"' {
                        scanner.advance();
                        let svalue = scan_string(&mut scanner, "\"");
                        result.push(TokenInfo::new(Token::String(svalue), pos));
                    } else if !scanner.eol() {
                        return Err(format!("Unexpected character '{}' at {}:{}", 
                            scanner.top(), pos.0, pos.1));
                    }
                },
            }
        }
        return Ok(result);
    }
}

