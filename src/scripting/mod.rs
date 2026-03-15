pub mod ast;
pub mod interpreter;
pub mod lexer;
pub mod parser;

use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;

/// Evaluate a source string and return the result (or an error message).
pub fn eval(source: &str) -> Result<interpreter::Value, String> {
    let tokens = Lexer::new(source).tokenize()?;
    let program = Parser::new(tokens).parse()?;
    let mut interp = Interpreter::new();
    interp.exec_program(&program)
}
