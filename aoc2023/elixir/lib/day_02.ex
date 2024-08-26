defmodule AdventOfCode.Day02 do
  def part1(path) do
    read(path)
    |> read_line
    |> Enum.sum()
  end

  def part2(path) do
    read(path)
    |> read_line2
    |> Enum.sum()
  end

  #  Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
  defp read(path) do
    {:ok, file} = File.open(path, [:read, :utf8])
    lines = IO.read(file, :eof)
    String.split(lines, "\n")
  end

  defp read_line([line | rest]) do
    [game, cubes] = String.split(line, ":")
    [_, id] = String.split(game, " ")

    cubes =
      String.split(cubes, ";")
      |> Enum.map(fn set ->
        Enum.map(
          String.split(set, ","),
          fn c -> cube(String.split(c)) end
        )
      end)

    if valid_limits?(cubes) do
      [String.to_integer(id) | read_line(rest)]
    else
      read_line(rest)
    end
  end

  defp read_line([]), do: []

  defp read_line2([line | rest]) do
    [game, cubes] = String.split(line, ":")
    [_, _id] = String.split(game, " ")

    cubes =
      String.split(cubes, ";")
      |> Enum.map(fn set ->
        map_cube(String.split(set, ","), %{red: 0, blue: 0, green: 0})
      end)

    result = compute_fewest_cubes(cubes, %{red: 0, green: 0, blue: 0})

    [result.red * result.green * result.blue | read_line2(rest)]
  end

  defp read_line2([]), do: []

  defp cube([digit, "blue"]), do: {:blue, String.to_integer(digit)}
  defp cube([digit, "red"]), do: {:red, String.to_integer(digit)}
  defp cube([digit, "green"]), do: {:green, String.to_integer(digit)}

  defp map_cube([head | rest], state) do
    [num, color] = String.split(head)
    num = String.to_integer(num)

    case color do
      "red" -> map_cube(rest, %{state | red: num})
      "green" -> map_cube(rest, %{state | green: num})
      "blue" -> map_cube(rest, %{state | blue: num})
    end
  end

  defp map_cube([], state), do: state

  defp valid_limits?(cubes) do
    normalise =
      Enum.map(cubes, fn set ->
        Enum.map(set, fn c ->
          case c do
            {:blue, value} when value > 14 -> 1
            {:blue, _value} -> 0
            {:red, value} when value > 12 -> 1
            {:red, _value} -> 0
            {:green, value} when value > 13 -> 1
            {:green, _value} -> 0
          end
        end)
      end)

    result =
      Enum.reduce(normalise, 0, fn x, acc ->
        v = Enum.sum(x)
        v + acc
      end)

    result == 0
  end

  defp compute_fewest_cubes([%{red: red, green: green, blue: blue} | rest], result) do
    result = if result.red < red, do: %{result | red: red}, else: result
    result = if result.green < green, do: %{result | green: green}, else: result
    result = if result.blue < blue, do: %{result | blue: blue}, else: result
    compute_fewest_cubes(rest, result)
  end

  defp compute_fewest_cubes([], result), do: result
end
