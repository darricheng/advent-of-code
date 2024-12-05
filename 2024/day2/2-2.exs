defmodule AdventOfCode.Day2_2 do
  def run do
    IO.puts("Running 2-2")

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
    |> Enum.map(fn level ->
      convert_to_differences(level)
    end)
    |> Enum.reduce(0, &(check_safety(&1) + &2))
  end

  defp convert_to_differences(report, acc \\ {[], %{plus: 0, minus: 0, same: 0, over_sized: 0}})
  defp convert_to_differences([_ | []], acc), do: acc

  defp convert_to_differences(
         [first | second_onwards],
         {relations, freq}
       ) do
    [second | _] = second_onwards

    diff = first - second

    freq =
      cond do
        diff == 0 ->
          Map.update!(freq, :same, &(&1 + 1))

        diff > 3 ->
          freq |> Map.update!(:plus, &(&1 + 1)) |> Map.update!(:over_sized, &(&1 + 1))

        diff < -3 ->
          freq |> Map.update!(:minus, &(&1 + 1)) |> Map.update!(:over_sized, &(&1 + 1))

        diff > 0 ->
          Map.update!(freq, :plus, &(&1 + 1))

        diff < 0 ->
          Map.update!(freq, :minus, &(&1 + 1))
      end

    convert_to_differences(second_onwards, {[diff | relations], freq})
  end

  # safe without removing values
  defp check_safety({_, %{plus: _, minus: 0, same: 0, over_sized: 0}}), do: 1
  defp check_safety({_, %{plus: 0, minus: _, same: 0, over_sized: 0}}), do: 1

  # only one pair of same values, the rest are all same direction
  defp check_safety({_, %{plus: _, minus: 0, same: 1, over_sized: 0}}), do: 1
  defp check_safety({_, %{plus: 0, minus: _, same: 1, over_sized: 0}}), do: 1

  # only one size error, safe only if error at the end
  defp check_safety({relations, %{plus: _, minus: 0, same: 0, over_sized: 1}}) do
    over_sized_tail_check(relations, true)
  end

  defp check_safety({relations, %{plus: 0, minus: _, same: 0, over_sized: 1}}) do
    over_sized_tail_check(relations, true)
  end

  defp check_safety({relations, %{plus: _, minus: 1, same: 0, over_sized: over_sized}}) do
    if over_sized > 1 do
      0
    else
      # 1. locate the odd relation
      odd_value_index = Enum.find_index(relations, &(&1 < 0))
      {odd_value, rest_of_relations} = List.pop_at(relations, odd_value_index)

      # 2. squash together with previous relation (odd_value_index - 1 of rest_of_relations), then check all values are 1 to 3
      squashed_with_left =
        if odd_value_index == 0 do
          [_ | tail] = relations
          tail
        else
          List.update_at(rest_of_relations, odd_value_index - 1, &(&1 + odd_value))
        end

      # 3. squash together with next relation (odd_value_index of rest_of_relations), then check all values are 1 to 3
      squashed_with_right =
        if odd_value_index == length(relations) - 1 do
          Enum.drop(relations, -1)
        else
          List.update_at(rest_of_relations, odd_value_index, &(&1 + odd_value))
        end

      if(
        Enum.all?(squashed_with_left, &(&1 >= 1 && &1 <= 3)) ||
          Enum.all?(squashed_with_right, &(&1 >= 1 && &1 <= 3))
      ) do
        1
      else
        0
      end
    end
  end

  defp check_safety({relations, %{plus: 1, minus: _, same: 0, over_sized: over_sized}}) do
    if over_sized > 1 do
      0
    else
      # 1. locate the odd relation
      odd_value_index = Enum.find_index(relations, &(&1 > 0))
      {odd_value, rest_of_relations} = List.pop_at(relations, odd_value_index)

      # 2. squash together with previous relation (odd_value_index - 1 of rest_of_relations), then check all values are 1 to 3
      squashed_with_left =
        if odd_value_index == 0 do
          [_ | tail] = relations
          tail
        else
          List.update_at(rest_of_relations, odd_value_index - 1, &(&1 + odd_value))
        end

      # 3. squash together with next relation (odd_value_index of rest_of_relations), then check all values are 1 to 3
      squashed_with_right =
        if odd_value_index == length(relations) - 1 do
          Enum.drop(relations, -1)
        else
          List.update_at(rest_of_relations, odd_value_index, &(&1 + odd_value))
        end

      if(
        Enum.all?(squashed_with_left, &(&1 <= -1 && &1 >= -3)) ||
          Enum.all?(squashed_with_right, &(&1 <= -1 && &1 >= -3))
      ) do
        1
      else
        0
      end
    end
  end

  # the rest should be unsafe
  defp check_safety(unsafe) do
    unsafe |> IO.inspect(label: :unsafe)
    0
  end

  # if the over sized value is the first or last one, it is safe 
  defp over_sized_tail_check(relations, first_value? \\ false)

  # if reach last value, it's the only over sized one
  defp over_sized_tail_check([last | []], _), do: 1

  # check first value
  defp over_sized_tail_check([head | tail], true) do
    cond do
      head > 3 -> 1
      head < -3 -> 1
      true -> over_sized_tail_check(tail)
    end
  end

  # other mid values
  defp over_sized_tail_check([head | tail], false) do
    cond do
      head > 3 -> 0
      head < -3 -> 0
      true -> over_sized_tail_check(tail)
    end
  end
end

AdventOfCode.Day2_2.run() |> IO.inspect()
