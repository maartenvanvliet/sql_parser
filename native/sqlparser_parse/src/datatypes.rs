use rustler::types::atom::Atom;
use rustler::{Encoder, Env, Term};
use rustler::{NifStruct, NifTaggedEnum, NifUntaggedEnum};
// use rustler::{map}
mod type_atoms {
    rustler::atoms! {
        value,
        unary_op,
        binary_op,
        nested,
        all_op,
        any_op,
        is_unknown,
        is_not_unknown,
        is_null,
        is_not_null,
        is_true,
        is_not_true,
        is_false,
        is_not_false,
        identifier,
        compound_identifier,
        composite_access

    }
}

mod result_atoms {
    rustler::atoms! {
        not_implemented
    }
}
#[derive(NifStruct)]
#[rustler(encode)]
#[module = "SqlParser.Document"]
pub struct Document {
    statements: Vec<Statement>,
}
impl Document {
    pub fn new(ast: Vec<sqlparser::ast::Statement>) -> Self {
        Self {
            statements: ast
                .iter()
                .map(|s| match s {
                    sqlparser::ast::Statement::Query(query) => {
                        Statement::Query(Query::new(*query.clone()))
                    }
                    _ => Statement::NotImplemented(result_atoms::not_implemented()),
                })
                .collect(),
        }
    }
}

#[derive(NifUntaggedEnum)]
#[rustler(encode)]
pub enum SetExpr {
    Select(Select),
    NotImplemented(Atom)
}

#[derive(NifStruct)]
#[module = "SqlParser.Wildcard"]
pub struct Wildcard {}
impl Wildcard {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(NifStruct)]
#[rustler(encode)]
#[module = "SqlParser.ExprWithAlias"]
pub struct ExprWithAlias {
    expr: Expr,
    alias: Ident,
}

#[derive(NifUntaggedEnum)]
#[rustler(encode)]
pub enum SelectItem {
    Wildcard(Wildcard),
    UnnamedExpr(Expr),
    NotImplemented(Atom),
    ExprWithAlias(ExprWithAlias), // QualifiedWildcard(ObjectName, WildcardAdditionalOptions),
}
#[derive(NifStruct)]
#[rustler(encode)]
#[module = "SqlParser.TableWithJoins"]
pub struct TableWithJoins {
    pub relation: TableFactor, // pub joins: Vec<Join>,
}
impl TableWithJoins {
    pub fn new(ast: &sqlparser::ast::TableWithJoins) -> Self {
        let relation = match &ast.relation {
            sqlparser::ast::TableFactor::Table{name, ..} => TableFactor::Table(Table {
                name: ObjectName {
                    names: name.0.iter().map(|p| Ident::from(p.clone())).collect(),
                }
            }),
            sqlparser::ast::TableFactor::NestedJoin{..} => TableFactor::NotImplemented(result_atoms::not_implemented()),
            sqlparser::ast::TableFactor::Derived{..} => TableFactor::NotImplemented(result_atoms::not_implemented()),
            sqlparser::ast::TableFactor::TableFunction{..} => TableFactor::NotImplemented(result_atoms::not_implemented()),
            sqlparser::ast::TableFactor::UNNEST{..} => TableFactor::NotImplemented(result_atoms::not_implemented()),
            
        };
        Self {
            relation: relation, //TableFactor::Table(ast.relation),
        }
    }
}
#[derive(NifStruct)]
#[module = "SqlParser.Ident"]
pub struct Ident {
    pub value: String,
    pub quote_style: Option<String>,
}

impl From<sqlparser::ast::Ident> for Ident {
    fn from(ident: sqlparser::ast::Ident) -> Self {
        Self {
            value: ident.to_string(),
            quote_style: match ident.quote_style {
                None => None,
                Some(style) => Some(style.to_string()),
            },
        }
    }
}

#[derive(NifStruct)]
#[module = "SqlParser.ObjectName"]
pub struct ObjectName {
    names: Vec<Ident>,
}

#[derive(NifUntaggedEnum)]
pub enum TableFactor {
    Table(Table),
    NotImplemented(Atom)
}
#[derive(NifStruct)]
#[module = "SqlParser.Table"]
pub struct Table {
    name: ObjectName,
}

