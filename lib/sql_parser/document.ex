defmodule SqlParser.Document do
  defstruct [:statements]
end

defmodule SqlParser.Query do
  defstruct [:body]
end

defmodule SqlParser.Select do
  defstruct [
    :projection,
    from: []
  ]
end

defmodule SqlParser.Wildcard do
  defstruct []
end
