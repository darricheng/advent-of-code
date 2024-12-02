IO.puts("Running [[DAY]]-[[PART]]")

path = "./day[[DAY]]/data.txt"

data =
  case File.read(path) do
    {:ok, data} ->
      data

    {:error, reason} ->
      IO.puts("File read failed. Reason: #{reason}")
      System.halt(0)
  end

data |> IO.inspect()
