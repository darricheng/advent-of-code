defmodule AdventOfCode.Day1_2 do
  def run do
    IO.puts("Running 1-2")

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

    main_calc(left, right, %{})
  end

  defp main_calc(left, right, map) do
    [left_head | left_tail] = left

    case Map.fetch(map, left_head) do
      {:ok, value} ->
        value + main_calc(left_tail, right, map)

      :error ->
        score = calc_freq(right, left_head) * left_head
        updated_map = Map.put(map, left_head, score)

        cond do
          left_tail == [] -> score
          true -> score + main_calc(left_tail, right, updated_map)
        end
    end
  end

  defp calc_freq(list, key) do
    [head | tail] = list

    cond do
      tail == [] ->
        cond do
          head == key -> 1
          true -> 0
        end

      head == key ->
        1 + calc_freq(tail, key)

      true ->
        calc_freq(tail, key)
    end
  end
end

AdventOfCode.Day1_2.run() |> IO.inspect(label: :result)
