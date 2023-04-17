defmodule SqlParser.Document do
  defstruct [:statements]
end

defmodule SqlParser.Values do
  defstruct [:explicit_row, :rows]
end

defmodule SqlParser.SetOperation do
  defstruct [:op, :set_quantifier, :left, :right]
end

defmodule SqlParser.Query do
  defstruct [:body, :order_by, :limit, :offset]
end

defmodule SqlParser.Expr do
  defstruct [:type, :value]
end

defmodule SqlParser.Number do
  defstruct [:value, :long]
end

defmodule SqlParser.Boolean do
  defstruct [:value]
end

defmodule SqlParser.Null do
  defstruct []
end

defmodule SqlParser.ExprWithAlias do
  defstruct [:alias, :expr]
end

defmodule SqlParser.Select do
  defstruct [
    :distinct,
    :projection,
    :selection,
    :having,
    from: [],
    group_by: [],
    sort_by: []
  ]
end

defmodule SqlParser.OrderByExpr do
  defstruct [:expr, :asc, :nulls_first]
end

defmodule SqlParser.JoinOperator do
  defstruct [:operator, :kind]
end

defmodule SqlParser.JoinConstraint do
  defstruct [:constraint, :kind]
end

defmodule SqlParser.UnaryOp do
  defstruct [:op, :expr]
end

defmodule SqlParser.BinaryOp do
  defstruct [:left, :op, :right]
end

defmodule SqlParser.Wildcard do
  defstruct []
end

defmodule SqlParser.TableWithJoins do
  defstruct [:relation, :joins]
end

defmodule SqlParser.Join do
  defstruct [:join_operator, :relation]
end

defmodule SqlParser.Ident do
  defstruct [:quote_style, :value]
end

defmodule SqlParser.Table do
  defstruct [:name]
end

defmodule SqlParser.ObjectName do
  defstruct [:names]
end

defmodule SqlParser.CompositeAccess do
  defstruct [:expr, :key]
end

defmodule SqlParser.InList do
  defstruct [:expr, :list, :negated]
end

defmodule SqlParser.InSubquery do
  defstruct [:expr, :subquery, :negated]
end

defmodule SqlParser.InUnnest do
  defstruct [:expr, :array_expr, :negated]
end

defmodule SqlParser.Between do
  defstruct [:expr, :negated, :low, :high]
end

defmodule SqlParser.SimilarTo do
  defstruct [:expr, :negated, :pattern, :escape_char]
end

defmodule SqlParser.Like do
  defstruct [:expr, :negated, :pattern, :escape_char]
end

defmodule SqlParser.ILike do
  defstruct [:expr, :negated, :pattern, :escape_char]
end

defmodule SqlParser.Offset do
  defstruct [:value, :rows]
end
