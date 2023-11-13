from pathlib import Path
import sys

def parse(puzzle_input):
    return [[int(nu) for nu in n.split("\n")] for n in puzzle_input.split("\n\n")]

def part1(input):
    """
    Input: Array
        Example:
        [[10,20,30], [20], [10, 40]]
    """
    return max([sum(elf) for elf in input])


def part2(input):
    return sum(sorted([sum(elf) for elf in input], reverse=True)[:3])

if __name__ == "__main__":
    for path in sys.argv[1:]:
        print(f"\n{path}:")
        puzzle_input = Path(path).read_text().strip()
        
        puzzle_input = parse(puzzle_input)
        print(f"Part 1:{part1(puzzle_input)}")
        print(f"Part 2:{part2(puzzle_input)}")
        
