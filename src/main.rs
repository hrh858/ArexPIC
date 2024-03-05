use core::{parser_v2::ParserV2};


mod core;
fn main() {
    let input = "1 + 1";
    let maybe_parser = ParserV2::new(input);
    match maybe_parser {
        Ok(mut parser) => println!("Parser: {:?}", parser.parse()),
        Err(e) => eprintln!("Error initializing parser: {}", e),
    };

    let input = "3 * 2";
    let maybe_parser = ParserV2::new(input);
    match maybe_parser {
        Ok(mut parser) => println!("Parser: {:?}", parser.parse()),
        Err(e) => eprintln!("Error initializing parser: {}", e),
    };

    let input = "1 + 3 * 2 + 7";
    let maybe_parser = ParserV2::new(input);
    match maybe_parser {
        Ok(mut parser) => println!("Parser: {:?}", parser.parse()),
        Err(e) => eprintln!("Error initializing parser: {}", e),
    };

    let input = "-1 + 3 * 4"; // !!!
    let maybe_parser = ParserV2::new(input);
    match maybe_parser {
        Ok(mut parser) => println!("Parser: {:?}", parser.parse()),
        Err(e) => eprintln!("Error initializing parser: {}", e),
    };

    let input = "(1 + 3) * 4"; // !!!
    let maybe_parser = ParserV2::new(input);
    match maybe_parser {
        Ok(mut parser) => println!("Parser: {:?}", parser.parse()),
        Err(e) => eprintln!("Error initializing parser: {}", e),
    };



    // let input = "(5+10-3*(3-1))^3.2167";
    // let mut t = Tokenizer::new(input);
    // println!("Parsing: {}", input);
    // while let Some(maybe_token) = t.next() {
    //     match maybe_token {
    //         Err(error) => eprintln!("- Error: {}", error),
    //         Ok(token) => println!("- Token: {:?}", token),
    //     }
    // }
    // println!("");
    //
    // let input = "0 + 1 + 1 + 2 + 3 + 5";
    // let mut t = Tokenizer::new(input);
    // println!("Parsing: {}", input);
    // while let Some(maybe_token) = t.next() {
    //     match maybe_token {
    //         Err(error) => eprintln!("- Error: {}", error),
    //         Ok(token) => println!("- Token: {:?}", token),
    //     }
    // }
    // println!("");
    //
    // let input = "1 + (3*2) - 2^10";
    // let mut t = Tokenizer::new(input);
    // println!("Parsing: {}", input);
    // while let Some(maybe_token) = t.next() {
    //     match maybe_token {
    //         Err(error) => eprintln!("- Error: {}", error),
    //         Ok(token) => println!("- Token: {:?}", token),
    //     }
    // }
}
