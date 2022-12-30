defmodule SqlParser do
  @moduledoc """
  Documentation for `SqlParser`.
  """

  @doc """
  Examples

    iex>  {:ok, [%SqlParser.Query{}]} = SqlParser.parse("SELECT * FROM foo")
  """
  def parse(sql) do
    case SqlParser.Parse.run(sql) do
      {:ok, %SqlParser.Document{statements: statements}} -> {:ok, statements}
      {:error, error} -> {:error, error}
    end
  end
end
