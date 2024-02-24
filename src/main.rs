use core::tokenizer::Tokenizer;

mod core;

fn main() {
    let input = "(5+10-3*(3-1))^3.2167";
    let mut t = Tokenizer::new(input);
    println!("Parsing: {}", input);
    while let Some(maybe_token) = t.next() {
        match maybe_token {
            Err(error) => eprintln!("- Error: {}", error),
            Ok(token) => println!("- Token: {:?}", token),
        }
    }
    println!("");

    let input = "0 + 1 + 1 + 2 + 3 + 5";
    let mut t = Tokenizer::new(input);
    println!("Parsing: {}", input);
    while let Some(maybe_token) = t.next() {
        match maybe_token {
            Err(error) => eprintln!("- Error: {}", error),
            Ok(token) => println!("- Token: {:?}", token),
        }
    }
    println!("");

    let input = "1 + (3*2) - 2^10";
    let mut t = Tokenizer::new(input);
    println!("Parsing: {}", input);
    while let Some(maybe_token) = t.next() {
        match maybe_token {
            Err(error) => eprintln!("- Error: {}", error),
            Ok(token) => println!("- Token: {:?}", token),
        }
    }
}
