pub mod ast;
pub mod codegen;
pub mod error;
pub mod lexer;
pub mod parser;
pub mod tokens;

pub type Res<T> = Result<T, error::RscError>;
