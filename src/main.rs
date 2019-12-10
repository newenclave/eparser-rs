#![allow(unused)]

mod eparser;
use eparser::scanner::Scanner as StrScanner;
use eparser::parser::Parser as Parser;

use std::any::Any;

struct TestCalls<'f> {
    calls: Vec<&'f dyn Fn(&mut TestCalls)>,
}

impl<'f> TestCalls<'f> {
    fn new() -> TestCalls<'f> {
        TestCalls {
            calls: Vec::new()
        }
    }
    fn add<F: Fn(&mut TestCalls)>(&mut self, val: &'f F) {
        self.calls.push(val);
    }

    fn call(&mut self) {
        let tmp = self.calls.clone();
        for c in tmp.iter() {
            c(self)
        }
    }
}

fn main() {
    
    let mut lex: eparser::lexer::Lexer<String> = eparser::lexer::Lexer::new();
    let mut tc = TestCalls::new();

    tc.add(&|_: &mut TestCalls| { print!("Hello") });
    tc.add(&|_: &mut TestCalls| { print!(", ") });
    tc.add(&|_: &mut TestCalls| { println!("world!") });
    tc.call();

    // let cop = tc.calls.clone();

    // for i in cop.iter() {
    //     i(&mut tc)
    // }

    lex.add("hell", "HELL!".to_string());

    let r = lex.run("hell hell_ 123 k kkk k2 if if2 ident 
    float 
    90.188 
    \"and this \nis a < string value\" 
    test values 224.67 322342.43 0.001");

    match &r {
        Err(expr) => println!("Fail! {}", expr),
        Ok(v) => {
            for i in v.iter() {
                println!("{} @ {:?}", i.to_string(), i.position());
            }
        },
    };

    let mut par: Parser<String, String> = Parser::new(r.unwrap());
}
