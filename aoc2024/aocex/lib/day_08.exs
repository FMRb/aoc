defmodule AOCDay08 do
  def parse_grid(inputs) do
    for {line, y} <- inputs |> Enum.with_index(),
        {char, x} <- line |> String.graphemes() |> Enum.with_index(),
        into: %{},
        do: {{x, y}, char}
  end

  def part1_antinodes(grid) do
    for {p, c} <- grid do
      case c do
        "." -> nil
        _ -> {p, c}
      end
    end
    |> Enum.filter(& &1)
    |> Enum.reduce(%{}, fn {p, c}, acc ->
      Map.update(acc, c, [p], fn exist_value -> [p | exist_value] end)
    end)
    |> Map.values()
    |> Enum.map(&generate_combinations/1)
    |> Enum.map(&new_antinodes/1)
    |> List.flatten()
    |> Enum.filter(fn {x, y} -> Map.has_key?(grid, {x, y}) end)
  end

  def part2_antinodes(grid) do
    antenaas =
      for {p, c} <- grid do
        case c do
          "." -> nil
          _ -> {p, c}
        end
      end
      |> Enum.filter(& &1)
      |> Enum.reduce(%{}, fn {p, c}, acc ->
        Map.update(acc, c, [p], fn exist_value -> [p | exist_value] end)
      end)

    antinodes =
      antenaas
      |> Map.values()
      |> Enum.map(&generate_combinations/1)
      |> Enum.map(&resonant_antinodes(&1, grid))

    [Map.values(antenaas) | antinodes]
    |> List.flatten()
  end

  defp new_antinodes(antenna) do
    antenna
    |> Enum.map(fn [{ax, ay}, {bx, by}] ->
      dx = abs(ax - bx)
      dy = abs(ay - by)

      cond do
        ax > bx and ay < by -> [{ax + dx, ay - dy}, {bx - dx, by + dy}]
        bx > ax and by < ay -> [{bx + dx, by - dy}, {ax - dx, ay + dy}]
        ax < bx and ay < by -> [{ax - dx, ay - dy}, {bx + dx, by + dy}]
        bx < ax and by < ay -> [{bx - dx, by - dy}, {ax + dx, ay + dy}]
        ax > bx and by == ay -> [{ax + dx, ay}, {bx - dx, by}]
        bx > ax and by == ay -> [{bx + dx, by}, {ax - dx, ay}]
        ax == bx and ay > by -> [{ax, ay + dy}, {bx, by - dy}]
        ax == bx and by > ay -> [{bx, by + dy}, {ax, ay - dy}]
        true -> nil
      end
    end)
  end

  defp resonant_antinodes(antenna, grid) do
    antenna
    |> Enum.map(fn [{ax, ay}, {bx, by}] ->
      dx = abs(ax - bx)
      dy = abs(ay - by)

      cond do
        ax > bx and ay < by ->
          [add_antinode({ax, dx}, {ay, -dy}, grid) | add_antinode({bx, -dx}, {by, dy}, grid)]

        bx > ax and by < ay ->
          [add_antinode({bx, dx}, {by, -dy}, grid) | add_antinode({ax, -dx}, {ay, +dy}, grid)]

        ax < bx and ay < by ->
          [add_antinode({ax, -dx}, {ay, -dy}, grid) | add_antinode({bx, +dx}, {by, +dy}, grid)]

        bx < ax and by < ay ->
          [add_antinode({bx, -dx}, {by, -dy}, grid) | add_antinode({ax, +dx}, {ay, +dy}, grid)]

        ax > bx and by == ay ->
          [add_antinode({ax, dx}, {ay, 0}, grid) | add_antinode({bx, -dx}, {by, 0}, grid)]

        bx > ax and by == ay ->
          [add_antinode({bx, dx}, {by, 0}, grid) | add_antinode({ax, -dx}, {ay, 0}, grid)]

        ax == bx and ay > by ->
          [add_antinode({ax, 0}, {ay, dy}, grid) | add_antinode({bx, 0}, {by, -dy}, grid)]

        ax == bx and by > ay ->
          [add_antinode({bx, 0}, {by, +dy}, grid) | add_antinode({ax, 0}, {ay, -dy}, grid)]

        true ->
          nil
      end
    end)
  end

  defp add_antinode({x, dx}, {y, dy}, grid) do
    new_x = x + dx
    new_y = y + dy

    if Map.has_key?(grid, {new_x, new_y}) do
      [{new_x, new_y} | add_antinode({new_x, dx}, {new_y, dy}, grid)]
    else
      []
    end
  end

  defp generate_combinations(list) do
    for i <- 0..(length(list) - 2),
        j <- (i + 1)..(length(list) - 1) do
      [Enum.at(list, i), Enum.at(list, j)]
    end
  end
end

grid =
  File.stream!("./inputs/day_08.in", :line)
  |> Stream.map(&String.trim/1)
  |> Enum.to_list()
  |> AOCDay08.parse_grid()

part1 =
  grid
  |> AOCDay08.part1_antinodes()
  |> Enum.uniq()
  |> Enum.count()

part2 =
  grid
  |> AOCDay08.part2_antinodes()
  |> Enum.uniq()
  |> Enum.count()

IO.puts(part1)
IO.puts(part2)
