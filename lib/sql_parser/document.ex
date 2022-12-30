defmodule SqlParser.Document do
  defstruct [:statements]
end

defmodule SqlParser.Query do
  defstruct [:body,:order_by, :limit, :offset]
end
defmodule SqlParser.Expr do
  defstruct [:type, :val]
end
defmodule SqlParser.ExprWithAlias do
  defstruct [:alias, :expr]
end


defmodule SqlParser.Select do
  defstruct [
    :distinct,
    :projection,
    :selection,
    from: [],
    group_by: [],
    sort_by: []
  ]
end
defmodule SqlParser.OrderByExpr do
  defstruct [:expr, :asc, :nulls_first]

end

defmodule SqlParser.BinaryOp do
  defstruct [:left, :op, :right]
end

defmodule SqlParser.Wildcard do
  defstruct []
end

defmodule SqlParser.TableWithJoins do
  defstruct [:relation]
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
