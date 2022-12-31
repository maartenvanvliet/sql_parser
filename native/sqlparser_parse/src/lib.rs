use sqlparser::parser::Parser;

use rustler::types::atom;
use rustler::{Atom, Error, NifTaggedEnum};

mod datatypes;
pub use datatypes::Document;

#[derive(NifTaggedEnum)]
// #[rustler(encode)]
pub enum Dialect {
    Ansi,
    BigQuery,
    ClickHouse,
    Generic,
    Hive,
    Mssql,
    Mysql,
    Postgres,
    Redshift,
    Sqlite,
    Snowflake,
}

#[derive(NifTaggedEnum)]
// #[rustler(encode)]
pub enum RecursionLimit {
    Infinity,
    Limit(usize),
}

#[rustler::nif]
fn parse_statements(
    sql: String,
    dialect: Dialect,
    recursion_limit: RecursionLimit,
) -> Result<(Atom, Document), Error> {
    let sql_dialect: Box<dyn sqlparser::dialect::Dialect> = match dialect {
        Dialect::Ansi => Box::new(sqlparser::dialect::AnsiDialect {}),
        Dialect::BigQuery => Box::new(sqlparser::dialect::BigQueryDialect {}),
        Dialect::ClickHouse => Box::new(sqlparser::dialect::ClickHouseDialect {}),
        Dialect::Generic => Box::new(sqlparser::dialect::GenericDialect {}),
        Dialect::Hive => Box::new(sqlparser::dialect::HiveDialect {}),
        Dialect::Mssql => Box::new(sqlparser::dialect::MsSqlDialect {}),
        Dialect::Mysql => Box::new(sqlparser::dialect::MySqlDialect {}),
        Dialect::Postgres => Box::new(sqlparser::dialect::PostgreSqlDialect {}),
        Dialect::Redshift => Box::new(sqlparser::dialect::RedshiftSqlDialect {}),
        Dialect::Sqlite => Box::new(sqlparser::dialect::SQLiteDialect {}),
        Dialect::Snowflake => Box::new(sqlparser::dialect::SnowflakeDialect {}),
    };

    let mut parser = Parser::new(&*sql_dialect);
    parser = match recursion_limit {
        RecursionLimit::Infinity => parser,
        RecursionLimit::Limit(limit) => parser.with_recursion_limit(limit),
    };

    let ast = match parser.try_with_sql(sql.as_str()) {
        Ok(mut p) => match p.parse_statements() {
            Ok(ast) => Ok((atom::ok(), Document::new(ast))),
            Err(e) => Err(Error::Term(Box::new(e.to_string()))),
        },

        Err(e) => Err(Error::Term(Box::new(e.to_string()))),
    };
    return ast;
}

rustler::init!("Elixir.SqlParser.Parse", [parse_statements]);
