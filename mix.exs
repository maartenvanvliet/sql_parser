defmodule SqlParser.MixProject do
  use Mix.Project

  @url "https://github.com/maartenvanvliet/sql_parser"

  @version "0.2.5"
  def project do
    [
      app: :sql_parser,
      version: @version,
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
        files: ~w(LICENSE README.md lib priv native mix.exs .formatter.exs checksum-*.exs)
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
      {:rustler, ">= 0.0.0", optional: true},
      {:rustler_precompiled, "~> 0.6.1"},
      {:ex_doc, "~> 0.29", only: [:dev, :test]}
    ]
  end
end
