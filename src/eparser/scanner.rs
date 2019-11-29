#![allow(unused)]

pub struct Scanner<'a> {
    current: &'a str,
    position: (usize, usize),
}

impl<'a> Scanner<'a> {
    pub fn new (value: &str) -> Scanner {
        Scanner {
            current: value,
            position: (1, 1), 
        }
    }

    pub fn get(&self) -> &str {
        self.current
    }

    pub fn value(&self) -> char {
        if self.current.len() > 0 { 
            let c: char = self.current.chars().nth(0).unwrap();
            return c;
        } 
        return '\0';
    }

    pub fn position(&self) -> (usize, usize) {
        (self.position.0, self.position.1)
    }

    pub fn advance(&mut self) -> bool {
        if self.current.len() > 0 {
            let c: char = self.current.chars().next().unwrap();
            self.current = &self.current[1..];
            self.position = match c {
                '\n' => (self.position.0 + 1, 1),
                  _  => (self.position.0, self.position.1 + 1),
            };
            return true;
        } 
        return false;
    }

    pub fn eol(&self) -> bool {
        return self.current.len() == 0;
    }

    pub fn backup(&self) -> Scanner<'a> {
        return Scanner {
            current: self.current,
            position: self.position
        }
    }

    pub fn restore<'b>(&mut self, other: &'b Scanner<'a> ) {
        self.current = other.current;
        self.position = other.position;
    }
}
