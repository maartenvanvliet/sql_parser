defmodule SqlParser.MixProject do
  use Mix.Project

  @url "https://github.com/maartenvanvliet/sql_parser"

  def project do
    [
      app: :sql_parser,
      version: "0.1.1",
      elixir: "~> 1.14",
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      description: "Sql parser wrapping sqlparser-rs",
      source_url: @url,
      homepage_url: @url,
      package: [
        maintainers: ["Maarten van Vliet"],
        licenses: ["MIT"],
        links: %{"GitHub" => @url},
        files: ~w(LICENSE README.md lib priv mix.exs .formatter.exs)
      ],
      docs: [
        main: "SqlParser",
        source_url: @url,
        canonical: "http://hexdocs.pm/sql_parser"
      ]
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger]
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:rustler, "~> 0.25"},
      {:ex_doc, "~> 0.29"}
    ]
  end
end
