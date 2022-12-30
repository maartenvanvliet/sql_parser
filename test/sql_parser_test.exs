defmodule SqlParserTest do
  use ExUnit.Case
  doctest SqlParser

  test "simple query" do
    assert {:ok, %SqlParser.Document{} = doc} =
             SqlParser.Parse.run("SELECT * FROM a WHERE b.a = c")

    assert %SqlParser.Document{
             statements: [
               %SqlParser.Query{
                 body: %SqlParser.Select{
                   projection: [%SqlParser.Wildcard{}],
                   selection: %SqlParser.BinaryOp{
                     left: [
                       %SqlParser.Ident{quote_style: nil, value: "b"},
                       %SqlParser.Ident{quote_style: nil, value: "a"}
                     ],
                     op: :eq,
                     right: %SqlParser.Ident{quote_style: nil, value: "c"}
                   },
                   from: [
                     %SqlParser.TableWithJoins{
                       relation: %SqlParser.Table{
                         name: %SqlParser.ObjectName{
                           names: [%SqlParser.Ident{quote_style: nil, value: "a"}]
                         }
                       }
                     }
                   ]
                 }
               }
             ]
           } = doc
  end

  test "update query" do
    assert {:ok, %SqlParser.Document{statements: [:not_implemented]}} ==
             SqlParser.Parse.run("UPDATE foo SET bar = 1")
  end
  test "group query" do
    assert {:ok, %SqlParser.Document{}} =
             SqlParser.Parse.run("SELECT * FROM a group by e")

  end

  test "order by query" do
    assert {:ok, %SqlParser.Document{} } ==
             SqlParser.Parse.run("SELECT * FROM a ORDER BY f")

  end
  @ops %{
    plus: "+",
    minus: "-",
    multiply: "*",
    divide: "/",
    modulo: "%",
    string_concat: "||",
    gt: ">",
    lt: "<",
    gt_eq: ">=",
    lt_eq: "<=",
    spaceship: "<=>",
    eq: "=",
    not_eq: "<>",
    and: "AND",
    or: "OR",
    xor: "XOR",
    bitwise_or: "|",
    bitwise_and: "&",
    bitwise_xor: "^"
    # pg_bitwise_xor: "#",
    # pg_bitwise_shift_left: "<<",
    # pg_bitwise_shift_right: ">>",
    # pg_regex_match: "~",
    # pg_regex_imatch: "~*",
    # pg_regex_not_match: "!~",
    # pg_regex_not_imatch: "!~*"
  }

  test "ops work" do
    for {name, op} <- @ops do
      assert {:ok, %SqlParser.Document{} = doc} =
               SqlParser.Parse.run("SELECT * FROM a WHERE b.a #{op} d")

      assert %SqlParser.Document{
               statements: [
                 %SqlParser.Query{
                   body: %SqlParser.Select{
                     selection: %SqlParser.BinaryOp{
                       left: _,
                       op: ^name,
                       right: _
                     }
                   }
                 }
               ]
             } = doc
    end
  end

  @exprs %{
    # %SqlParser.BinaryOp{left: %SqlParser.Ident{quote_style: nil, value: "a"}, op: :eq, right: {:number, "1", false}} => "a = 1",

    # %SqlParser.Ident{quote_style: nil, value: "a"} => "a",
    # %SqlParser.Ident{quote_style: nil, value: "d"} => "b IS FALSE",
    false => "c.d IS NULL",
    # SqlParser.Ident => "d IS NOT FALSE",
    # %SqlParser.Ident{quote_style: nil, value: "e"} => "e IS NOT TRUE",
    # [%SqlParser.Ident{}, %SqlParser.Ident{}] => "table_alias.column"
    # minus: "-",
    # multiply: "*",
    # divide: "/",
    # modulo: "%",
    # string_concat: "||",
    # gt: ">",
    # lt: "<",
    # gt_eq: ">=",
    # lt_eq: "<=",
    # spaceship: "<=>",
    # eq: "=",
    # not_eq: "<>",
    # and: "AND",
    # or: "OR",
    # xor: "XOR",
    # bitwise_or: "|",
    # bitwise_and: "&",
    # bitwise_xor: "^",
    # pg_bitwise_xor: "#",
    # pg_bitwise_shift_left: "<<",
    # pg_bitwise_shift_right: ">>",
    # pg_regex_match: "~",
    # pg_regex_imatch: "~*",
    # pg_regex_not_match: "!~",
    # pg_regex_not_imatch: "!~*"
  }
  @tag :f
  test "expr work" do
    for {expected_selection, expr} <- @exprs do
      assert {:ok, %SqlParser.Document{} = doc} =
               SqlParser.Parse.run("SELECT c as b from a WHERE #{expr}" |> dbg)

      assert %SqlParser.Document{
               statements: [
                 %SqlParser.Query{
                   body: %SqlParser.Select{
                    projection: projection,
                     selection: selection
                   }
                 }
               ]
             } = doc

             assert projection == 1
      assert expected_selection == selection
      # assert match?(^module, selection)
    end
  end
end
