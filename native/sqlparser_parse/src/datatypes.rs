use rustler::types::atom::Atom;
use rustler::{Decoder, Encoder, Env, Term};
use rustler::{NifResult, NifStruct, NifTaggedEnum, NifUntaggedEnum};
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
        in_list,
        in_subquery,
        in_unnest,
        between,
        like,
        ilike,
        similar_to,
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
// #[rustler(encode)]
#[module = "SqlParser.Document"]
pub struct Document {
    pub statements: Vec<Statement>,
}
// sqlparser::ast::Query
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
impl From<Value> for sqlparser::ast::Value {
    fn from(value: Value) -> Self {
        // sqlparser::ast::Value {
        //     value: ident.value,
        //     quote_style: None,
        // }
        //Value::Number(number)
        match value {
            Value::Number(number) => sqlparser::ast::Value::Number(number.value, number.long),
            Value::Boolean(boolean) => sqlparser::ast::Value::Boolean(boolean.value),
            _ => sqlparser::ast::Value::Number("3".to_string(), false),
        }
    }
}
impl From<BinaryOperator> for sqlparser::ast::BinaryOperator {
    fn from(binary_operator: BinaryOperator) -> Self {
        match binary_operator {
            BinaryOperator::And => Self::And,
            BinaryOperator::BitwiseAnd => Self::BitwiseAnd,
            BinaryOperator::GtEq => Self::GtEq,
            BinaryOperator::Gt => Self::Gt,
            BinaryOperator::LtEq => Self::LtEq,
            BinaryOperator::Lt => Self::Lt,
            BinaryOperator::Plus => Self::Plus,
            BinaryOperator::Minus => Self::Minus,
            BinaryOperator::Multiply => Self::Multiply,
            BinaryOperator::Divide => Self::Divide,
            BinaryOperator::Modulo => Self::Modulo,
            BinaryOperator::StringConcat => Self::StringConcat,
            BinaryOperator::Eq => Self::Eq,
            _ => Self::Eq,
        }
    }
}
impl From<UnaryOperator> for sqlparser::ast::UnaryOperator {
    fn from(unary_operator: UnaryOperator) -> Self {
        match unary_operator {
            UnaryOperator::Plus => Self::Plus,
            UnaryOperator::Minus => Self::Minus,
            UnaryOperator::Not => Self::Not,
            UnaryOperator::PGBitwiseNot => Self::PGBitwiseNot,
            UnaryOperator::PGSquareRoot => Self::PGSquareRoot,
            UnaryOperator::PGCubeRoot => Self::PGCubeRoot,
            UnaryOperator::PGPostfixFactorial => Self::PGPostfixFactorial,
            UnaryOperator::PGPrefixFactorial => Self::PGPrefixFactorial,
            UnaryOperator::PGAbs => Self::PGAbs,
        }
    }
}

