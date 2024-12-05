defmodule AdventOfCode.Day03 do
  def parse_multiplications(list) do
    list
    |> Enum.map(&parse_multiplication/1)
    |> Enum.sum()
  end

  def parse_multiplications_two(_, flag\\true)
  def parse_multiplications_two([], _), do: 0
  def parse_multiplications_two([head | tail], flag) do
    case head do
      "mul" <> _rest when flag -> parse_multiplication(head) + parse_multiplications_two(tail, flag)
      "don't" <> _rest -> parse_multiplications_two(tail, false)
      "do" <> _rest -> parse_multiplications_two(tail, true)
      _ -> parse_multiplications_two(tail, flag)
    end
  end


  defp parse_multiplication(item) do
    [_, rest] = String.split(item, "(", parts: 2)
    [a, b] = String.split(rest, ",", parts: 2)
    b = String.trim_trailing(b, ")")
    a = String.to_integer(a)
    b = String.to_integer(b)
    a * b
  end
end

pattern = ~r/(mul\(\d+,\d+\))/
pattern_two = ~r/(mul\(\d+,\d+\))|(don't\(\)|do\(\))/

File.stream!("./inputs/day_03.test", :line)
|> Stream.map(&String.trim/1)
|> Stream.filter(&(byte_size(&1) > 0))
|> Stream.map(fn line -> Regex.scan(pattern, line, capture: :first) end)
|> Stream.map(&List.flatten/1)
|> Stream.map(fn line -> AdventOfCode.Day03.parse_multiplications(line) end)
|> Enum.to_list()
|> Enum.sum()
|> IO.inspect(label: "Part 1")

File.stream!("./inputs/day_03.in", :line)
|> Stream.map(&String.trim/1)
|> Stream.filter(&(byte_size(&1) > 0))
|> Stream.map(fn line -> Regex.scan(pattern_two, line, capture: :first) end)
|> Stream.map(&List.flatten/1)
|> Stream.map(fn line -> AdventOfCode.Day03.parse_multiplications_two(line) end)
|> Enum.to_list()
|> Enum.sum()
|> IO.inspect(label: "Part 2")
