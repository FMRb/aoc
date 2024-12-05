defmodule AdventOfCode.Day02 do
  def is_safe_decreasing?([]), do: true
  def is_safe_decreasing?([[a, b] | tail]) do
    a - b <= 3 && a - b > 0 && is_safe_decreasing?(tail)
  end

  def is_safe_increasing?([]), do: true
  def is_safe_increasing?([[a, b] | tail]) do
    b - a <= 3 && b - a > 0 && is_safe_increasing?(tail)
  end

  def is_safe(report) do
    levels = Enum.chunk_every(report, 2, 1, :discard)

    case levels do
      [] -> true
      [[a, b] | tail] when a > b and a - b <= 3 -> is_safe_decreasing?(tail)
      [[a, b] | tail] when a < b and b - a <= 3 -> is_safe_increasing?(tail)
      _ -> false
    end
  end

  def is_safe_dampener(report) do
    rng = 0..(length(report))
    is_safe(report) || Enum.any?(Enum.map(rng, fn i -> is_safe(List.delete_at(report, i))end))
  end
end
reports = File.stream!("./inputs/day_02.in", :line)
|> Stream.map(&String.trim/1)
|> Stream.filter(&(byte_size(&1) > 0))
|> Stream.map(&String.split(&1, " ", trim: true))
|> Stream.map(fn l -> Enum.map(l, &String.to_integer/1) end)
|> Enum.to_list()

Enum.map(reports, fn report -> AdventOfCode.Day02.is_safe(report) end)
|> Enum.filter(&(&1))
|> Enum.count()
|> IO.inspect(label: "Part 1")

Enum.map(reports, fn report -> AdventOfCode.Day02.is_safe_dampener(report) end)
|> Enum.filter(&(&1))
|> Enum.count()
|> IO.inspect(label: "Part 2")
