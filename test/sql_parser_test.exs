defmodule SqlParserTest do
  use ExUnit.Case
  doctest SqlParser

  test "greets the world" do
    assert {:ok, %SqlParser.Document{} = doc} = SqlParser.Parse.run("SELECT * FROM a")

    assert %SqlParser.Document{
             statements: [
               %SqlParser.Query{body: %SqlParser.Select{projection: [%SqlParser.Wildcard{}]}}
             ]
           } = doc
  end
end
