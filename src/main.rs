mod lexer;
mod parser;

use lexer::Lexer;
use parser::Parser;

fn main() {
    let input = "
        let baz = a: Int -> a

        let complex = {
            let helper = a: Int -> a
            helper(9)
        }
    ";

    // Лексический анализ
    let lexer = Lexer::new(input);

    // Синтаксический анализ
    let mut parser = Parser::new(lexer);
    let ast = parser.parse();

    println!("AST: {:#?}", ast);
}
