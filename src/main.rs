use std::io;
mod lexer;
use lexer::Lexer;

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
    }
}
