defmodule SqlParser do
  @moduledoc """
  Documentation for `SqlParser`.
  """

  @doc """
  Parses a SQL query into an AST.

  Optionally accepts a dialect and a recursion limit.

  Examples

    iex>  {:ok, [%SqlParser.Query{}]} = SqlParser.parse("SELECT * FROM foo", dialect: :postgres, recursion_limit: 10)

    iex>  {:ok, [%SqlParser.Query{}]} = SqlParser.parse("SELECT * FROM foo")
  """
  def parse(sql, opts \\ [])

  def parse(sql, dialect) when is_atom(dialect) do
    parse(sql, dialect: dialect)
  end

  def parse(sql, opts) when is_list(opts) do
    dialect = opts[:dialect] || :ansi
    recursion_limit = (opts[:recursion_limit] && {:limit, opts[:recursion_limit]}) || :infinity

    case SqlParser.Parse.parse_statements(sql, dialect, recursion_limit) do
      {:ok, %SqlParser.Document{statements: statements}} -> {:ok, statements}
      {:error, error} -> {:error, error}
    end
  end

  def to_sql(ast, opts \\ []) do
    dialect = opts[:dialect] || :ansi

    case SqlParser.Parse.to_sql(ast, dialect) do
      {:ok, sql} -> {:ok, sql}
      {:error, error} -> {:error, error}
    end
  end
end
