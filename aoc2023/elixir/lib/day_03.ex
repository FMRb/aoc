defmodule AdventOfCode.Day03 do
  def part1(path) do
    result =
      read(path)
      |> Enum.with_index()
      |> parse_schematic()
      |> Enum.map(

    IO.inspect(result, label: "Result")
  end

  def part2(path) do
    read(path)
  end

  defp read(path) do
    {:ok, file} = File.open(path, [:read, :utf8])
    lines = IO.read(file, :eof)
    String.split(lines, "\n", trim: true)
  end

  defp compute_part_numbers(schematics) do
    adjacent = [{-1, -1}, {-1, 0}, {-1, 1}, {0, 1}, {0, -1}, {1, 0}, {1, -1}, {1, 1}]

    Enum.map(schematics)
    |> check_symbols()
  end

  defp parse_schematic([line | rest]) do
    IO.inspect(line, label: "Line")

    {content, idx} = line
    numbers = parse_numbers(content)
    symbols = parse_symbols(content)
    IO.inspect(numbers, label: "numbers")
    IO.inspect(symbols, label: "symbols")
    [{idx, numbers, symbols} | parse_schematic(rest)]
  end

  defp parse_schematic([]), do: []

  defp parse_symbols(str, pos \\ 0)
  defp parse_symbols(<<n, rest::binary>>, pos) when n in ?0..?9, do: parse_symbols(rest, pos + 1)
  defp parse_symbols(<<".", rest::binary>>, pos), do: parse_symbols(rest, pos + 1)
  defp parse_symbols(<<_, rest::binary>>, pos), do: [pos | parse_symbols(rest, pos + 1)]
  defp parse_symbols(<<>>, _pos), do: []

  defp parse_numbers(str, pos \\ 0, num \\ 0, len \\ 0)

  defp parse_numbers(<<n, rest::binary>>, pos, store_num, len) when n in ?0..?9 do
    new_num = store_num * 10 + (n - ?0)
    parse_numbers(rest, pos + 1, new_num, len + 1)
  end

  defp parse_numbers(<<_, rest::binary>>, pos, 0, _len), do: parse_numbers(rest, pos + 1, 0, 0)

  defp parse_numbers(<<_, rest::binary>>, pos, new_num, len),
    do: [{new_num, pos - len, pos - 1} | parse_numbers(rest, pos + 1, 0, 0)]

  defp parse_numbers("", _pos, 0, _len), do: []
  defp parse_numbers("", pos, n, len), do: [{n, pos - len, pos - 1}]
end
