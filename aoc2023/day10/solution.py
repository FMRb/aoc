from collections import deque

def find_start(grid):
    start_point = None
    for y, row in enumerate(grid):
        for x, ch in enumerate(row):
            if ch == 'S':
                start_point = (x, y)

    if start_point == None:
        raise Exception("No start for the grid")

    x, y = start_point 

    start_pipe = None
    if grid[y - 1][x] == "7" and grid[y][x - 1] == "-":
        start_pipe = "J"
    elif grid[y - 1][x] == "|" and grid[y][x+1] == "-":
        start_pipe = "L"
    elif grid[y][x-1] == "-" and grid[y][x + 1] == "-":
        start_pipe = "-"
    if grid[y+1][x] == "|" and grid[y][x + 1] == "J":
        start_pipe = "F"
    elif grid[y+1][x] == "|" and grid[y][x + 1] == "-":
        start_pipe = "F"
    elif grid[y][x-1] == "-" and grid[y+1][x] == "|":
        start_pipe = "7"
    elif grid[y][x-1] == "F" and grid[y+1][x] == "|":
        start_pipe = "7"
    elif grid[y-1][x] == "|" and grid[y+1][x] == "|":
        start_pipe = "|"
    elif grid[y][x-1] == "-" and grid[y][x+1] == "7":
        start_pipe = "-"
    elif grid[y+1][x] == "J" and grid[y][x+1] == "7":
        start_pipe = "F"

    if start_pipe == None:
        raise Exception("Start pipe cannot be None")

    return (start_point, start_pipe)

mov_map = {
    "|": [(0,1), (0, -1)],
    "-": [(1, 0), (-1, 0)],
    "L": [(0,-1), (1, 0)],
    "J": [(-1, 0), (0, -1)],
    "7": [(0, 1), (-1, 0)],
    "F": [(1, 0), (0, 1)]
}

def part1(grid):
    start_point, start_pipe = find_start(grid)

    pipes = deque()
    mov_a, mov_b = mov_map.get(start_pipe)
    pipes.append((start_point[0] + mov_a[0], start_point[1] + mov_a[1], 1))
    pipes.append((start_point[0] + mov_b[0], start_point[1] + mov_b[1], 1))

    max_level = 0
    visited = {
    }
    visited[start_point] = True
    while pipes:
        x, y, level = pipes.popleft()
        if visited.get((x,y)) != None:
            continue
        if mov_map.get(grid[y][x]):
            mov_a, mov_b = mov_map[grid[y][x]]
            pipes.append((x + mov_a[0], y + mov_a[1], level + 1))
            pipes.append((x + mov_b[0], y + mov_b[1], level + 1))

        visited[(x,y)] = True
        if max_level < level:
            max_level = level

    print("Part 1: ", max_level)


coords = [(1, 0), (0, 1), (-1, 0), (0, -1)]

edges = ["F","J","L","7","S"]
close_edges = {
    "L": "J",
    "F": "7",
}
def is_point_inside_loop(point, grid, loop_map):
    left_point = (0, point[1])

    c_p = left_point

    hits = 0
    last_edge = None
    while c_p != point:
        x, y = c_p
        ch = grid[y][x]
        if loop_map.get(c_p) != None:
            if ch == "|":
                hits += 1
            elif ch != "-" and last_edge == None:
                hits += 1
                last_edge = ch
            elif close_edges.get(last_edge) == ch:
                last_edge = None
                hits +=1
            elif ch != "-":
                last_edge = None

        c_p = (x + 1, y)

    return hits % 2 != 0


def print_grid(loop_map, point_inside_loop):
    for y, row in enumerate(grid):
        for x, ch in enumerate(row):
            if loop_map.get((x,y)) != None:
                print(f"\033[1;31;40m{ch}\033[0;0m", end="")
            elif point_inside_loop.get((x,y)) != None:
                print(f"\033[1;32;40m&\033[0;0m", end="")
            else:
                print(ch, end="")
        print()

def part2(grid):
    start_point, start_pipe = find_start(grid)

    pipes = deque()
    x, y = start_point
    mov_a, mov_b = mov_map.get(start_pipe)
    pipes.append((x + mov_a[0], y + mov_a[1]))
    pipes.append((x + mov_b[0], y + mov_b[1]))

    grid[y][x] = start_pipe
    visited = {}
    visited[start_point] = True
    while pipes:
        x, y = pipes.popleft()
        if visited.get((x,y)) != None:
            continue
        if mov_map.get(grid[y][x]):
            mov_a, mov_b = mov_map[grid[y][x]]
            pipes.append((x + mov_a[0], y + mov_a[1]))
            pipes.append((x + mov_b[0], y + mov_b[1]))

        visited[(x,y)] = True

    no_loop_points = []
    for y, row in enumerate(grid):
        for x, _ in enumerate(row):
            if visited.get((x, y)) == None:
                no_loop_points.append((x,y))


    point_inside_loop = {}
    for point in no_loop_points:
        if is_point_inside_loop(point, grid, visited):
            point_inside_loop[point] = True


    # print_grid(visited, point_inside_loop)

    print("Part 2: ", len(point_inside_loop))



if __name__ == "__main__":
    grid = []
    for r in open(0).read().splitlines():
        row = []
        for chr in [*r]:
            row.append(chr)
        grid.append(row)

    part1(grid)
    part2(grid)

