defmodule AdventOfCode.Day01 do
  def part1(path) do
    read(path)
      |> calibrate
  end

  def part2(path) do
    read(path)
    |> compute_line
  end

  defp read(path) do
    {:ok, file} = File.open(path, [:read, :utf8])
    lines = IO.read(file, :eof)
    String.split(lines, "\n")
  end

  defp calibrate([line | rest]) do
    filtered = line
    |> String.to_charlist
    |> Enum.filter(fn x -> x in ?0..?9 end)

    result = [List.first(filtered), List.last(filtered)]
    |> List.to_string()
    |> String.to_integer()
    result + calibrate(rest)
  end
  defp calibrate([]), do: 0

  defp compute_line([line | rest]) do
    converted = convert_to_digit(line)

    result = [List.first(converted), List.last(converted)]
    |> List.to_string()
    |> String.to_integer()

    result + compute_line(rest)
  end
  defp compute_line([]), do: 0

  defp convert_to_digit(<<"one", _rest::binary>> = str), do: ["1" | convert_to_digit(tail_str(str))]
  defp convert_to_digit(<<"two", _rest::binary>> = str), do: ["2" | convert_to_digit(tail_str(str))]
  defp convert_to_digit(<<"three", _rest::binary>> = str), do: ["3" | convert_to_digit(tail_str(str))]
  defp convert_to_digit(<<"four", _rest::binary>> = str), do: ["4" | convert_to_digit(tail_str(str))]
  defp convert_to_digit(<<"five", _rest::binary>> = str), do: ["5" | convert_to_digit(tail_str(str))]
  defp convert_to_digit(<<"six", _rest::binary>> = str), do: ["6" | convert_to_digit(tail_str(str))]
  defp convert_to_digit(<<"seven", _rest::binary>> = str), do: ["7" | convert_to_digit(tail_str(str))]
  defp convert_to_digit(<<"eight", _rest::binary>> = str), do: ["8" | convert_to_digit(tail_str(str))]
  defp convert_to_digit(<<"nine", _rest::binary>> = str), do: ["9" | convert_to_digit(tail_str(str))]
  defp convert_to_digit(<<"1", rest::binary>>), do: ["1" | convert_to_digit(rest)]
  defp convert_to_digit(<<"2", rest::binary>>), do: ["2" | convert_to_digit(rest)]
  defp convert_to_digit(<<"3", rest::binary>>), do: ["3" | convert_to_digit(rest)]
  defp convert_to_digit(<<"4", rest::binary>>), do: ["4" | convert_to_digit(rest)]
  defp convert_to_digit(<<"5", rest::binary>>), do: ["5" | convert_to_digit(rest)]
  defp convert_to_digit(<<"6", rest::binary>>), do: ["6" | convert_to_digit(rest)]
  defp convert_to_digit(<<"7", rest::binary>>), do: ["7" | convert_to_digit(rest)]
  defp convert_to_digit(<<"8", rest::binary>>), do: ["8" | convert_to_digit(rest)]
  defp convert_to_digit(<<"9", rest::binary>>), do: ["9" | convert_to_digit(rest)]
  defp convert_to_digit(<<_, rest::binary>>), do: convert_to_digit(rest)
  defp convert_to_digit(<<>>), do: []

  defp tail_str(<<_, rest::binary>>), do: rest
end
