defmodule AOCDay06 do
  def parse_grid(inputs) do
    for {line, y} <- inputs |> Enum.with_index(),
        {char, x} <- line |> String.graphemes() |> Enum.with_index(),
        into: %{},
        do: {{y, x}, char}
  end

  def find_and_replace_guard(grid) do
    for(
      {p, c} <- grid,
      do:
        if c == "^" do
          {Map.put(grid, p, "."), p}
        else
          nil
        end
    )
    |> Enum.filter(& &1)
    |> hd()
  end

  defp move({y, x}, {dir, i}) do
    case dir do
      :U -> {y - i, x}
      :D -> {y + i, x}
      :L -> {y, x - i}
      :R -> {y, x + i}
    end
  end

  defp turn(dir) do
    case dir do
      :U -> :R
      :D -> :L
      :L -> :U
      :R -> :D
    end
  end

  def part1(grid, start_pos) do
    prev = %{}
    dir = :U
    next(grid, start_pos, dir, prev)
  end

  def part2(grid, start_pos, options) do
    for {y, x} <- Map.keys(options) do
      cond do
        {y, x} == start_pos ->
          false

        Map.get(grid, {y, x}) == "." ->
          if possible_obstacle(grid, {y, x}, start_pos) do
            {y, x}
          else
            nil
          end

        true ->
          false
      end
    end
  end

  defp next(nil, nil, nil, prev), do: prev

  defp next(grid, pos, dir, prev) do
    n = move(pos, {dir, 1})

    case Map.get(grid, n) do
      "#" -> next(grid, pos, turn(dir), prev)
      "." -> next(grid, n, dir, Map.put(prev, pos, true))
      _ -> next(nil, nil, nil, Map.put(prev, pos, true))
    end
  end

  defp possible_obstacle(grid, p, start_pos) do
    prev = %{}
    is_obstacle_valid(Map.put(grid, p, "#"), start_pos, :U, prev)
  end

  defp is_obstacle_valid(grid, pos, dir, prev) do
    next_pos = move(pos, {dir, 1})
    next_c = Map.get(grid, next_pos)
    has_visited = Map.has_key?(prev, {dir, pos})

    case {has_visited, next_c} do
      {true, _} -> true
      {false, "#"} -> is_obstacle_valid(grid, pos, turn(dir), prev)
      {false, "."} -> is_obstacle_valid(grid, next_pos, dir, Map.put(prev, {dir, pos}, true))
      {_, _} -> false
    end
  end
end

grid =
  File.stream!("./inputs/day_06.in", :line)
  |> Stream.map(&String.trim/1)
  |> Enum.to_list()
  |> AOCDay06.parse_grid()

{grid, pos} = AOCDay06.find_and_replace_guard(grid)
visited = AOCDay06.part1(grid, pos)

obstacles = AOCDay06.part2(grid, pos, visited)

obstacles
|> Enum.filter(& &1)
|> Enum.count()
|> IO.inspect()
