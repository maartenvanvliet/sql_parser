defmodule SqlParser.Parse do
  @moduledoc false
  use Rustler,
    otp_app: :sql_parser,
    crate: :sqlparser_parse

  # When loading a NIF module, dummy clauses for all NIF function are required.
  # NIF dummies usually just error out when called when the NIF is not loaded, as that should never normally happen.
  def parse_statements(_sql, _dialect, _recursion_limit), do: :erlang.nif_error(:nif_not_loaded)

  def to_sql(_ast, _dialect), do: :erlang.nif_error(:nif_not_loaded)
end
