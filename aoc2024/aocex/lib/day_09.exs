defmodule AOCDay09 do
  def block_representation([{block, id} | tail]) do
    case block do
      [files, 0] ->
        [{id, files} | block_representation(tail)]

      [files, free_spaces] ->
        [{id, files}, {:free, free_spaces} | block_representation(tail)]

      [files] ->
        [{id, files}]

      _ ->
        []
    end
  end

  def file_compacting(blocks) do
    if has_free_space?(blocks) do
      index =
        Enum.find_index(blocks, fn
          {:free, _} -> true
          {_, _} -> false
        end)

      {:free, spots} = Enum.at(blocks, index)

      {id, times} = List.last(blocks)

      updates =
        cond do
          spots > times -> {[{id, times}, {:free, spots - times}], nil}
          times > spots -> {[{id, spots}], [{id, times - spots}]}
          spots == times -> {[{id, spots}], nil}
          true -> {nil, nil}
        end

      case updates do
        {nil, nil} ->
          nil

        {replace, nil} ->
          (Enum.slice(blocks, 0, index) ++ replace ++ Enum.slice(blocks, (index + 1)..-2//1))
          |> drop_last_free()
          |> file_compacting()

        {replace, update} ->
          (Enum.slice(blocks, 0, index) ++
             replace ++ Enum.slice(blocks, (index + 1)..-2//1) ++ update)
          |> drop_last_free()
          |> file_compacting()
      end
    else
      blocks
    end
  end

  def decrease_id_compacting(blocks, []), do: blocks

  def decrease_id_compacting(blocks, [{id, space} | tail]) do
    index =
      Enum.find_index(blocks, fn
        {:free, spots} -> space <= spots
        {_, _} -> false
      end)

    index_target = Enum.find_index(blocks, fn {i, s} -> id == i and s == space end)

    if index == nil or index >= index_target do
      decrease_id_compacting(blocks, tail)
    else
      {:free, spots} = Enum.at(blocks, index)

      half_blocks = Enum.slice(blocks, (index + 1)..-1//1)
      index_file = Enum.find_index(half_blocks, fn {i, s} -> i == id and s == space end)

      transformed_half =
        if index_file != nil do
          List.replace_at(half_blocks, index_file, {:free, space})
        else
          half_blocks
        end

      new_blocks =
        cond do
          spots == space ->
            Enum.slice(blocks, 0, index) ++
              [{id, space}] ++ transformed_half

          spots > space ->
            Enum.slice(blocks, 0, index) ++
              [{id, space}, {:free, spots - space}] ++ transformed_half

          true ->
            nil
        end

      new_blocks
      |> join_frees()
      |> decrease_id_compacting(tail)
    end
  end

  def checksum(blocks) do
    {_, result} =
      blocks
      |> Enum.reduce({0, 0}, fn
        {:free, times}, {index, result} ->
          {index + times, result}

        {value, times}, {index, result} ->
          new_result =
            for(x <- index..(index + times - 1), do: value * x)
            |> Enum.sum()

          {index + times, result + new_result}
      end)

    result
  end

  defp join_frees([]), do: []

  defp join_frees([{:free, a}, {:free, b} | tail]) do
    [{:free, a + b} | join_frees(tail)]
  end

  defp join_frees([head | tail]) do
    [head | join_frees(tail)]
  end

  defp drop_last_free(blocks) do
    last = List.last(blocks)

    case last do
      {:free, _} -> Enum.slice(blocks, 0..-2//1) |> drop_last_free()
      _ -> blocks
    end
  end

  defp has_free_space?(blocks) do
    Enum.any?(blocks, fn
      {:free, _} -> true
      {_, _} -> false
    end)
  end
end

blocks =
  File.stream!("./inputs/day_09.in", :line)
  |> Stream.map(&String.trim/1)
  |> Enum.to_list()
  |> hd()
  |> String.graphemes()
  |> Enum.map(&String.to_integer/1)
  |> Enum.chunk_every(2, 2)
  |> Enum.with_index()
  |> AOCDay09.block_representation()

# part1 =
#   blocks
#   |> AOCDay09.file_compacting()
#   |> AOCDay09.checksum()
# 
reverse_blocks =
  blocks
  |> Enum.filter(fn
    {:free, _} -> false
    {_, _} -> true
  end)
  |> Enum.reverse()

part2 =
  AOCDay09.decrease_id_compacting(blocks, reverse_blocks)
  |> AOCDay09.checksum()

# IO.puts(part1)
IO.puts(part2)
