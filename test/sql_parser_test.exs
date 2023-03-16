defmodule SqlParserTest do
  use ExUnit.Case
  doctest SqlParser
  alias SqlParser.Expr
  alias SqlParser.Ident

  # test "to_sql" do
  #   # WHERE b.a = d
  #   sql = "SELECT * FROM a JOIN b on a.id = b.a_id WHERE b.a = d"
  #   {:ok, statements} = SqlParser.parse(sql) |> IO.inspect
  #   assert {:ok, sql} == SqlParser.to_sql(%SqlParser.Document{statements: statements} , dialect: :postgres)
  # end
  test "recursion limit" do
    assert {:error, "sql parser error: recursion limit exceeded"} =
             SqlParser.parse("SELECT * FROM a WHERE b.a = c", recursion_limit: 1)
  end

  test "simple query" do
    assert {:ok, [query]} = SqlParser.parse("SELECT * FROM a WHERE b.a = c")

    assert %SqlParser.Query{
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
           } = query
  end

  test "update query" do
    assert {:ok, [:not_implemented]} ==
             SqlParser.parse("UPDATE foo SET bar = 1")
  end

  test "group query" do
    assert {:ok, [_query]} = SqlParser.parse("SELECT * FROM a group by e")
  end

  test "order by query" do
    assert {:ok,
            [
              %SqlParser.Query{
                body: %SqlParser.Select{
                  distinct: false,
                  from: [
                    %SqlParser.TableWithJoins{
                      joins: [],
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
                      value: %Ident{quote_style: nil, value: "f"}
                    },
                    asc: nil,
                    nulls_first: nil
                  }
                ],
                limit: nil,
                offset: nil
              }
            ]} ==
             SqlParser.parse("SELECT * FROM a ORDER BY f")
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
      assert {:ok, [%SqlParser.Query{} = doc]} =
               SqlParser.parse("SELECT * FROM a WHERE b.a #{op} d")

      assert %SqlParser.Query{
               body: %SqlParser.Select{
                 selection: %Expr{
                   type: :binary_op,
                   value: %SqlParser.BinaryOp{
                     left: _,
                     op: ^name,
                     right: _
                   }
                 }
               }
             } = doc
    end
  end

  test "join work" do
    assert {:ok, [%SqlParser.Query{} = doc]} =
             SqlParser.parse("SELECT * FROM a JOIN b on a.id = b.a_id WHERE b.a = d")

    assert %SqlParser.Query{
             body: %SqlParser.Select{
               from: [
                 %SqlParser.TableWithJoins{
                   relation: %SqlParser.Table{
                     name: %SqlParser.ObjectName{
                       names: [%SqlParser.Ident{quote_style: nil, value: "a"}]
                     }
                   },
                   joins: [
                     %SqlParser.Join{
                       join_operator: %SqlParser.JoinOperator{
                         operator: %{
                           __struct__: SqlParser.JoinConstraint,
                           constraint: %SqlParser.Expr{
                             type: :binary_op,
                             value: %SqlParser.BinaryOp{
                               left: %SqlParser.Expr{
                                 type: :compound_identifier,
                                 value: [
                                   %SqlParser.Ident{quote_style: nil, value: "a"},
                                   %SqlParser.Ident{quote_style: nil, value: "id"}
                                 ]
                               },
                               op: :eq,
                               right: %SqlParser.Expr{
                                 type: :compound_identifier,
                                 value: [
                                   %SqlParser.Ident{quote_style: nil, value: "b"},
                                   %SqlParser.Ident{quote_style: nil, value: "a_id"}
                                 ]
                               }
                             }
                           },
                           kind: :on
                         },
                         kind: :inner
                       },
                       relation: %SqlParser.Table{
                         name: %SqlParser.ObjectName{
                           names: [%SqlParser.Ident{quote_style: nil, value: "b"}]
                         }
                       }
                     }
                   ]
                 }
               ]
             }
           } = doc
  end

  @exprs %{
    %Expr{
      type: :binary_op,
      value: %SqlParser.BinaryOp{
        left: %Expr{type: :identifier, value: %Ident{quote_style: nil, value: "a"}},
        op: :eq,
        right: %Expr{type: :value, value: {:number, "1", false}}
      }
    } => "a = 18446744073709551616.18446744073709551616",
    %Expr{
      type: :is_null,
      value: %Expr{type: :identifier, value: %Ident{quote_style: nil, value: "c"}}
    } => "c IS NULL"
  }

  @tag :skip
  test "expr work" do
    for {expected_selection, expr} <- @exprs do
      assert {:ok, [query]} = SqlParser.parse("SELECT c as b from a WHERE #{expr}")

      assert %SqlParser.Query{
               body: %SqlParser.Select{
                 selection: selection
               }
             } = query

      assert expected_selection == selection
    end
  end
end