impl From<Expr> for sqlparser::ast::Expr {
    fn from(expr: Expr) -> Self {
        match expr.value {
            ExprEnum::Identifier(ident) => {
                sqlparser::ast::Expr::Identifier(sqlparser::ast::Ident {
                    value: ident.value,
                    quote_style: None,
                })
            }
            ExprEnum::CompoundIdentifier(idents) => sqlparser::ast::Expr::CompoundIdentifier(
                idents
                    .iter()
                    .map(|i| sqlparser::ast::Ident {
                        value: i.value.clone(),
                        quote_style: None,
                    })
                    .collect(),
            ),
            ExprEnum::Value(value) => {
                sqlparser::ast::Expr::Value(sqlparser::ast::Value::from(value))
            }
            ExprEnum::BinaryOp(op) => sqlparser::ast::Expr::BinaryOp {
                left: Box::new(sqlparser::ast::Expr::from(*op.left.clone())),
                op: sqlparser::ast::BinaryOperator::from(op.op),
                right: Box::new(sqlparser::ast::Expr::from(*op.right.clone())),
            },
            ExprEnum::IsFalse(expr) => {
                sqlparser::ast::Expr::IsFalse(Box::new(sqlparser::ast::Expr::from(*expr.clone())))
            }
            ExprEnum::IsNotFalse(expr) => sqlparser::ast::Expr::IsNotFalse(Box::new(
                sqlparser::ast::Expr::from(*expr.clone()),
            )),
            ExprEnum::IsTrue(expr) => {
                sqlparser::ast::Expr::IsTrue(Box::new(sqlparser::ast::Expr::from(*expr.clone())))
            }
            ExprEnum::IsNotTrue(expr) => {
                sqlparser::ast::Expr::IsNotTrue(Box::new(sqlparser::ast::Expr::from(*expr.clone())))
            }
            ExprEnum::IsNull(expr) => {
                sqlparser::ast::Expr::IsNull(Box::new(sqlparser::ast::Expr::from(*expr.clone())))
            }
            ExprEnum::IsNotNull(expr) => {
                sqlparser::ast::Expr::IsNotNull(Box::new(sqlparser::ast::Expr::from(*expr.clone())))
            }
            ExprEnum::IsUnknown(expr) => {
                sqlparser::ast::Expr::IsUnknown(Box::new(sqlparser::ast::Expr::from(*expr.clone())))
            }
            ExprEnum::IsNotUnknown(expr) => sqlparser::ast::Expr::IsNotUnknown(Box::new(
                sqlparser::ast::Expr::from(*expr.clone()),
            )),
            ExprEnum::UnaryOp(op) => sqlparser::ast::Expr::UnaryOp {
                expr: Box::new(sqlparser::ast::Expr::from(*op.expr.clone())),
                op: sqlparser::ast::UnaryOperator::from(op.op)
            },
            ExprEnum::SimilarTo(..)
            | ExprEnum::Nested(..)
            | ExprEnum::NotImplemented(..)
            | ExprEnum::Between(..)
            | ExprEnum::Like(..)
            | ExprEnum::ILike(..)
            | ExprEnum::InUnnest(..)
            | ExprEnum::InSubquery(..)
            | ExprEnum::InList(..)
            | ExprEnum::AllOp(..)
            | ExprEnum::AnyOp(..)
            | ExprEnum::CompositeAccess(..) => sqlparser::ast::Expr::Identifier(sqlparser::ast::Ident {
                value: "abd".to_string(),
                quote_style: None,
            }),
        }
    }
}
impl From<SelectItem> for sqlparser::ast::SelectItem {
    fn from(select_item: SelectItem) -> Self {
        match select_item {
            SelectItem::UnnamedExpr(expr) => {
                sqlparser::ast::SelectItem::UnnamedExpr(sqlparser::ast::Expr::from(expr))
            }
            SelectItem::Wildcard(_) => {
                sqlparser::ast::SelectItem::Wildcard(sqlparser::ast::WildcardAdditionalOptions {
                    opt_exclude: None,
                    opt_except: None,
                    opt_rename: None,
                })
            }
            _ => sqlparser::ast::SelectItem::UnnamedExpr(sqlparser::ast::Expr::Identifier(
                sqlparser::ast::Ident {
                    value: "abd".to_string(),
                    quote_style: None,
                },
            )),
        }
    }
}
impl From<Ident> for sqlparser::ast::Ident {
    fn from(ident: Ident) -> Self {
        sqlparser::ast::Ident {
            value: ident.value,
            quote_style: None,
        }
    }
}
impl From<ObjectName> for sqlparser::ast::ObjectName {
    fn from(object_name: ObjectName) -> Self {
        sqlparser::ast::ObjectName(
            object_name
                .names
                .iter()
                .map(|l| sqlparser::ast::Ident::from(l.clone()))
                .collect(),
        )
    }
}
impl From<TableFactor> for sqlparser::ast::TableFactor {
    fn from(select_item: TableFactor) -> Self {
        let name = sqlparser::ast::ObjectName([].to_vec());
        match select_item {
            TableFactor::Table(table) => sqlparser::ast::TableFactor::Table {
                name: sqlparser::ast::ObjectName::from(table.name),
                alias: None,
                args: None,
                with_hints: [].to_vec(),
            },
            _ => sqlparser::ast::TableFactor::Table {
                name: name,
                alias: None,
                args: None,
                with_hints: [].to_vec(),
            },
        }
    }
}
impl From<JoinOperator> for sqlparser::ast::JoinOperator {
    fn from(join_operator: JoinOperator) -> Self {
        match join_operator {
            _ => sqlparser::ast::JoinOperator::CrossJoin
        }
    }
}
impl From<Join> for sqlparser::ast::Join {
    fn from(join: Join) -> Self {
        sqlparser::ast::Join {
            relation: sqlparser::ast::TableFactor::from(join.relation),
            join_operator: sqlparser::ast::JoinOperator::from(join.join_operator),
        }
    }
}
impl From<TableWithJoins> for sqlparser::ast::TableWithJoins {
    fn from(table_with_joins: TableWithJoins) -> Self {
        sqlparser::ast::TableWithJoins {
            relation: sqlparser::ast::TableFactor::from(table_with_joins.relation),
            joins: table_with_joins.joins.iter().map(|j| sqlparser::ast::Join::from(j.clone())).collect(),
        }
    }
}
impl From<OrderByExpr> for sqlparser::ast::OrderByExpr {
    fn from(order_by_expr: OrderByExpr) -> Self {
        sqlparser::ast::OrderByExpr {
            expr: sqlparser::ast::Expr::from(order_by_expr.expr),
            asc: order_by_expr.asc,
            nulls_first: order_by_expr.nulls_first,
        }
    }
}
impl From<SetExpr> for sqlparser::ast::SetExpr {
    fn from(setexpr: SetExpr) -> Self {
        match setexpr {
            SetExpr::Select(select) => {
                sqlparser::ast::SetExpr::Select(Box::new(sqlparser::ast::Select {
                    distinct: select.distinct,
                    top: None,
                    projection: select
                        .projection
                        .iter()
                        .map(|l| sqlparser::ast::SelectItem::from(l.clone()))
                        .collect(),
                    into: None,
                    from: select
                        .from
                        .iter()
                        .map(|l| sqlparser::ast::TableWithJoins::from(l.clone()))
                        .collect(),
                    lateral_views: [].to_vec(),
                    selection: select.selection.map(|l| sqlparser::ast::Expr::from(l)),
                    group_by: select
                        .group_by
                        .iter()
                        .map(|l| sqlparser::ast::Expr::from(l.clone()))
                        .collect(),
                    cluster_by: [].to_vec(),
                    distribute_by: [].to_vec(),
                    sort_by: select
                        .sort_by
                        .iter()
                        .map(|l| sqlparser::ast::Expr::from(l.clone()))
                        .collect(),
                    having: select.having.map(|l| sqlparser::ast::Expr::from(l)),
                    qualify: None,
                }))
            }
            _ => sqlparser::ast::SetExpr::Select(Box::new(sqlparser::ast::Select {
                distinct: false,
                top: None,
                projection: [].to_vec(),
                into: None,
                from: [].to_vec(),
                lateral_views: [].to_vec(),
                selection: None,
                group_by: [].to_vec(),
                cluster_by: [].to_vec(),
                distribute_by: [].to_vec(),
                sort_by: [].to_vec(),
                having: None,
                qualify: None,
            })),
        }
    }
}
impl From<Statement> for sqlparser::ast::Statement {
    fn from(value: Statement) -> Self {
        match value {
            Statement::Query(query) => {
                sqlparser::ast::Statement::Query(Box::new(sqlparser::ast::Query {
                    body: Box::new(sqlparser::ast::SetExpr::from(query.body)),
                    limit: query.limit.map(|l| sqlparser::ast::Expr::from(l)),
                    with: None,
                    order_by: query
                        .order_by
                        .iter()
                        .map(|l| sqlparser::ast::OrderByExpr::from(l.clone()))
                        .collect(),
                    locks: [].to_vec(),
                    fetch: None,
                    offset: None,
                }))
            }
            _ => sqlparser::ast::Statement::Query(Box::new(sqlparser::ast::Query {
                body: Box::new(sqlparser::ast::SetExpr::Select(Box::new(
                    sqlparser::ast::Select {
                        distinct: false,
                        top: None,
                        projection: [].to_vec(),
                        into: None,
                        from: [].to_vec(),
                        lateral_views: [].to_vec(),
                        selection: None,
                        group_by: [].to_vec(),
                        cluster_by: [].to_vec(),
                        distribute_by: [].to_vec(),
                        sort_by: [].to_vec(),
                        having: None,
                        qualify: None,
                    },
                ))),

                limit: None,
                with: None,
                order_by: [].to_vec(),
                locks: [].to_vec(),
                fetch: None,
                offset: None,
            })),
        }
    }
}
#[derive(NifStruct)]
//#[rustler(encode)]
#[module = "SqlParser.Values"]
#[derive(Clone)]
pub struct Values {
    explicit_row: bool,
    rows: Vec<Vec<Expr>>,
}
#[derive(NifTaggedEnum, Clone)]
pub enum SetOperator {
    Union,
    Except,
    Intersect,
}
#[derive(NifTaggedEnum, Clone)]
pub enum SetQuantifier {
    All,
    Distinct,
    None,
}
#[derive(NifStruct)]
//#[rustler(encode)]
#[module = "SqlParser.SetOperation"]
#[derive(Clone)]
pub struct SetOperation {
    op: SetOperator,
    set_quantifier: SetQuantifier,
    left: Box<SetExpr>,
    right: Box<SetExpr>,
}

