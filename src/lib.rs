// expression  →  equality ;
// equality    → comparison ( ( "!=" | "==" ) comparison )* ;
// comparison  → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
// term        →  factor ( ( "-" | "+" ) factor )* ;
// factor      →  unary ( ( "/" | "*" ) unary )* ;
// unary       →  ( "!" | "-" ) unary | primary ;
// primary     →  NUMBER | STRING | "true" | "false" | "(" expression ")" ;

mod combinators;
mod error;
mod interpreter;
mod node;
mod parser;
mod value;

pub use combinators::{ParseResult, Parser};
pub use interpreter::{eval, Environment};
pub use node::{Node, Operator};
pub use parser::program;
pub use value::Value;
