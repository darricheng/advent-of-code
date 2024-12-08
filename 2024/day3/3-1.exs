defmodule AdventOfCode.Day3_1 do
  def run do
    IO.puts("Running 3-1")

    path = "./day3/data.txt"

    data =
      case File.read(path) do
        {:ok, data} ->
          data |> String.trim()

        {:error, reason} ->
          IO.puts("File read failed. Reason: #{reason}")
          System.halt(0)
      end

    data
    |> IO.inspect()
    |> then(&Regex.scan(~r/mul\(\d+,\d+\)/, &1))
    |> IO.inspect()
    |> List.flatten()
    |> Enum.map(fn str ->
      Regex.scan(~r/\d+/, str)
      |> List.flatten()
      |> Enum.map(&String.to_integer(&1))
      |> Enum.reduce(&(&1 * &2))
    end)
    |> Enum.reduce(&(&1 + &2))
  end
end

AdventOfCode.Day3_1.run() |> IO.inspect()
