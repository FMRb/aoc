from functools import reduce
from math import gcd

# Define a function 'lcm' that calculates the least common multiple (LCM) of a list of 'numbers'.
def lcm(numbers):
    # Use the 'reduce' function to apply a lambda function that calculates the LCM for a pair of numbers.
    # The lambda function multiplies two numbers and divides the result by their greatest common divisor (gcd).
    # This process is applied cumulatively to all numbers in the list.
    return reduce((lambda x, y: int(x * y / gcd(x, y))), numbers)

def find_goal(start_node, directions, inst_map):
    steps = 0
    d_i = 0
    node = start_node
    while True:
        d = directions[d_i]
        if d == "L":
            node = inst_map[node][0]
        else:
            node = inst_map[node][1]

        steps += 1
        if node[-1] == 'Z':
            break
        d_i = (d_i + 1) % len(directions)
    return steps

def part1(directions, inst_map):
    node = "AAA"
    goal = "ZZZ"
    d_i = 0

    steps = 0

    while True:
        d = directions[d_i]
        if d == "L":
            node = inst_map[node][0]
        else:
            node = inst_map[node][1]

        steps += 1
        if node == goal:
            break
        d_i = (d_i + 1) % len(directions)

    print("Part 1: ", steps)

def part2(directions, inst_map):
    all_int_end_a = [k for k in inst_map.keys() if k[-1] == 'A']

    results = [find_goal(node, directions,inst_map) for node in all_int_end_a]

    print("Part 2: ", lcm(results))

if __name__ == "__main__":
    directions, *instructions = open(0).read().splitlines()
    instructions = instructions[1:]
    directions = [*directions]

    inst_map = {}
    for instruction in instructions:
        f, t = instruction.split(" = ")
        inst_map[f] = t.replace("(","").replace(")","").split(", ")

    part1(directions, inst_map)
    part2(directions, inst_map)
