use std::io;

mod lexer;
use lexer::Lexer;
mod parser;
use parser::Parser;

fn main() {
    loop {
        let mut expr = String::new();
        io::stdin()
            .read_line(&mut expr)
            .expect("Failed to read line");
        expr.pop();
        let expr = expr.trim().to_string();
        let lexer = Lexer::new(expr);
        let tokens = lexer.parse();
        println!("{tokens:?}");
        let parser = Parser::new(tokens);
        let ast = parser.parse();
        println!("{ast:?}");
    }
}
