import math


def find_adjacent(heightmap, width, index):
    adjacents = []
    # up
    if index >= width:
        nindex = index - width
        adjacents.append((heightmap[nindex], nindex))
    # down
    if (len(heightmap) - width) > index:
        nindex = index + width
        adjacents.append((heightmap[nindex], nindex))
    # left
    if index % width != 0:
        nindex = index - 1
        adjacents.append((heightmap[nindex], nindex))
    # right
    if index % width != (width - 1):
        nindex = index + 1
        adjacents.append((heightmap[index + 1], nindex))

    return adjacents


def part1(heightmap, row_width):
    result = 0
    for index in range(len(heightmap)):
        adjacents = find_adjacent(heightmap, row_width, index)
        value = heightmap[index]
        if min([x[0] for x in adjacents]) > value:
            result += value + 1
    print(result)


def find_basin(heightmap, width, index, visited):
    size = 1
    visited.add(index)
    adjacents = find_adjacent(heightmap, width, index)
    for (a_value, a_index) in adjacents:
        if a_value != 9 and not a_index in visited:
            nsize, nvisited = find_basin(heightmap, width, a_index, visited)
            size += nsize
            visited.union(nvisited)

    return size, visited


def part2(heightmap, row_width):
    discovered_index = set()
    basins = []
    for index in range(len(heightmap)):
        value = heightmap[index]
        if value == 9 or index in discovered_index:
            discovered_index.add(index)
            continue
        value, visited_index = find_basin(
            heightmap, row_width, index, discovered_index)
        discovered_index.union(visited_index)
        basins.append(value)

    largest_basins = sorted(basins)[-3:]
    print(largest_basins)

    print(math.prod(largest_basins))


if __name__ == '__main__':
    data = [x.strip() for x in open('data.in').readlines()]
    row_width = len(data[0])
    heightmap = [int(n) for line in data for n in line]
    part1(heightmap, row_width)
    part2(heightmap, row_width)
