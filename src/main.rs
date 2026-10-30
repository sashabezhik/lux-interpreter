mod ast;
mod lexer;
mod parser;

use lexer::Lexer;
use parser::Parser;

fn main() {
    let source = r#"
        let x = 5 + 3 * 2;
        
        fn factorial(n) {
            if n <= 1 {
                return 1;
            }
            return n * factorial(n - 1);
        }
        
        print factorial(5);
    "#;

    let mut lexer = Lexer::new(source.to_string());
    let result = lexer.parse_input_into_tokens();

    let tokens = match result {
        Ok(tokens) => tokens,
        Err(e) => {
            eprintln!("Lexer error: {}", e);
            return;
        }
    };

    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    match result {
        Ok(statements) => {
            println!("AST:");
            for stmt in statements {
                println!("{:#?}", stmt);
            }
        }
        Err(e) => {
            eprintln!("Parser error: {}", e);
        }
    }
}
