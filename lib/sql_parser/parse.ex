defmodule SqlParser.Parse do
  version = Mix.Project.config()[:version]

  @moduledoc false
  use RustlerPrecompiled,
    otp_app: :sql_parser,
    crate: :sqlparser_parse,
    base_url: "https://github.com/maartenvanvliet/sql_parser/releases/download/v#{version}",
    force_build: System.get_env("RUSTLER_PRECOMPILATION_SQL_PARSER_BUILD") in ["1", "true"],
    version: version

  # When loading a NIF module, dummy clauses for all NIF function are required.
  # NIF dummies usually just error out when called when the NIF is not loaded, as that should never normally happen.
  def parse_statements(_sql, _dialect, _recursion_limit), do: :erlang.nif_error(:nif_not_loaded)

  def to_sql(_ast, _dialect), do: :erlang.nif_error(:nif_not_loaded)
end