#[derive(Clone, NifUntaggedEnum)]
// #[rustler(encode)]
pub enum SetExpr {
    Select(Select),
    Query(Box<Query>),
    Values(Values),
    SetOperation(SetOperation),
    // Insert(Statement),
    NotImplemented(Atom),
}
impl Encoder for Box<SetExpr> {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        let data = &**self;
        data.encode(env)
    }
}
impl Decoder<'_> for Box<SetExpr> {
    fn decode<'a>(_term: Term<'a>) -> NifResult<Self> {
        Err(rustler::error::Error::BadArg)
    }
}

#[derive(NifStruct)]
#[module = "SqlParser.Wildcard"]
#[derive(Clone)]
pub struct Wildcard {}
impl Wildcard {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(NifStruct)]
//#[rustler(encode)]
#[module = "SqlParser.ExprWithAlias"]
#[derive(Clone)]
pub struct ExprWithAlias {
    expr: Expr,
    alias: Ident,
}
#[derive(NifStruct)]
//#[rustler(encode)]
#[module = "SqlParser.JoinConstraint"]
#[derive(Clone)]
pub struct JoinConstraint {
    constraint: JoinConstraintEnum,
    kind: Atom,
}

mod join_constraints_atoms {
    rustler::atoms! {
        on,
        using,
        natural,
        none
    }
}

#[derive(NifUntaggedEnum, Clone)]
//#[rustler(encode)]
pub enum JoinConstraintEnum {
    On(Expr),
    Using(Vec<Ident>),
    Natural(Atom),
    None(Atom),
}
impl From<sqlparser::ast::JoinConstraint> for JoinConstraint {
    fn from(join_constraint: sqlparser::ast::JoinConstraint) -> Self {
        match join_constraint {
            sqlparser::ast::JoinConstraint::On(expr) => JoinConstraint {
                constraint: JoinConstraintEnum::On(Expr::new(expr)),
                kind: join_constraints_atoms::on(),
            },
            sqlparser::ast::JoinConstraint::Using(ident) => JoinConstraint {
                constraint: JoinConstraintEnum::Using(
                    ident.iter().map(|i| Ident::from(i.clone())).collect(),
                ),
                kind: join_constraints_atoms::using(),
            },
            sqlparser::ast::JoinConstraint::Natural => JoinConstraint {
                constraint: JoinConstraintEnum::Natural(join_constraints_atoms::natural()),
                kind: join_constraints_atoms::natural(),
            },
            sqlparser::ast::JoinConstraint::None => JoinConstraint {
                constraint: JoinConstraintEnum::None(join_constraints_atoms::none()),
                kind: join_constraints_atoms::none(),
            },
        }
    }
}

