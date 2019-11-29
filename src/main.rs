mod eparser;

fn main() {
    let test_string = &String::from("black and orange\n white and blue yellow \ngreen red not color")[0..];

    let mut scan = eparser::scanner::Scanner::new(&test_string);
    let mut t: eparser::trie::Trie<String> = eparser::trie::Trie::new();

    t.set("orange", String::from("ORANGE"));
    t.set("green", String::from("GREEN"));
    t.set("black", String::from("BLACK"));
    t.set("white", String::from("WHITE"));
    t.set("blue", String::from("BLUE"));
    t.set("red", String::from("RED"));

    let mut result = String::new();

    while !scan.eol() {
        let pos = scan.position();
        let bla = t.get(&mut scan);
        match bla {
            Some(expr) => { 
                println!("Found value '{}' at {}:{}", expr, pos.0, pos.1);
                result.push_str(&expr);
            }, 
            None => { 
                result.push(scan.value());
                scan.advance(); 
            },
        }
    }
    println!("ResultString is: '{}'", result)
}
