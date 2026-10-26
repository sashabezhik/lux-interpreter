mod ast;
mod lexer;

use lexer::Lexer;

fn main() {
    let source = r#"
        let x = 42;
        let name = "World";
        print "Hello, " + name + "!";

        fn greet(person) {
            return "Hi, " + person;
        }

        if x > 10 {
            print "Big number";
        }
    "#;

    let mut lexer = Lexer::new(source.to_string());
    let result = lexer.parse_input_into_tokens();

    match result {
        Ok(tokens) => {
            println!("Tokens:");
            for token_info in tokens {
                println!(
                    "   {:?} at line {}, col {}",
                    token_info.token, token_info.line, token_info.column,
                );
            }
        }
        Err(e) => {
            eprintln!("Lexer error: {}", e);
        }
    }
}
