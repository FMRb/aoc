defmodule AdventOfCode.Day02Test do
  use ExUnit.Case
  doctest AdventOfCode.Day02

  @tag :skip
  test "part1" do
    input_path = "inputs/day02_sample.in"
    assert AdventOfCode.Day02.part1(input_path) == 8
  end

  @tag :skip
  test "part2" do
    input_path = "inputs/day02_sample.in"
    assert AdventOfCode.Day02.part2(input_path) == 2286
  end
end
