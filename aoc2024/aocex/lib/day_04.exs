defmodule AdventOfCode.Day04 do
  def count_xmax_words(grid) do
    grid
    |> Enum.with_index()
    |> Enum.map(fn {line, i} ->
      line
      |> Enum.with_index()
      |> Enum.map(fn
        {"X", j} ->
          build_xmax_directions(i, j, grid, [
            [-1, 0],
            [1, 0],
            [0, -1],
            [0, 1],
            [-1, -1],
            [-1, 1],
            [1, -1],
            [1, 1]
          ])

        _ ->
          0
      end)
      |> Enum.sum()
    end)
    |> Enum.sum()
  end

  def build_xmax_directions(_i, _j, _grid, []), do: 0

  def build_xmax_directions(i, j, grid, [head | rest]) do
    build_xmax(i, j, grid, head, "X") + build_xmax_directions(i, j, grid, rest)
  end

  def build_xmax(i, j, grid, [di, dj] = delta, word) do
    max_i = length(grid)
    max_j = length(Enum.at(grid, 0))
    new_i = i + di
    new_j = j + dj
    char = Enum.at(Enum.at(grid, new_i, []), new_j, "F")
    new_word = word <> char

    case new_word do
      "XM" when new_i < max_i and new_j < max_j and new_i >= 0 and new_j >= 0 ->
        build_xmax(new_i, new_j, grid, delta, new_word)

      "XMA" when new_i < max_i and new_j < max_j and new_i >= 0 and new_j >= 0 ->
        build_xmax(new_i, new_j, grid, delta, new_word)

      "XMAS" when new_i < max_i and new_j < max_j and new_i >= 0 and new_j >= 0 ->
        1

      _ ->
        0
    end
  end

  def count_x_max_words(grid) do
    grid
    |> Enum.with_index()
    |> Enum.map(fn {line, i} ->
      line
      |> Enum.with_index()
      |> Enum.map(fn
        {"A", j} ->
          check_x_max_words(i, j, grid, [[-1, -1], [1, 1]], [[1, -1], [-1, 1]])

        _ ->
          0
      end)
      |> Enum.sum()
    end)
    |> Enum.sum()
  end

  def check_x_max_words(_i, _j, _grid, []), do: 0

  def check_x_max_words(i, j, grid, [[da_i, da_j], [db_i, db_j]], [[dc_i, dc_j], [dd_i, dd_j]]) do
    char_a = Enum.at(Enum.at(grid, i + da_i, []), j + da_j, "F")
    char_b = Enum.at(Enum.at(grid, i + db_i, []), j + db_j, "F")
    char_c = Enum.at(Enum.at(grid, i + dc_i, []), j + dc_j, "F")
    char_d = Enum.at(Enum.at(grid, i + dd_i, []), j + dd_j, "F")

    combination = char_a <> char_b <> char_c <> char_d

    case combination do
      "MSMS" -> 1
      "MSSM" -> 1
      "SMSM" -> 1
      "SMMS" -> 1
      _ -> 0
    end
  end
end

File.stream!("./inputs/day_04.in", :line)
|> Stream.map(&String.trim/1)
|> Stream.filter(&(byte_size(&1) > 0))
|> Stream.map(&String.split(&1, "", trim: true))
|> Enum.to_list()
|> AdventOfCode.Day04.count_x_max_words()
|> IO.inspect()
