from collections import deque

adjacents = [(1,0), (-1,0), (0,1), (0,-1)]
def part1(garden):
    start_point = None
    for y, row in enumerate(garden):
        for x, ch in enumerate(row):
            if ch == "S":
                start_point = (x, y)

    assert start_point != None

    positions = [start_point]

    for _ in range(64):
        new_positions = []
        while positions:
            x, y = positions.pop()
            for dx, dy in adjacents:
                if x+dx < 0 or x+dx > len(garden[0]) or y+dy < 0 or y+dy > len(garden):
                    continue
                if garden[y+dy][x+dx] == ".":
                    new_positions.append((x+dx, y+dy))
        positions = list(set(new_positions))

    # for y, row in enumerate(garden):
    #     for x, ch in enumerate(row):
    #         if (x, y) in positions:
    #             print("O", end="")
    #         else:
    #             print(ch, end="")
    #     print()
    # 

    # +1 for starting position
    print(len(positions) + 1)


def part2(garden):
    start_point = None
    for y, row in enumerate(garden):
        for x, ch in enumerate(row):
            if ch == "S":
                start_point = (x, y)

    assert start_point != None
    assert len(garden) == len(garden[0])

    size = len(garden)


    cache = {}
    print(len(garden[0]), len(garden))
    for i in [0, 1, 2, 3, 4, 5, 6 , 7, 8, 9 , 10]:
        positions = [start_point]
        for _ in range(i):
            new_positions = []
            while positions:
                x, y = positions.pop()

                for dx, dy in adjacents:
                    coord = ((x+dx + len(garden[0])) % len(garden[0]), (y+dy + len(garden)) % len(garden))

                    new_tile = garden[coord[1]][coord[0]]  
                    if new_tile == "." or new_tile == "S":
                        new_positions.append((x+dx, y+dy))

            positions = list(set(new_positions))
        print(f"{i}: {len(positions)}")

    #print("Part 2: ", len(positions) + 1)


if __name__ == "__main__":
    garden = []
    for line in open(0).read().strip().splitlines():
        garden.append(line)
    
    #part1(garden)
    part2(garden)
    
