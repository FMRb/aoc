defmodule AOCDay10 do
  def parse_grid(inputs) do
    for {line, y} <- inputs |> Enum.with_index(),
        {n, x} <- line |> Enum.with_index(),
        into: %{},
        do: {{x, y}, n}
  end

  def score_trails(grid) do
    find_trailheads(grid)
    |> Enum.map(fn x ->
      score_trail(x, grid) |> Enum.count()
    end)
    |> Enum.sum()
  end

  def rate_trails(grid) do
    find_trailheads(grid)
    |> Enum.map(fn x ->
      rate_trail(x, grid) |> Enum.count()
    end)
  end

  defp score_trail(pos, grid) do
    value = Map.get(grid, pos, 0)
    nexts = check_next(pos)

    for n_pos <- nexts do
      case Map.get(grid, n_pos, nil) do
        9 when 9 == value + 1 ->
          n_pos

        n when n == value + 1 ->
          score_trail(n_pos, grid)

        _ ->
          nil
      end
    end
    |> List.flatten()
    |> Enum.filter(& &1)
    |> Enum.uniq()
  end

  defp rate_trail(pos, grid) do
    value = Map.get(grid, pos, 0)
    nexts = check_next(pos)

    for n_pos <- nexts do
      case Map.get(grid, n_pos, nil) do
        9 when 9 == value + 1 ->
          n_pos

        n when is_integer(n) and n == value + 1 ->
          rate_trail(n_pos, grid)

        _ ->
          nil
      end
    end
    |> List.flatten()
    |> Enum.filter(& &1)
  end

  defp check_next({x, y}), do: [{x + 1, y}, {x - 1, y}, {x, y + 1}, {x, y - 1}]

  defp find_trailheads(grid) do
    grid
    |> Map.filter(fn {_p, n} -> n == 0 end)
    |> Map.to_list()
    |> Enum.map(fn {p, _v} -> p end)
  end
end

grid =
  File.stream!("./inputs/day_10.in", :line)
  |> Stream.map(&String.trim/1)
  |> Stream.map(&String.graphemes/1)
  |> Stream.map(
    &Enum.map(&1, fn
      "." -> "."
      x -> String.to_integer(x)
    end)
  )
  |> Enum.to_list()
  |> AOCDay10.parse_grid()

# part1 =
#   grid
#   |> AOCDay10.score_trails()

part2 =
  grid
  |> AOCDay10.rate_trails()
  |> Enum.sum()

IO.puts(part2)