#[derive(NifStruct, Clone)]
//#[rustler(encode)]
#[module = "SqlParser.JoinOperator"]
pub struct JoinOperator {
    operator: JoinOperatorEnum,
    kind: Atom,
}

mod join_operator_atoms {
    rustler::atoms! {
        inner,
        left_outer,
        right_outer,
        full_outer,
        cross_join,
        left_semi,
        right_semi,
        left_anti,
        right_anti,
        cross_apply,
        outer_apply
    }
}
#[derive(NifUntaggedEnum, Clone)]
//#[rustler(encode)]
pub enum JoinOperatorEnum {
    Inner(JoinConstraint),
    LeftOuter(JoinConstraint),
    RightOuter(JoinConstraint),
    FullOuter(JoinConstraint),
    // CrossJoin,
    LeftSemi(JoinConstraint),
    RightSemi(JoinConstraint),
    LeftAnti(JoinConstraint),
    RightAnti(JoinConstraint),
    // CrossApply,
    // OuterApply,
}

impl From<sqlparser::ast::JoinOperator> for JoinOperator {
    fn from(join_operator: sqlparser::ast::JoinOperator) -> Self {
        match join_operator {
            sqlparser::ast::JoinOperator::Inner(constraint) => JoinOperator {
                kind: join_operator_atoms::inner(),
                operator: JoinOperatorEnum::Inner(JoinConstraint::from(constraint)),
            },
            sqlparser::ast::JoinOperator::LeftOuter(constraint) => JoinOperator {
                kind: join_operator_atoms::left_outer(),
                operator: JoinOperatorEnum::LeftOuter(JoinConstraint::from(constraint)),
            },
            sqlparser::ast::JoinOperator::RightOuter(constraint) => JoinOperator {
                kind: join_operator_atoms::right_outer(),
                operator: JoinOperatorEnum::RightOuter(JoinConstraint::from(constraint)),
            },
            sqlparser::ast::JoinOperator::FullOuter(constraint) => JoinOperator {
                kind: join_operator_atoms::full_outer(),
                operator: JoinOperatorEnum::FullOuter(JoinConstraint::from(constraint)),
            },
            // sqlparser::ast::JoinOperator::CrossJoin => JoinOperator{ operator: JoinOperatorEnum::CrossJoin, },
            sqlparser::ast::JoinOperator::LeftSemi(constraint) => JoinOperator {
                kind: join_operator_atoms::left_semi(),
                operator: JoinOperatorEnum::LeftSemi(JoinConstraint::from(constraint)),
            },
            sqlparser::ast::JoinOperator::RightSemi(constraint) => JoinOperator {
                kind: join_operator_atoms::right_semi(),
                operator: JoinOperatorEnum::RightSemi(JoinConstraint::from(constraint)),
            },
            sqlparser::ast::JoinOperator::LeftAnti(constraint) => JoinOperator {
                kind: join_operator_atoms::left_anti(),
                operator: JoinOperatorEnum::LeftAnti(JoinConstraint::from(constraint)),
            },
            sqlparser::ast::JoinOperator::RightAnti(constraint) => JoinOperator {
                kind: join_operator_atoms::right_anti(),
                operator: JoinOperatorEnum::RightAnti(JoinConstraint::from(constraint)),
            },
            _ => todo!(),
            // sqlparser::ast::JoinOperator::CrossApply => JoinOperator{ operator: JoinOperatorEnum::CrossApply, },
            // sqlparser::ast::JoinOperator::OuterApply => JoinOperator{ operator: JoinOperatorEnum::OuterApply },
        }
    }
}

#[derive(NifUntaggedEnum, Clone)]
//#[rustler(encode)]
pub enum SelectItem {
    Wildcard(Wildcard),
    UnnamedExpr(Expr),
    NotImplemented(Atom),
    ExprWithAlias(ExprWithAlias), // QualifiedWildcard(ObjectName, WildcardAdditionalOptions),
}
#[derive(NifStruct)]
//#[rustler(encode)]
#[module = "SqlParser.Join"]
#[derive(Clone)]
pub struct Join {
    pub relation: TableFactor,
    pub join_operator: JoinOperator,
}

#[derive(NifStruct)]
//#[rustler(encode)]
#[module = "SqlParser.TableWithJoins"]
#[derive(Clone)]
pub struct TableWithJoins {
    pub relation: TableFactor,
    pub joins: Vec<Join>,
}
impl TableWithJoins {
    pub fn new(ast: &sqlparser::ast::TableWithJoins) -> Self {
        let relation = TableFactor::from(ast.relation.clone());
        Self {
            relation: relation,
            joins: ast
                .joins
                .iter()
                .map(|j| Join {
                    join_operator: JoinOperator::from(j.join_operator.clone()),
                    relation: TableFactor::from(j.relation.clone()),
                })
                .collect(),
        }
    }
}
#[derive(NifStruct)]
#[module = "SqlParser.Ident"]
#[derive(Clone)]
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
#[derive(Clone)]
pub struct ObjectName {
    names: Vec<Ident>,
}

