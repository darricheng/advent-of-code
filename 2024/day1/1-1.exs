defmodule AdventOfCode.Day1_1 do
  def run do
    IO.puts("Running 1-1")

    path = "./day1/data.txt"

    data =
      case File.read(path) do
        {:ok, data} ->
          data

        {:error, reason} ->
          IO.puts("File read failed. Reason: #{reason}")
          System.halt(0)
      end

    [left, right] =
      data
      |> IO.inspect()
      |> String.trim()
      |> String.split("\n")
      |> Enum.map(fn str ->
        String.split(str)
      end)
      |> Enum.zip()
      |> Enum.map(fn tuple ->
        Tuple.to_list(tuple) |> Enum.sort() |> Enum.map(&String.to_integer(&1))
      end)
      |> IO.inspect()

    IO.inspect([left, right])

    calc_diff(left, right)
  end

  defp calc_diff(left, right) do
    [left_head | left_tail] = left
    [right_head | right_tail] = right

    cond do
      left_tail == [] ->
        abs(left_head - right_head)

      true ->
        abs(left_head - right_head) + calc_diff(left_tail, right_tail)
    end
  end
end

AdventOfCode.Day1_1.run() |> IO.inspect(label: :result)
