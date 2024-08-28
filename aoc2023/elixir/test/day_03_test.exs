defmodule AdventOfCode.Day03Test do
  use ExUnit.Case
  doctest AdventOfCode.Day03

  test "part1" do
    input_path = "inputs/day03_sample.in"
    assert AdventOfCode.Day03.part1(input_path) == 4361
  end

  @tag :skip
  test "part2" do
    input_path = "inputs/day03_sample.in"
    assert AdventOfCode.Day03.part2(input_path) == 2286
  end
end