#[derive(NifTaggedEnum)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    StringConcat,
    Gt,
    Lt,
    GtEq,
    LtEq,
    Spaceship,
    Eq,
    NotEq,
    And,
    Or,
    Xor,
    BitwiseOr,
    BitwiseAnd,
    BitwiseXor,
    PGBitwiseXor,
    PGBitwiseShiftLeft,
    PGBitwiseShiftRight,
    PGRegexMatch,
    PGRegexIMatch,
    PGRegexNotMatch,
    PGRegexNotIMatch,
    NotImplemented,
}

impl From<sqlparser::ast::BinaryOperator> for BinaryOperator {
    fn from(op: sqlparser::ast::BinaryOperator) -> Self {
        match op {
            sqlparser::ast::BinaryOperator::Plus => Self::Plus,
            sqlparser::ast::BinaryOperator::Minus => Self::Minus,
            sqlparser::ast::BinaryOperator::Multiply => Self::Multiply,
            sqlparser::ast::BinaryOperator::Divide => Self::Divide,
            sqlparser::ast::BinaryOperator::Modulo => Self::Modulo,
            sqlparser::ast::BinaryOperator::StringConcat => Self::StringConcat,
            sqlparser::ast::BinaryOperator::Gt => Self::Gt,
            sqlparser::ast::BinaryOperator::Lt => Self::Lt,
            sqlparser::ast::BinaryOperator::GtEq => Self::GtEq,
            sqlparser::ast::BinaryOperator::LtEq => Self::LtEq,
            sqlparser::ast::BinaryOperator::Spaceship => Self::Spaceship,
            sqlparser::ast::BinaryOperator::Eq => Self::Eq,
            sqlparser::ast::BinaryOperator::NotEq => Self::NotEq,
            sqlparser::ast::BinaryOperator::And => Self::And,
            sqlparser::ast::BinaryOperator::Or => Self::Or,
            sqlparser::ast::BinaryOperator::Xor => Self::Xor,
            sqlparser::ast::BinaryOperator::BitwiseOr => Self::BitwiseOr,
            sqlparser::ast::BinaryOperator::BitwiseAnd => Self::BitwiseAnd,
            sqlparser::ast::BinaryOperator::BitwiseXor => Self::BitwiseXor,
            sqlparser::ast::BinaryOperator::PGBitwiseXor => Self::PGBitwiseXor,
            sqlparser::ast::BinaryOperator::PGBitwiseShiftLeft => Self::PGBitwiseShiftLeft,
            sqlparser::ast::BinaryOperator::PGBitwiseShiftRight => Self::PGBitwiseShiftRight,
            sqlparser::ast::BinaryOperator::PGRegexMatch => Self::PGRegexMatch,
            sqlparser::ast::BinaryOperator::PGRegexIMatch => Self::PGRegexIMatch,
            sqlparser::ast::BinaryOperator::PGRegexNotMatch => Self::PGRegexNotMatch,
            sqlparser::ast::BinaryOperator::PGRegexNotIMatch => Self::PGRegexNotIMatch,
            sqlparser::ast::BinaryOperator::PGCustomBinaryOperator(_) => Self::NotImplemented,
        }
    }
}

#[derive(NifTaggedEnum)]
pub enum UnaryOperator {
    Plus,
    Minus,
    Not,
    PGBitwiseNot,       //=> "~",
    PGSquareRoot,       //=> "|/",
    PGCubeRoot,         //=> "||/",
    PGPostfixFactorial, //=> "!",
    PGPrefixFactorial,  //=> "!!",
    PGAbs,              //=> "@",
}
impl From<sqlparser::ast::UnaryOperator> for UnaryOperator {
    fn from(op: sqlparser::ast::UnaryOperator) -> Self {
        match op {
            sqlparser::ast::UnaryOperator::Plus => Self::Plus,
            sqlparser::ast::UnaryOperator::Minus => Self::Minus,
            sqlparser::ast::UnaryOperator::Not => Self::Not,
            sqlparser::ast::UnaryOperator::PGBitwiseNot => Self::PGBitwiseNot,
            sqlparser::ast::UnaryOperator::PGSquareRoot => Self::PGSquareRoot,
            sqlparser::ast::UnaryOperator::PGCubeRoot => Self::PGCubeRoot,
            sqlparser::ast::UnaryOperator::PGPostfixFactorial => Self::PGPostfixFactorial,
            sqlparser::ast::UnaryOperator::PGPrefixFactorial => Self::PGPrefixFactorial,
            sqlparser::ast::UnaryOperator::PGAbs => Self::PGAbs,
        }
    }
}

