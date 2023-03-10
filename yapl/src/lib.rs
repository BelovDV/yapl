#![feature(once_cell)]
#![feature(trait_upcasting)]

mod common;
mod glue;

#[allow(unused)]
mod ast;
#[allow(unused)]
mod dfg;
mod parser;

/// `location::Context as File` -> Result<parser::Ast>.
pub use parser::parse;

/// parser::Ast -> ast::Ast.
pub use glue::parser2ast::parser2ast;

pub use common::error::Result;
pub use common::location::File;

pub use ast::Project;
