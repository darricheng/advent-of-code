defmodule AdventOfCode.Day2_1 do
  def run do
    IO.puts("Running 2-1")

    path = "./day2/data.txt"

    data =
      case File.read(path) do
        {:ok, data} ->
          data |> String.trim()

        {:error, reason} ->
          IO.puts("File read failed. Reason: #{reason}")
          System.halt(0)
      end

    reports =
      data
      |> String.split("\n")
      |> Enum.map(&String.split(&1))
      |> Enum.map(fn level ->
        level |> Enum.map(&String.to_integer(&1))
      end)
      |> IO.inspect()

    reports
    |> Enum.reduce(0, fn level, acc ->
      [first | second_onwards] = level
      [second | _] = second_onwards

      preset =
        case first - second < 0 do
          true -> :dsc
          false -> :asc
        end

      res = check(preset, second_onwards)

      IO.inspect(level: level, res: res)

      abs_val = abs(first - second)

      case abs_val >= 1 && abs_val <= 3 do
        true ->
          case res do
            true -> acc + 1
            false -> acc
          end

        false ->
          acc
      end
    end)
  end

  defp check(_, [_ | []]), do: true

  defp check(:asc, [first | second_onwards]) do
    [second | _] = second_onwards

    abs_val = abs(first - second)

    case abs_val >= 1 && abs_val <= 3 do
      true ->
        case second - first < 0 do
          true -> check(:asc, second_onwards)
          false -> false
        end

      false ->
        false
    end
  end

  defp check(:dsc, [first | second_onwards]) do
    [second | _] = second_onwards

    abs_val = abs(first - second)

    case abs_val >= 1 && abs_val <= 3 do
      true ->
        case first - second < 0 do
          true -> check(:dsc, second_onwards)
          false -> false
        end

      false ->
        false
    end
  end
end

AdventOfCode.Day2_1.run() |> IO.inspect()
