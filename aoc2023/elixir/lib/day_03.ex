defmodule AdventOfCode.Day03 do
  def part1(path) do
    result =
      read(path)
      |> Enum.with_index()
      |> parse_schematic()

    map_numbers =
      Enum.map(result, fn {idx, numbers, _s} ->
        {idx, numbers}
      end)
      |> Enum.into(%{})

    adjacents = [{1, 0}, {1, 1}, {1, -1}, {0, 1}, {0, -1}, {-1, 1}, {-1, 0}, {-1, -1}]

    symbols =
      Enum.flat_map(result, fn {idx, _n, symbols} ->
        Enum.flat_map(adjacents, fn {x, y} ->
          Enum.map(symbols, fn {_s, pos} ->
            {x + pos, y + idx}
          end)
          |> Enum.filter(fn {sx, sy} ->
            sx >= 0 and Map.has_key?(map_numbers, sy)
          end)
        end)
      end)

    total =
      Enum.reduce(Map.keys(map_numbers), [], fn idx, acc ->
        parts =
          Enum.filter(Map.get(map_numbers, idx), fn {_num, first, last} ->
            flag =
              Enum.any?(symbols, fn {x, y} ->
                y == idx and x >= first and x <= last
              end)

            flag
          end)
          |> Enum.map(fn {num, _f, _l} -> num end)

        parts ++ acc
      end)
      |> Enum.sum()

    IO.inspect(total, label: "Result")
    total
  end

  def part2(path) do
    schematics =
      read(path)
      |> Enum.with_index()
      |> parse_schematic()

    map_numbers =
      Enum.map(schematics, fn {idx, numbers, _s} ->
        {idx, numbers}
      end)
      |> Enum.into(%{})

    number_list =
      Enum.reduce(schematics, [], fn {idx, numbers, _s}, acc ->
        acc ++
          Enum.map(numbers, fn {n, first, last} ->
            {n, first, last, idx}
          end)
      end)

    adjacents = [{1, 0}, {1, 1}, {1, -1}, {0, 1}, {0, -1}, {-1, 1}, {-1, 0}, {-1, -1}]

    symbols =
      Enum.reduce(schematics, [], fn {idx, _n, symbols}, acc ->
        gears =
          Enum.filter(symbols, fn {c, _pos} -> c == ?* end)
          |> Enum.map(fn {_c, pos} ->
            Enum.map(adjacents, fn {x, y} ->
              {x + pos, y + idx}
            end)
            |> Enum.filter(fn {sx, sy} ->
              sx >= 0 and Map.has_key?(map_numbers, sy)
            end)
          end)

        gears ++ acc
      end)

    total =
      Enum.map(symbols, fn coords ->
        Enum.reduce(number_list, [], fn {n, first, last, row}, acc ->
          if Enum.any?(coords, fn {x, y} ->
               x >= first and x <= last and y == row
             end) do
            [n | acc]
          else
            acc
          end
        end)
      end)

    total =
      Enum.filter(total, fn hits -> length(hits) == 2 end)
      |> Enum.map(fn hits -> Enum.product(hits) end)
      |> Enum.sum()

    IO.inspect(total, label: "Total")

    ## total =
    ##   Enum.map(symbols, fn coords ->
    ##     Enum.reduce(coords, [], fn {x, y}, acc ->
    ##       parts =
    ##         Enum.filter(Map.get(map_numbers, y, []), fn {_num, first, last} ->
    ##           x >= first and x <= last
    ##         end)
    ##         |> Enum.map(fn {num, _f, _l} -> num end)

    ##       parts ++ acc
    ##     end)
    ##   end)
  end

  defp read(path) do
    {:ok, file} = File.open(path, [:read, :utf8])
    lines = IO.read(file, :eof)
    String.split(lines, "\n", trim: true)
  end

  defp parse_schematic([line | rest]) do
    {content, idx} = line
    numbers = parse_numbers(content)
    symbols = parse_symbols(content)
    [{idx, numbers, symbols} | parse_schematic(rest)]
  end

  defp parse_schematic([]), do: []

  defp parse_symbols(str, pos \\ 0)
  defp parse_symbols(<<n, rest::binary>>, pos) when n in ?0..?9, do: parse_symbols(rest, pos + 1)
  defp parse_symbols(<<".", rest::binary>>, pos), do: parse_symbols(rest, pos + 1)
  defp parse_symbols(<<c, rest::binary>>, pos), do: [{c, pos} | parse_symbols(rest, pos + 1)]
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