#[derive(NifStruct)]
#[rustler(encode)]
#[module = "SqlParser.UnaryOp"]
pub struct UnaryOp {
    op: UnaryOperator,
    expr: Box<Expr>,
}
#[derive(NifStruct)]
#[rustler(encode)]
#[module = "SqlParser.CompositeAccess"]
pub struct CompositeAccess {
    expr: Box<Expr>,
    key: Ident,
}

#[derive(NifStruct)]
#[rustler(encode)]
#[module = "SqlParser.BinaryOp"]
pub struct BinaryOp {
    left: Box<Expr>,
    op: BinaryOperator,
    right: Box<Expr>,
}

#[derive(NifTaggedEnum)]
pub enum Value {
    Number(String, bool),
    SingleQuotedString(String),
    // DollarQuotedString(DollarQuotedString),
    EscapedStringLiteral(String),
    NationalStringLiteral(String),
    HexStringLiteral(String),
    DoubleQuotedString(String),
    Boolean(bool),
    Null,
    Placeholder(String),
    UnQuotedString(String),
    NotImplemented,
}
impl From<sqlparser::ast::Value> for Value {
    fn from(value: sqlparser::ast::Value) -> Self {
        match value {
            sqlparser::ast::Value::Number(num, is_float) => Self::Number(num, is_float),
            sqlparser::ast::Value::SingleQuotedString(string) => Self::SingleQuotedString(string),
            // sqlparser::ast::Value::DollarQuotedString(dollar_quoted_string) => Self::DollarQuotedString(dollar_quoted_string),
            sqlparser::ast::Value::EscapedStringLiteral(string) => {
                Self::EscapedStringLiteral(string)
            }
            sqlparser::ast::Value::NationalStringLiteral(string) => {
                Self::NationalStringLiteral(string)
            }
            sqlparser::ast::Value::HexStringLiteral(string) => Self::HexStringLiteral(string),
            sqlparser::ast::Value::DoubleQuotedString(string) => Self::DoubleQuotedString(string),
            sqlparser::ast::Value::Boolean(boolean) => Self::Boolean(boolean),
            sqlparser::ast::Value::Null => Self::Null,
            sqlparser::ast::Value::Placeholder(placeholder) => Self::Placeholder(placeholder),
            sqlparser::ast::Value::UnQuotedString(string) => Self::UnQuotedString(string),
            _ => Self::NotImplemented,
        }
    }
}
#[derive(NifUntaggedEnum)]
#[rustler(encode)]
pub enum ExprEnum {
    Identifier(Ident),
    CompoundIdentifier(Vec<Ident>),
    // jsonaccess
    CompositeAccess(CompositeAccess),
    BinaryOp(BinaryOp),
    UnaryOp(UnaryOp),
    IsFalse(Box<Expr>),
    IsNotFalse(Box<Expr>),
    IsTrue(Box<Expr>),
    IsNotTrue(Box<Expr>),
    IsNull(Box<Expr>),
    IsNotNull(Box<Expr>),
    IsUnknown(Box<Expr>),
    IsNotUnknown(Box<Expr>),
    AnyOp(Box<Expr>),
    AllOp(Box<Expr>),
    Nested(Box<Expr>),
    Value(Value),
    NotImplemented(Atom)
}

#[derive(NifStruct)]
#[rustler(encode)]
#[module = "SqlParser.Expr"]
pub struct Expr {
    r#type: Atom,
    val: ExprEnum,
}

impl Encoder for Box<Expr> {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        let data = &**self;
        data.encode(env)
    }
}

