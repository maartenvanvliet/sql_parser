use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;

use rustler::types::atom;
use rustler::{Atom, Error};
mod datatypes;
pub use datatypes::Document;
#[rustler::nif]
fn run(sql: String) -> Result<(Atom, Document), Error> {
    // let ast = match graphql_parser::parse_query::<String>(doc.as_str()) {
    //     Ok(ast) => Ok((atom::ok(), Document::new(ast))),
    //     Err(e) => Err(Error::Term(Box::new(e.to_string()))),
    // };
    let dialect = GenericDialect {}; 
    let ast = match Parser::parse_sql(&dialect, sql.as_str()) {
        Ok(ast) => Ok((atom::ok(), Document::new(ast))),
        Err(e) => Err(Error::Term(Box::new(e.to_string()))),
    };
    return ast;
    // return Ok((atom::ok(), atom::ok()));
}

rustler::init!("Elixir.SqlParser.Parse", [run]);