#[derive(NifUntaggedEnum, Clone)]
pub enum TableFactor {
    Table(Table),
    NotImplemented(Atom),
}

impl From<sqlparser::ast::TableFactor> for TableFactor {
    fn from(table_factor: sqlparser::ast::TableFactor) -> Self {
        match table_factor {
            sqlparser::ast::TableFactor::Table { name, .. } => TableFactor::Table(Table {
                name: ObjectName {
                    names: name.0.iter().map(|p| Ident::from(p.clone())).collect(),
                },
            }),
            sqlparser::ast::TableFactor::NestedJoin { .. } => {
                TableFactor::NotImplemented(result_atoms::not_implemented())
            }
            sqlparser::ast::TableFactor::Derived { .. } => {
                TableFactor::NotImplemented(result_atoms::not_implemented())
            }
            sqlparser::ast::TableFactor::TableFunction { .. } => {
                TableFactor::NotImplemented(result_atoms::not_implemented())
            }
            sqlparser::ast::TableFactor::UNNEST { .. } => {
                TableFactor::NotImplemented(result_atoms::not_implemented())
            }
        }
    }
}

#[derive(NifStruct)]
#[module = "SqlParser.Table"]
#[derive(Clone)]
pub struct Table {
    name: ObjectName,
}

#[derive(NifTaggedEnum, Clone)]
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

#[derive(NifTaggedEnum, Clone)]
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
//#[rustler(encode)]
#[module = "SqlParser.UnaryOp"]
#[derive(Clone)]
pub struct UnaryOp {
    op: UnaryOperator,
    expr: Box<Expr>,
}
#[derive(NifStruct)]
//#[rustler(encode)]
#[module = "SqlParser.CompositeAccess"]
#[derive(Clone)]
pub struct CompositeAccess {
    expr: Box<Expr>,
    key: Ident,
}

#[derive(NifStruct)]
//#[rustler(encode)]
#[derive(Clone)]
#[module = "SqlParser.BinaryOp"]
pub struct BinaryOp {
    left: Box<Expr>,
    op: BinaryOperator,
    right: Box<Expr>,
}
#[derive(NifStruct)]
//#[rustler(encode)]
#[module = "SqlParser.Number"]
#[derive(Clone)]
pub struct Number {
    value: String,
    long: bool,
}
#[derive(NifStruct)]
//#[rustler(encode)]
#[module = "SqlParser.Boolean"]
#[derive(Clone)]
pub struct Boolean {
    value: bool,
}

#[derive(NifStruct)]
//#[rustler(encode)]
#[module = "SqlParser.Null"]
#[derive(Clone)]
pub struct Null {}

#[derive(NifUntaggedEnum)]
//#[rustler(encode)]
#[derive(Clone)]
pub enum Value {
    Number(Number),
    SingleQuotedString(String),
    // DollarQuotedString(DollarQuotedString),
    EscapedStringLiteral(String),
    NationalStringLiteral(String),
    HexStringLiteral(String),
    DoubleQuotedString(String),
    Boolean(Boolean),
    Null(Null),
    Placeholder(String),
    UnQuotedString(String),
    NotImplemented(Atom),
}
impl From<sqlparser::ast::Value> for Value {
    fn from(value: sqlparser::ast::Value) -> Self {
        match value {
            sqlparser::ast::Value::Number(num, long) => Self::Number(Number {
                value: num,
                long: long,
            }),
            sqlparser::ast::Value::SingleQuotedString(string) => Self::SingleQuotedString(string),
            sqlparser::ast::Value::DollarQuotedString(_dollar_quoted_string) => {
                Self::NotImplemented(result_atoms::not_implemented())
            }
            sqlparser::ast::Value::EscapedStringLiteral(string) => {
                Self::EscapedStringLiteral(string)
            }
            sqlparser::ast::Value::NationalStringLiteral(string) => {
                Self::NationalStringLiteral(string)
            }
            sqlparser::ast::Value::HexStringLiteral(string) => Self::HexStringLiteral(string),
            sqlparser::ast::Value::DoubleQuotedString(string) => Self::DoubleQuotedString(string),
            sqlparser::ast::Value::Boolean(boolean) => Self::Boolean(Boolean { value: boolean }),
            sqlparser::ast::Value::Null => Self::Null(Null {}),
            sqlparser::ast::Value::Placeholder(placeholder) => Self::Placeholder(placeholder),
            sqlparser::ast::Value::UnQuotedString(string) => Self::UnQuotedString(string),
        }
    }
}

#[derive(NifStruct)]
//#[rustler(encode)]
#[derive(Clone)]
#[module = "SqlParser.InList"]
pub struct InList {
    expr: Box<Expr>,
    list: Vec<Expr>,
    negated: bool,
}
#[derive(NifStruct)]
//#[rustler(encode)]
#[module = "SqlParser.InSubquery"]
#[derive(Clone)]
pub struct InSubquery {
    expr: Box<Expr>,
    subquery: Box<Query>,
    negated: bool,
}
#[derive(NifStruct)]
//#[rustler(encode)]
#[module = "SqlParser.InUnnest"]
#[derive(Clone)]
pub struct InUnnest {
    expr: Box<Expr>,
    array_expr: Box<Expr>,
    negated: bool,
}

