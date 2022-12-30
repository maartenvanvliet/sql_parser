defmodule SqlParserTest do
  use ExUnit.Case
  doctest SqlParser
  alias SqlParser.Expr
  alias SqlParser.Ident

  test "simple query" do
    assert {:ok, %SqlParser.Document{} = doc} =
             SqlParser.Parse.run("SELECT * FROM a WHERE b.a = c")

    assert %SqlParser.Document{
             statements: [
               %SqlParser.Query{
                 body: %SqlParser.Select{
                   projection: [%SqlParser.Wildcard{}],
                   selection: _,
                   from: [
                     %SqlParser.TableWithJoins{
                       relation: %SqlParser.Table{
                         name: %SqlParser.ObjectName{
                           names: [%Ident{quote_style: nil, value: "a"}]
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
    assert {:ok, %SqlParser.Document{}} = SqlParser.Parse.run("SELECT * FROM a group by e")
  end

  test "order by query" do
    assert {:ok,
            %SqlParser.Document{
              statements: [
                %SqlParser.Query{
                  body: %SqlParser.Select{
                    distinct: false,
                    from: [
                      %SqlParser.TableWithJoins{
                        relation: %SqlParser.Table{
                          name: %SqlParser.ObjectName{
                            names: [%Ident{quote_style: nil, value: "a"}]
                          }
                        }
                      }
                    ],
                    group_by: [],
                    having: nil,
                    projection: [%SqlParser.Wildcard{}],
                    selection: nil,
                    sort_by: []
                  },
                  order_by: [
                    %SqlParser.OrderByExpr{
                      expr: %Expr{
                        type: :identifier,
                        val: %Ident{quote_style: nil, value: "f"}
                      },
                      asc: nil,
                      nulls_first: nil
                    }
                  ],
                  limit: nil,
                  offset: nil
                }
              ]
            }} ==
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
                     selection: %Expr{
                       type: :binary_op,
                       val: %SqlParser.BinaryOp{
                         left: _,
                         op: ^name,
                         right: _
                       }
                     }
                   }
                 }
               ]
             } = doc
    end
  end

  @exprs %{
    %Expr{
      type: :binary_op,
      val: %SqlParser.BinaryOp{
        left: %Expr{type: :identifier, val: %Ident{quote_style: nil, value: "a"}},
        op: :eq,
        right: %Expr{type: :value, val: {:number, "1", false}}
      }
    } => "a = 1",
    %Expr{
      type: :is_null,
      val: %Expr{type: :identifier, val: %Ident{quote_style: nil, value: "c"}}
    } => "c IS NULL"
  }

  test "expr work" do
    for {expected_selection, expr} <- @exprs do
      assert {:ok, %SqlParser.Document{} = doc} =
               SqlParser.Parse.run("SELECT c as b from a WHERE #{expr}")

      assert %SqlParser.Document{
               statements: [
                 %SqlParser.Query{
                   body: %SqlParser.Select{
                     selection: selection
                   }
                 }
               ]
             } = doc

      assert expected_selection == selection
    end
  end
end