impl Expr {
    pub fn new(ast: sqlparser::ast::Expr) -> Self {
        match ast {
            sqlparser::ast::Expr::Identifier(ident) => Expr {
                r#type: type_atoms::identifier(),
                val: ExprEnum::Identifier(Ident::from(ident)),
            },
            sqlparser::ast::Expr::CompoundIdentifier(idents) => Expr {
                r#type: type_atoms::compound_identifier(),
                val: ExprEnum::CompoundIdentifier(
                    idents.iter().map(|p| Ident::from(p.clone())).collect(),
                ),
            },
            sqlparser::ast::Expr::CompositeAccess { expr, key } => Expr {
                r#type: type_atoms::composite_access(),
                val: ExprEnum::CompositeAccess(CompositeAccess {
                    expr: Box::new(Expr::new(*expr)),
                    key: Ident::from(key),
                }),
            },
            sqlparser::ast::Expr::IsFalse(expr) => Expr {
                r#type: type_atoms::is_false(),
                val: ExprEnum::IsFalse(Box::new(Expr::new(*expr))),
            },
            sqlparser::ast::Expr::IsNotFalse(expr) => Expr {
                r#type: type_atoms::is_not_false(),
                val: ExprEnum::IsNotFalse(Box::new(Expr::new(*expr))),
            },
            sqlparser::ast::Expr::IsTrue(expr) => Expr {
                r#type: type_atoms::is_true(),
                val: ExprEnum::IsTrue(Box::new(Expr::new(*expr))),
            },
            sqlparser::ast::Expr::IsNotTrue(expr) => Expr {
                r#type: type_atoms::is_not_true(),
                val: ExprEnum::IsNotTrue(Box::new(Expr::new(*expr))),
            },
            sqlparser::ast::Expr::IsNull(expr) => Expr {
                r#type: type_atoms::is_null(),
                val: ExprEnum::IsNull(Box::new(Expr::new(*expr))),
            },
            sqlparser::ast::Expr::IsNotNull(expr) => Expr {
                r#type: type_atoms::is_not_null(),
                val: ExprEnum::IsNotNull(Box::new(Expr::new(*expr))),
            },
            sqlparser::ast::Expr::IsUnknown(expr) => Expr {
                r#type: type_atoms::is_unknown(),
                val: ExprEnum::IsUnknown(Box::new(Expr::new(*expr))),
            },
            sqlparser::ast::Expr::IsNotUnknown(expr) => Expr {
                r#type: type_atoms::is_not_unknown(),
                val: ExprEnum::IsNotUnknown(Box::new(Expr::new(*expr))),
            },
            sqlparser::ast::Expr::BinaryOp { left, op, right } => Expr {
                r#type: type_atoms::binary_op(),
                val: ExprEnum::BinaryOp(BinaryOp {
                    left: Box::new(Expr::new(*left)),
                    op: op.into(),
                    right: Box::new(Expr::new(*right)),
                }),
            },
            sqlparser::ast::Expr::AnyOp(expr) => Expr {
                r#type: type_atoms::any_op(),
                val: ExprEnum::AnyOp(Box::new(Expr::new(*expr))),
            },
            sqlparser::ast::Expr::AllOp(expr) => Expr {
                r#type: type_atoms::all_op(),
                val: ExprEnum::AllOp(Box::new(Expr::new(*expr))),
            },
            sqlparser::ast::Expr::Nested(expr) => Expr {
                r#type: type_atoms::nested(),
                val: ExprEnum::Nested(Box::new(Expr::new(*expr))),
            },
            sqlparser::ast::Expr::UnaryOp { op, expr } => Expr {
                r#type: type_atoms::unary_op(),
                val: ExprEnum::UnaryOp(UnaryOp {
                    op: op.into(),
                    expr: Box::new(Expr::new(*expr)),
                }),
            },
            sqlparser::ast::Expr::Value(value) => Expr {
                r#type: type_atoms::value(),
                val: ExprEnum::Value(value.into()),
            },

            _ => Expr{
                r#type: result_atoms::not_implemented(),
                val: ExprEnum::NotImplemented(result_atoms::not_implemented())
            },
        }
    }
}
#[derive(NifStruct)]
#[rustler(encode)]
#[module = "SqlParser.Select"]
pub struct Select {
    pub distinct: bool,
    // pub top: Option<Top>,
    pub projection: Vec<SelectItem>,
    // pub into: Option<SelectInto>,
    pub from: Vec<TableWithJoins>,
    // pub lateral_views: Vec<LateralView>,
    pub selection: Option<Expr>,
    pub group_by: Vec<Expr>,
    // pub cluster_by: Vec<Expr>,
    // pub distribute_by: Vec<Expr>,
    pub sort_by: Vec<Expr>,
    pub having: Option<Expr>,
    // pub qualify: Option<Expr>,
}
impl Select {
    pub fn new(ast: sqlparser::ast::Select) -> Self {
        Self {
            distinct: ast.distinct,
            projection: ast
                .projection
                .iter()
                .map(|p| match p {
                    sqlparser::ast::SelectItem::Wildcard(_wildcard) => {
                        SelectItem::Wildcard(Wildcard::new())
                    }
                    sqlparser::ast::SelectItem::UnnamedExpr(expr) => {
                        SelectItem::UnnamedExpr(Expr::new(expr.clone()))
                    }
                    sqlparser::ast::SelectItem::ExprWithAlias { expr, alias } => {
                        SelectItem::ExprWithAlias(ExprWithAlias {
                            expr: Expr::new(expr.clone()),
                            alias: Ident::from(alias.clone()),
                        })
                    }
                    _ => SelectItem::NotImplemented(result_atoms::not_implemented()),
                })
                .collect(),
            from: ast.from.iter().map(|p| TableWithJoins::new(p)).collect(),
            selection: match ast.selection {
                Some(expr) => Some(Expr::new(expr)),
                None => None,
            },
            group_by: ast
                .group_by
                .iter()
                .map(|expr| Expr::new(expr.clone()))
                .collect(),
            sort_by: ast
                .sort_by
                .iter()
                .map(|expr| Expr::new(expr.clone()))
                .collect(),
            having: match ast.having {
                Some(expr) => Some(Expr::new(expr)),
                None => None,
            },
        }
    }
}

