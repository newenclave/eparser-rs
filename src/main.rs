#![allow(unused)]

mod eparser;
use eparser::scanner::Scanner as StrScanner;

fn skip_while(scan: &mut StrScanner, predic: fn(char) -> bool) {
    while !scan.eol() && predic(scan.top()) {
        scan.advance();
    }
}

fn read_while(scan: &mut StrScanner, predic: fn(char) -> bool) -> String {
    let mut result = String::new();
    while !scan.eol() && predic(scan.top()) {
        result.push(scan.top());
        scan.advance();
    }
    return result;
}

fn scan_ident(scan: &mut StrScanner, ending: &str) -> String {
    let mut result = String::new();
    while !scan.eol() && (scan.top().is_digit(10) || scan.top().is_ascii()) {
        result.push(scan.top());
        scan.advance();
    }
    return result;
}

fn is_ident(c: char) -> bool {
    return c.is_ascii_alphabetic() || c == '_';
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
    let top = match data.chars().next() {
        Some(expr) => expr,
        None => '\0',
    };
    return is_ident(top) && is_ident_string_rest(&data[top.len_utf8()..]);
}

fn main() {
    
    let mut lex: eparser::lexer::Lexer<String> = eparser::lexer::Lexer::new();

    let r = lex.run("123 k kkk k2 if if2 ident 
    float 
    90.188 
    \"and this \nis a string value\" 
    test values 224.67 32.43 0.001");

    match r {
        Err(expr) => println!("Fail! {}", expr),
        Ok(v) => {
            for i in v.iter() {
                println!("{} @ {:?}", i.to_string(), i.position());
            }
        },
    };
}
