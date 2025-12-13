pub mod token;
pub mod lexer;
pub mod ast;
pub mod parser;
pub mod types;
pub mod type_checker;
pub mod runtime;
pub mod interpreter;
pub mod error;

pub use error::AxityError;

pub fn run_source(source: &str) -> Result<String, AxityError> {
    let tokens = lexer::lex(source)?;
    let ast = parser::parse(&tokens)?;
    type_checker::check(&ast)?;
    let mut rt = runtime::Runtime::new();
    let mut out = String::new();
    interpreter::execute(&ast, &mut rt, &mut out)?;
    Ok(out)
}

