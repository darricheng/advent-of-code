defmodule AdventOfCode.Day[[DAY]]_[[PART]] do
  def run do
    IO.puts("Running [[DAY]]-[[PART]]")

    path = "./day[[DAY]]/data.txt"

    data =
      case File.read(path) do
        {:ok, data} ->
          data |> String.trim()

        {:error, reason} ->
          IO.puts("File read failed. Reason: #{reason}")
          System.halt(0)
      end

    data |> IO.inspect()
  end
end

AdventOfCode.Day[[DAY]]_[[PART]].run() |> IO.inspect()