#[derive(NifStruct)]
#[rustler(encode)]
#[module = "SqlParser.Offset"]
pub struct Offset {
    pub value: Expr,
    pub rows: OffsetRows
}

#[derive(NifStruct)]
#[rustler(encode)]
#[module = "SqlParser.OrderByExpr"]
pub struct OrderByExpr {
    pub expr: Expr,
    pub asc: Option<bool>,
    pub nulls_first: Option<bool>,
}

#[derive(NifStruct)]
#[rustler(encode)]
#[module = "SqlParser.Query"]
pub struct Query {
    // pub with: Option<With>,
    pub body: SetExpr,
    pub order_by: Vec<OrderByExpr>,
    pub limit: Option<Expr>,
    pub offset: Option<Offset>,
    // pub fetch: Option<Fetch>,
    // pub lock: Option<LockType>,
}
#[derive(NifTaggedEnum)]
pub enum OffsetRows {
    None,
    Row,
    Rows,
}

impl Query {
    pub fn new(ast: sqlparser::ast::Query) -> Self {
        let body = match *ast.body {
            sqlparser::ast::SetExpr::Select(select) => SetExpr::Select(Select::new(*select)),
            _ => SetExpr::NotImplemented(result_atoms::not_implemented()),
        };
        Self {
            body: body,
            order_by: ast
                .order_by
                .iter()
                .map(|order_by_expr| OrderByExpr {
                    expr: Expr::new(order_by_expr.expr.clone()),
                    asc: order_by_expr.asc,
                    nulls_first: order_by_expr.nulls_first,
                })
                .collect(),
            limit: match ast.limit {
                Some(expr) => Some(Expr::new(expr)),
                None => None,
            },
            offset: match ast.offset {
                Some(offset) => Some(Offset {
                    value: Expr::new(offset.value),
                    rows: match offset.rows {
                        sqlparser::ast::OffsetRows::None => OffsetRows::None,
                        sqlparser::ast::OffsetRows::Row => OffsetRows::Row,
                        sqlparser::ast::OffsetRows::Rows => OffsetRows::Rows,
                    },
                }),
                None => None,
            },
        }
    }
}

#[derive(NifUntaggedEnum)]
#[rustler(encode)]
pub enum Statement {
    Query(Query), // Fragment(FragmentDefinition),
    NotImplemented(Atom)
}