#[derive(NifStruct)]
//#[rustler(encode)]
#[module = "SqlParser.Between"]
#[derive(Clone)]
pub struct Between {
    expr: Box<Expr>,
    negated: bool,
    low: Box<Expr>,
    high: Box<Expr>,
}
#[derive(NifStruct)]
//#[rustler(encode)]
#[module = "SqlParser.SimilarTo"]
#[derive(Clone)]
pub struct SimilarTo {
    negated: bool,
    expr: Box<Expr>,
    pattern: Box<Expr>,
    escape_char: Option<String>,
}
#[derive(NifStruct)]
//#[rustler(encode)]
#[module = "SqlParser.Like"]
#[derive(Clone)]
pub struct Like {
    negated: bool,
    expr: Box<Expr>,
    pattern: Box<Expr>,
    escape_char: Option<String>,
}
#[derive(NifStruct)]
//#[rustler(encode)]
#[module = "SqlParser.ILike"]
#[derive(Clone)]
pub struct ILike {
    negated: bool,
    expr: Box<Expr>,
    pattern: Box<Expr>,
    escape_char: Option<String>,
}
#[derive(NifUntaggedEnum)]
//#[rustler(encode)]
#[derive(Clone)]
pub enum ExprEnum {
    Identifier(Ident),
    CompoundIdentifier(Vec<Ident>),
    // jsonaccess
    CompositeAccess(CompositeAccess),

    UnaryOp(UnaryOp),
    IsFalse(Box<Expr>),
    IsNotFalse(Box<Expr>),
    IsTrue(Box<Expr>),
    IsNotTrue(Box<Expr>),
    IsNull(Box<Expr>),
    IsNotNull(Box<Expr>),
    IsUnknown(Box<Expr>),
    IsNotUnknown(Box<Expr>),
    // IsDistinctFrom(Box<Expr>, Box<Expr>),
    // IsNotDistinctFrom(Box<Expr>, Box<Expr>),
    InList(InList),
    InSubquery(InSubquery),
    InUnnest(InUnnest),
    Between(Between),
    BinaryOp(BinaryOp),
    Like(Like),
    ILike(ILike),
    SimilarTo(SimilarTo),
    AnyOp(Box<Expr>),
    AllOp(Box<Expr>),
    Nested(Box<Expr>),
    Value(Value),
    NotImplemented(Atom),
}
#[derive(NifStruct, Clone)]
//#[rustler(encode)]
#[module = "SqlParser.Expr"]
pub struct Expr {
    r#type: Atom,
    value: ExprEnum,
}

