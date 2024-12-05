{left, right} = File.stream!("./inputs/day_01.in", :line)
|> Stream.map(&String.trim/1)
|> Stream.filter(&(byte_size(&1) > 0))
|> Stream.map(&String.split(&1, "   ", trim: true))
|> Enum.to_list()
|> Enum.reduce({[],[]}, fn [a, b], {left, right} ->
    left = [String.to_integer(a) | left]
    right = [String.to_integer(b) | right]
    {left, right}
  end)

right_map = Enum.reduce(right, %{}, fn n, acc ->
  Map.update(acc, n, 1, &(&1 + 1))
end)

left_sorted = Enum.sort(left)
right_sorted = Enum.sort(right)
Enum.zip(left_sorted, right_sorted)
|> Enum.map(fn {a, b} -> abs(b - a) end)
|> Enum.sum
|> IO.inspect(label: 'Part 1')

Enum.map(left, fn n ->
  n * Map.get(right_map, n, 0)
end)
|> Enum.sum()
|> IO.inspect(label: 'Part 2')
