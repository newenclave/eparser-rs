#![allow(unused)]

mod eparser;
use eparser::scanner::Scanner as StrScanner;

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

fn main() {
    
    let mut scan = eparser::scanner::Scanner::new("5.09er rt"); 
    
    let dig = scan_number(&mut scan);

    match dig {
        Number::Integer(d) => println!("int({})", d),
        Number::Floating(f) => println!("float({})", f),
    }
    println!("resr: '{}'", scan.get())
}