impl Encoder for Box<Expr> {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        let data = &**self;
        data.encode(env)
    }
}
impl Decoder<'_> for Box<Expr> {
    fn decode<'a>(term: Term<'a>) -> NifResult<Self> {
        let expr: Expr = term.decode()?;
        println!("{:#?}", term);
        // println!("{:#?}", expr);
        // Err(rustler::error::Error::BadArg)
        Ok(Box::new(expr))
    }
}
impl Expr {
    pub fn new(ast: sqlparser::ast::Expr) -> Self {
        match ast {
            sqlparser::ast::Expr::Identifier(ident) => Expr {
                r#type: type_atoms::identifier(),
                value: ExprEnum::Identifier(Ident::from(ident)),
            },
            sqlparser::ast::Expr::CompoundIdentifier(idents) => Expr {
                r#type: type_atoms::compound_identifier(),
                value: ExprEnum::CompoundIdentifier(
                    idents.iter().map(|p| Ident::from(p.clone())).collect(),
                ),
            },
            sqlparser::ast::Expr::CompositeAccess { expr, key } => Expr {
                r#type: type_atoms::composite_access(),
                value: ExprEnum::CompositeAccess(CompositeAccess {
                    expr: Box::new(Expr::new(*expr)),
                    key: Ident::from(key),
                }),
            },
            sqlparser::ast::Expr::IsFalse(expr) => Expr {
                r#type: type_atoms::is_false(),
                value: ExprEnum::IsFalse(Box::new(Expr::new(*expr))),
            },
            sqlparser::ast::Expr::IsNotFalse(expr) => Expr {
                r#type: type_atoms::is_not_false(),
                value: ExprEnum::IsNotFalse(Box::new(Expr::new(*expr))),
            },
            sqlparser::ast::Expr::IsTrue(expr) => Expr {
                r#type: type_atoms::is_true(),
                value: ExprEnum::IsTrue(Box::new(Expr::new(*expr))),
            },
            sqlparser::ast::Expr::IsNotTrue(expr) => Expr {
                r#type: type_atoms::is_not_true(),
                value: ExprEnum::IsNotTrue(Box::new(Expr::new(*expr))),
            },
            sqlparser::ast::Expr::IsNull(expr) => Expr {
                r#type: type_atoms::is_null(),
                value: ExprEnum::IsNull(Box::new(Expr::new(*expr))),
            },
            sqlparser::ast::Expr::IsNotNull(expr) => Expr {
                r#type: type_atoms::is_not_null(),
                value: ExprEnum::IsNotNull(Box::new(Expr::new(*expr))),
            },
            sqlparser::ast::Expr::IsUnknown(expr) => Expr {
                r#type: type_atoms::is_unknown(),
                value: ExprEnum::IsUnknown(Box::new(Expr::new(*expr))),
            },
            sqlparser::ast::Expr::IsNotUnknown(expr) => Expr {
                r#type: type_atoms::is_not_unknown(),
                value: ExprEnum::IsNotUnknown(Box::new(Expr::new(*expr))),
            },
            sqlparser::ast::Expr::InList {
                expr,
                list,
                negated,
            } => Expr {
                r#type: type_atoms::in_list(),
                value: ExprEnum::InList(InList {
                    expr: Box::new(Expr::new(*expr)),
                    list: list.iter().map(|p| Expr::new(p.clone())).collect(),
                    negated: negated,
                }),
            },
            sqlparser::ast::Expr::InSubquery {
                expr,
                subquery,
                negated,
            } => Expr {
                r#type: type_atoms::in_subquery(),
                value: ExprEnum::InSubquery(InSubquery {
                    expr: Box::new(Expr::new(*expr)),
                    subquery: Box::new(Query::new(*subquery)),
                    negated: negated,
                }),
            },
            sqlparser::ast::Expr::InUnnest {
                expr,
                array_expr,
                negated,
            } => Expr {
                r#type: type_atoms::in_subquery(),
                value: ExprEnum::InUnnest(InUnnest {
                    expr: Box::new(Expr::new(*expr)),
                    array_expr: Box::new(Expr::new(*array_expr)),
                    negated: negated,
                }),
            },
            sqlparser::ast::Expr::Between {
                expr,
                negated,
                low,
                high,
            } => Expr {
                r#type: type_atoms::in_subquery(),
                value: ExprEnum::Between(Between {
                    expr: Box::new(Expr::new(*expr)),
                    negated: negated,
                    low: Box::new(Expr::new(*low)),
                    high: Box::new(Expr::new(*high)),
                }),
            },
            sqlparser::ast::Expr::BinaryOp { left, op, right } => Expr {
                r#type: type_atoms::binary_op(),
                value: ExprEnum::BinaryOp(BinaryOp {
                    left: Box::new(Expr::new(*left)),
                    op: op.into(),
                    right: Box::new(Expr::new(*right)),
                }),
            },
            sqlparser::ast::Expr::Like {
                negated,
                expr,
                pattern,
                escape_char,
            } => Expr {
                r#type: type_atoms::like(),
                value: ExprEnum::Like(Like {
                    expr: Box::new(Expr::new(*expr)),
                    negated: negated,
                    pattern: Box::new(Expr::new(*pattern)),
                    escape_char: match escape_char {
                        Some(c) => Some(c.to_string()),
                        None => None,
                    },
                }),
            },
            sqlparser::ast::Expr::ILike {
                negated,
                expr,
                pattern,
                escape_char,
            } => Expr {
                r#type: type_atoms::ilike(),
                value: ExprEnum::ILike(ILike {
                    expr: Box::new(Expr::new(*expr)),
                    negated: negated,
                    pattern: Box::new(Expr::new(*pattern)),
                    escape_char: match escape_char {
                        Some(c) => Some(c.to_string()),
                        None => None,
                    },
                }),
            },
            sqlparser::ast::Expr::SimilarTo {
                negated,
                expr,
                pattern,
                escape_char,
            } => Expr {
                r#type: type_atoms::similar_to(),
                value: ExprEnum::SimilarTo(SimilarTo {
                    expr: Box::new(Expr::new(*expr)),
                    negated: negated,
                    pattern: Box::new(Expr::new(*pattern)),
                    escape_char: match escape_char {
                        Some(c) => Some(c.to_string()),
                        None => None,
                    },
                }),
            },
            sqlparser::ast::Expr::AnyOp(expr) => Expr {
                r#type: type_atoms::any_op(),
                value: ExprEnum::AnyOp(Box::new(Expr::new(*expr))),
            },
            sqlparser::ast::Expr::AllOp(expr) => Expr {
                r#type: type_atoms::all_op(),
                value: ExprEnum::AllOp(Box::new(Expr::new(*expr))),
            },
            sqlparser::ast::Expr::Nested(expr) => Expr {
                r#type: type_atoms::nested(),
                value: ExprEnum::Nested(Box::new(Expr::new(*expr))),
            },
            sqlparser::ast::Expr::UnaryOp { op, expr } => Expr {
                r#type: type_atoms::unary_op(),
                value: ExprEnum::UnaryOp(UnaryOp {
                    op: op.into(),
                    expr: Box::new(Expr::new(*expr)),
                }),
            },
            sqlparser::ast::Expr::Value(value) => Expr {
                r#type: type_atoms::value(),
                value: ExprEnum::Value(value.into()),
            },
            sqlparser::ast::Expr::SafeCast { .. }
            | sqlparser::ast::Expr::TryCast { .. }
            | sqlparser::ast::Expr::Cast { .. }
            | sqlparser::ast::Expr::JsonAccess { .. }
            | sqlparser::ast::Expr::IsDistinctFrom(_, _)
            | sqlparser::ast::Expr::AtTimeZone { .. }
            | sqlparser::ast::Expr::Extract { .. }
            | sqlparser::ast::Expr::Ceil { .. }
            | sqlparser::ast::Expr::Floor { .. }
            | sqlparser::ast::Expr::Position { .. }
            | sqlparser::ast::Expr::Substring { .. }
            | sqlparser::ast::Expr::Trim { .. }
            | sqlparser::ast::Expr::Overlay { .. }
            | sqlparser::ast::Expr::Collate { .. }
            | sqlparser::ast::Expr::TypedString { .. }
            | sqlparser::ast::Expr::MapAccess { .. }
            | sqlparser::ast::Expr::Function(_)
            | sqlparser::ast::Expr::AggregateExpressionWithFilter { .. }
            | sqlparser::ast::Expr::Case { .. }
            | sqlparser::ast::Expr::Exists { .. }
            | sqlparser::ast::Expr::Subquery { .. }
            | sqlparser::ast::Expr::ArraySubquery(_)
            | sqlparser::ast::Expr::ListAgg(_)
            | sqlparser::ast::Expr::ArrayAgg(_)
            | sqlparser::ast::Expr::GroupingSets(_)
            | sqlparser::ast::Expr::Cube(_)
            | sqlparser::ast::Expr::Rollup(_)
            | sqlparser::ast::Expr::Tuple(_)
            | sqlparser::ast::Expr::ArrayIndex { .. }
            | sqlparser::ast::Expr::Array(_)
            | sqlparser::ast::Expr::Interval { .. }
            | sqlparser::ast::Expr::MatchAgainst { .. }
            | sqlparser::ast::Expr::IsNotDistinctFrom(_, _) => Expr {
                r#type: result_atoms::not_implemented(),
                value: ExprEnum::NotImplemented(result_atoms::not_implemented()),
            },
        }
    }
}
#[derive(NifStruct)]
// #[rustler(encode)]
#[module = "SqlParser.Select"]
#[derive(Clone)]
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
                    sqlparser::ast::SelectItem::QualifiedWildcard(_, _) => {
                        SelectItem::NotImplemented(result_atoms::not_implemented())
                    }
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

