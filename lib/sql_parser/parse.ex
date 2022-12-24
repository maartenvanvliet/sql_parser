defmodule SqlParser.Parse do
  use Rustler,
    otp_app: :sql_parser,
    crate: :sqlparser_parse

  # When loading a NIF module, dummy clauses for all NIF function are required.
  # NIF dummies usually just error out when called when the NIF is not loaded, as that should never normally happen.
  def run(_arg1), do: :erlang.nif_error(:nif_not_loaded)
end
