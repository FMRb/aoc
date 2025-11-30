defmodule AOCDay11 do
  def multiple_blinks(stones, 0, _cache), do: stones

  def multiple_blinks(stones, times, cache) do
    multiple_blinks(blink(stones, cache), times - 1, cache)
  end

  def blink([], _c), do: []

  def blink([stone | tail], cache) do
    if Map.has_key?(cache, stone) do
      [Map.get(cache, stone) | blink(tail, cache)] |> List.flatten()
    else
      new_stone = op(stone)
      Map.put(cache, stone, new_stone)
      [new_stone | blink(tail, cache)] |> List.flatten()
    end
  end

  defp op(0), do: [1]

  defp op(n) do
    digits = Integer.digits(n)
    len_digits = length(digits)

    cond do
      rem(len_digits, 2) == 0 ->
        {left, right} = Enum.split(digits, div(len_digits, 2))

        [
          left |> Enum.join() |> String.to_integer(),
          right |> Enum.join() |> String.to_integer()
        ]

      true ->
        [n * 2024]
    end
  end
end

File.stream!("./inputs/day_11.test", :line)
|> Stream.map(&String.trim/1)
|> Stream.map(&String.split/1)
|> Stream.map(&Enum.map(&1, fn x -> String.to_integer(x) end))
|> Enum.to_list()
|> List.flatten()
|> AOCDay11.multiple_blinks(25, %{})
|> Enum.count()
|> IO.inspect()