#[derive(Clone, NifStruct)]
//#[rustler(encode)]
#[module = "SqlParser.Offset"]
pub struct Offset {
    pub value: Expr,
    pub rows: OffsetRows,
}

#[derive(NifStruct)]
//#[rustler(encode)]
#[derive(Clone)]
#[module = "SqlParser.OrderByExpr"]
pub struct OrderByExpr {
    pub expr: Expr,
    pub asc: Option<bool>,
    pub nulls_first: Option<bool>,
}

#[derive(Clone, NifStruct)]
// #[rustler(encode)]
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
#[derive(NifTaggedEnum, Clone)]
pub enum OffsetRows {
    None,
    Row,
    Rows,
}

impl Encoder for Box<Query> {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        let data = &**self;
        data.encode(env)
    }
}
impl Decoder<'_> for Box<Query> {
    fn decode<'a>(_term: Term<'a>) -> NifResult<Self> {
        Err(rustler::error::Error::BadArg)
    }
}
impl From<sqlparser::ast::SetExpr> for SetExpr {
    fn from(set_expr: sqlparser::ast::SetExpr) -> Self {
        match set_expr {
            sqlparser::ast::SetExpr::Select(select) => SetExpr::Select(Select::new(*select)),
            sqlparser::ast::SetExpr::Query(query) => SetExpr::Query(Box::new(Query::new(*query))),
            sqlparser::ast::SetExpr::Values(values) => SetExpr::Values(Values {
                rows: values
                    .rows
                    .iter()
                    .map(|row| row.iter().map(|expr| Expr::new(expr.clone())).collect())
                    .collect(),
                explicit_row: values.explicit_row,
            }),
            sqlparser::ast::SetExpr::SetOperation {
                op,
                set_quantifier,
                left,
                right,
            } => SetExpr::SetOperation(SetOperation {
                op: match op {
                    sqlparser::ast::SetOperator::Union => SetOperator::Union,
                    sqlparser::ast::SetOperator::Except => SetOperator::Except,
                    sqlparser::ast::SetOperator::Intersect => SetOperator::Intersect,
                },
                set_quantifier: match set_quantifier {
                    sqlparser::ast::SetQuantifier::All => SetQuantifier::All,
                    sqlparser::ast::SetQuantifier::Distinct => SetQuantifier::Distinct,
                    sqlparser::ast::SetQuantifier::None => SetQuantifier::None,
                },
                left: Box::new((*left).into()),
                right: Box::new((*right).into()),
            }),
            sqlparser::ast::SetExpr::Insert(_) | sqlparser::ast::SetExpr::Table(_) => {
                SetExpr::NotImplemented(result_atoms::not_implemented())
            }
        }
    }
}

impl Query {
    pub fn new(ast: sqlparser::ast::Query) -> Self {
        Self {
            body: (*ast.body).into(),
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

#[derive(Clone, NifUntaggedEnum)]
// #[rustler(encode)]
pub enum Statement {
    Query(Query),
    NotImplemented(Atom),
}
