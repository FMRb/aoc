defmodule AdventOfCode.Day01Test do
  use ExUnit.Case
  doctest AdventOfCode.Day01

  @tag :skip
  test "part1" do
    input_path = "inputs/day01_sample.txt"
    assert AdventOfCode.Day01.part1(input_path) == 142
  end

  @tag :skip
  test "part2" do
    input_path = "inputs/day01_p2_sample.in"
    assert AdventOfCode.Day01.part2(input_path) == 281
  end
end
