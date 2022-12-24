
use rustler::{   NifStruct, NifUntaggedEnum};



#[derive(NifStruct)]
#[module = "SqlParser.Document"]
pub struct Document {
    statements: Vec<Statement>
}
impl Document {
    pub fn new(ast: Vec<sqlparser::ast::Statement>) -> Self {
        Self {
            statements: ast.iter().map(|s| match s {
                sqlparser::ast::Statement::Query(query) => {
                    Statement::Query(Query::new(* query.clone()))
                }
                _ => todo!()
            }).collect()
        }
    }
}

#[derive(NifUntaggedEnum)]
pub enum SetExpr {
    Select(Select)
}

#[derive(NifStruct)]
#[module = "SqlParser.Wildcard"]
pub struct Wildcard {
}
impl Wildcard {
    pub fn new() -> Self {
        Self {

        }
    }
}

#[derive(NifUntaggedEnum)]
pub enum SelectItem {
    Wildcard(Wildcard)
}
// #[derive(NifStruct)]
// #[module = "SqlParser.TableWithJoins"]
// pub struct TableWithJoins {
//     pub relation: TableFactor
//     // pub quote_style: Option<char>,
// }

#[derive(NifStruct)]
#[module = "SqlParser.Ident"]
pub struct Ident {
    pub value: String,
    // pub quote_style: Option<char>,
}
// impl Ident {
//     pub fn new(name: sqlparser::ast::Ident) -> Self {
//         Self {
//             value: name.to_string()
//         }
//     }
// }
// #[derive(NifStruct)]
// #[module = "SqlParser.ObjectName"]
// pub struct ObjectName {
//     names: Vec<Ident>
// }
// impl ObjectName {
//     pub fn new(object_name: Vec<sqlparser::ast::ObjectName>) -> Self {
//         Self {
//             names: object_name.iter().map(|p| Ident::new(p.0)).collect()
//         }
//     }
// }
#[derive(NifUntaggedEnum)]
pub enum TableFactor {
    Table(Table)
}
#[derive(NifStruct)]
#[module = "SqlParser.Table"]
pub struct Table {
    // pub projection: Vec<SelectItem>,
    // pub from: Vec<TableWithJoins>,
    // pub with: Option<With>,
    // pub body: SetExpr,
    // pub order_by: Vec<OrderByExpr>,
    // pub limit: Option<Expr>,
    // pub offset: Option<Offset>,
    // pub fetch: Option<Fetch>,
    // pub lock: Option<LockType>,
}
// impl Table {
//     pub fn new(object_name: Vec<sqlparser::ast::ObjectName>) -> Self {
//         Self {
//             names: object_name.iter().map(|p| Ident::new(p.0)).collect()
//         }
//     }
// }
#[derive(NifStruct)]
#[module = "SqlParser.Select"]
pub struct Select {
    pub projection: Vec<SelectItem>,
    // pub from: Vec<TableWithJoins>,
    // pub with: Option<With>,
    // pub body: SetExpr,
    // pub order_by: Vec<OrderByExpr>,
    // pub limit: Option<Expr>,
    // pub offset: Option<Offset>,
    // pub fetch: Option<Fetch>,
    // pub lock: Option<LockType>,
}
impl Select {
    pub fn new(ast: sqlparser::ast::Select) -> Self {
        Self {
            projection: ast.projection.iter().map(|p| match p {
                sqlparser::ast::SelectItem::Wildcard(_wildcard) => {
                    SelectItem::Wildcard(Wildcard::new())
                }
                _ => todo!()
            }).collect(),
            // from: ast.from.iter().map(|p|
            //     match p.relation {
            //         sqlparser::ast::TableFactor::Table{name: name} => {
            //             TableFactor::Table {
            //                 name: name,
            //                 // alias: table.alias,
            //                 // args: table.args,
            //                 // with_hints: table.with_hints,
            //             }
            //         }
            //     }
            // ).collect()
        }
    }
}

#[derive(NifStruct)]
#[module = "SqlParser.Query"]
pub struct Query {
    // pub with: Option<With>,
    pub body: SetExpr,
    // pub order_by: Vec<OrderByExpr>,
    // pub limit: Option<Expr>,
    // pub offset: Option<Offset>,
    // pub fetch: Option<Fetch>,
    // pub lock: Option<LockType>,
}
impl Query {
    pub fn new(ast: sqlparser::ast::Query) -> Self {
        let body = match *ast.body {
            sqlparser::ast::SetExpr::Select(select) => {
                SetExpr::Select(Select::new(*select))
            }
            _ => todo!()
        };
        Self {
            body: body
        }
    }
}

#[derive(NifUntaggedEnum)]
pub enum Statement {
    Query(Query)
    // Fragment(FragmentDefinition),
}
