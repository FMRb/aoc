defmodule AOC24D5 do
  def is_correct?(produce, rules_map) do
    p_index =
      produce
      |> Enum.with_index()
      |> Enum.map(&check_order_correct?(&1, produce, rules_map))
      |> Enum.all?()
  end

  def is_correct_sort?(produce, rules_map) do
    p_index =
      produce
      |> Enum.with_index()
      |> Enum.map(&check_order_incorrect?(&1, produce, rules_map))
      |> Enum.any?()
  end

  defp check_order_correct?({n, i}, produce, rules_map) do
    rules = Map.get(rules_map, n, [])

    result =
      Enum.slice(produce, 0..i)
      |> Enum.map(fn t -> Enum.member?(rules, t) end)
      |> Enum.any?()

    not result
  end

  defp check_order_incorrect?({n, i}, produce, rules_map) do
    rules = Map.get(rules_map, n, [])

    result =
      Enum.slice(produce, 0..i)
      |> Enum.map(fn t -> Enum.member?(rules, t) end)
      |> Enum.any?()

    result
  end
end

{rules, produce} =
  File.stream!("./inputs/day_05.in", :line)
  |> Stream.map(&String.split(&1, "\n", trim: true))
  |> Enum.to_list()
  |> Enum.reduce({[], []}, fn
    [], acc ->
      acc

    [str], acc ->
      {rules, produce} = acc

      cond do
        String.contains?(str, "|") ->
          new_rule =
            str
            |> String.split("|")
            |> Enum.map(&String.to_integer/1)
            |> List.to_tuple()

          rules = [new_rule | rules]
          {rules, produce}

        String.contains?(str, ",") ->
          new_produce =
            str
            |> String.split(",")
            |> Enum.map(&String.to_integer/1)

          produce = [new_produce | produce]
          {rules, produce}

        true ->
          {rules, produce}
      end
  end)

rules_map =
  rules
  |> Enum.reduce(%{}, fn {a, b}, acc ->
    {_, acc} =
      Map.get_and_update(acc, a, fn current_value ->
        case current_value do
          nil -> {current_value, [b]}
          _ when is_list(current_value) -> {current_value, [b | current_value]}
          _ -> {current_value, nil}
        end
      end)

    acc
  end)

produce
|> Enum.filter(&AOC24D5.is_correct?(&1, rules_map))
|> Enum.map(fn p ->
  Enum.at(p, floor(length(p) / 2))
end)
|> Enum.sum()
|> IO.inspect(label: "Part 1")

produce
|> Enum.filter(&AOC24D5.is_correct_sort?(&1, rules_map))
|> Enum.map(
  &Enum.sort(
    &1,
    fn a, b ->
      rules = Map.get(rules_map, a, [])
      not Enum.member?(rules, b)
    end
  )
)
|> Enum.map(fn p ->
  Enum.at(p, floor(length(p) / 2))
end)
|> Enum.sum()
|> IO.inspect(label: "Part 2")
