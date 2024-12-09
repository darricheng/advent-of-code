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
    |> then(&Regex.scan(~r/mul\(\d+,\d+\)|don't|do/, &1))
    |> List.flatten()
    |> IO.inspect()
    |> Enum.map(fn str ->
      cond do
        str == "don't" or str == "do" ->
          str

        true ->
          Regex.scan(~r/\d+/, str)
          |> List.flatten()
          |> Enum.map(&String.to_integer(&1))
          |> Enum.reduce(&(&1 * &2))
      end
    end)
    |> Enum.reduce({0, true}, fn curr, {acc, add?} ->
      cond do
        curr == "don't" -> {acc, false}
        curr == "do" -> {acc, true}
        add? == true -> {acc + curr, true}
        add? == false -> {acc, false}
      end
    end)
  end
end

AdventOfCode.Day3_1.run() |> IO.inspect()
