data =
  case File.read(path) do
    {:ok, data} ->
      data

    {:error, reason} ->
      IO.puts("File read failed. Reason: #{reason}")
      System.halt(0)
  end

data |> IO.inspect()
