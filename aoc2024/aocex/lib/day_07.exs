defmodule AOCDay07 do
  def part1(target, [result]), do: target == result

  def part1(target, [a, b | tail]) do
    add = a + b
    mul = a * b

    part1(target, [add | tail]) || part1(target, [mul | tail])
  end

  def part2(target, [result]), do: target == result

  def part2(target, [a, b | tail]) do
    add = a + b
    mul = a * b
    con = concatenate(a, b)

    part2(target, [add | tail]) || part2(target, [mul | tail]) || part2(target, [con | tail])
  end

  defp concatenate(a, b), do: String.to_integer(Integer.to_string(a) <> Integer.to_string(b))
end

inputs =
  File.stream!("./inputs/day_07.in", :line)
  |> Stream.map(&String.trim/1)
  |> Stream.map(&String.split(&1, ": "))
  |> Stream.map(fn [test_value, nums] ->
    nums = nums |> String.split(" ") |> Enum.map(&String.to_integer/1)
    {String.to_integer(test_value), nums}
  end)
  |> Enum.to_list()

part1 =
  inputs
  |> Enum.filter(fn {value, nums} -> AOCDay07.part1(value, nums) end)
  |> Enum.map(fn {value, _n} -> value end)
  |> Enum.sum()

IO.puts(part1)

part2 =
  inputs
  |> Enum.filter(fn {value, nums} -> AOCDay07.part2(value, nums) end)
  |> Enum.map(fn {value, _n} -> value end)
  |> Enum.sum()

IO.puts(part2)
