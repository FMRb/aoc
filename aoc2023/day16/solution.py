def beam(layout, start_ray, start_direction):
    rays = [(start_ray, start_direction)]
    map_ray = {}
    while rays:
        (x, y), (dx, dy) = rays.pop()

        pos = (x + dx, y + dy)
        if (
            pos[0] < 0
            or pos[0] >= len(layout[0])
            or pos[1] < 0
            or pos[1] >= len(layout)
        ):
            continue

        n_ch = layout[y + dy][x + dx]

        if n_ch == ".":
            dir = (dx, dy)
            if map_ray.get(pos) and dir in map_ray.get(pos):
                continue

            rays.append((pos, dir))
            map_ray.setdefault(pos, set()).add(dir)
        elif n_ch in "\\/":
            ndx = -dy if n_ch == "/" else dy
            ndy = -dx if n_ch == "/" else dx
            dir = (ndx, ndy)
            if map_ray.get(pos) and dir in map_ray.get(pos):
                continue

            rays.append((pos, (ndx, ndy)))
            map_ray.setdefault(pos, set()).add(dir)
        elif n_ch in "|-":
            # Moving horizontally
            if dx != 0 and n_ch == "|":
                a_dx = 0
                a_dy = -1
                b_dx = 0
                b_dy = 1

                dir_a = (a_dx, a_dy)
                dir_b = (b_dx, b_dy)

                if not (map_ray.get(pos) and dir_a in map_ray.get(pos)):
                    rays.append((pos, dir_a))
                    map_ray.setdefault(pos, set()).add(dir_a)

                if not (map_ray.get(pos) and dir_b in map_ray.get(pos)):
                    rays.append((pos, dir_b))
                    map_ray.setdefault(pos, set()).add(dir_a)

            elif dy != 0 and n_ch == "-":
                a_dx = -1
                a_dy = 0
                b_dx = 1
                b_dy = 0
                dir_a = (a_dx, a_dy)
                dir_b = (b_dx, b_dy)

                if not (map_ray.get(pos) and dir_a in map_ray.get(pos)):
                    rays.append((pos, dir_a))
                    map_ray.setdefault(pos, set()).add(dir_a)

                if not (map_ray.get(pos) and dir_b in map_ray.get(pos)):
                    rays.append((pos, dir_b))
                    map_ray.setdefault(pos, set()).add(dir_a)

            else:
                dir = (dx, dy)
                if map_ray.get(pos) and dir in map_ray.get(pos):
                    continue
                rays.append((pos, dir))
                map_ray.setdefault(pos, set()).add(dir)

    return map_ray


def part1(layout):
    start_ray = (-1, 0)
    # right (1,0)
    # left (-1,0)
    # top (0, -1)
    # bottom (0, 1)
    start_direction = (1, 0)

    map_ray = beam(layout, start_ray, start_direction)

    # for y in range(len(layout)):
    #     for x in range(len(layout[0])):
    #         if (x, y) in map_ray:
    #             print("#", end="")
    #         else:
    #             print(".", end="")
    #     print()
    # for k in map_ray:
    #     print(f"pos {k} ", map_ray[k])
    print("Part 1: ", len(map_ray))


def part2(layout):
    start_ray = (-1, 0)
    # right (1,0)
    # left (-1,0)
    # top (0, -1)
    # bottom (0, 1)
    start_direction = (1, 0)

    max_energized = 0

    # Vertical
    #   Down
    for x in range(len(layout[0])):
        start_ray = (x, -1)
        start_direction = (0, 1)
        map_ray = beam(layout, start_ray, start_direction)
        max_energized = max(max_energized, len(map_ray))
    #   Up
    for x in range(len(layout[0])):
        start_ray = (x, len(layout))
        start_direction = (0, -1)
        map_ray = beam(layout, start_ray, start_direction)
        max_energized = max(max_energized, len(map_ray))

    # Horizontal
    # Left -> Right
    for y in range(len(layout)):
        start_ray = (-1, y)
        start_direction = (1, 0)
        map_ray = beam(layout, start_ray, start_direction)
        max_energized = max(max_energized, len(map_ray))

    # Right -> Left
    for y in range(len(layout)):
        start_ray = (len(layout[0]), y)
        start_direction = (-1, 0)
        map_ray = beam(layout, start_ray, start_direction)
        max_energized = max(max_energized, len(map_ray))

    print("Part 2: ", max_energized)

    # for y in range(len(layout)):
    #     for x in range(len(layout[0])):
    #         if (x, y) in map_ray:
    #             print("#", end="")
    #         else:
    #             print(".", end="")
    #     print()
    # for k in map_ray:
    #     print(f"pos {k} ", map_ray[k])


if __name__ == "__main__":
    layout = [line for line in open(0).read().splitlines()]
    # part1(layout)
    part2(layout)
