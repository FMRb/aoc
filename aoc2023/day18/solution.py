dirs = {"R": (1, 0), "L": (-1, 0), "U": (0, -1), "D": (0, 1)}

corners = {"DR": 1, "UR": -1}

cache = {}


def is_inside_polygon(point, edges, left_boundary):
    num_intersection = 0
    x, y = point

    if point in cache:
        return cache[point]

    corner = None
    for ra in range(left_boundary, x + 1):
        if (ra, y) in edges and (ra + 1, y) not in edges and not corner:
            num_intersection += 1
        elif (
            (ra, y) in edges
            and (ra, y - 1) in edges
            and (ra - 1, y) not in edges
            and not corner
        ):
            corner = "DR"
            num_intersection += 1
        elif (
            (ra, y) in edges
            and (ra, y + 1) in edges
            and (ra - 1, y) not in edges
            and not corner
        ):
            corner = "UR"
            num_intersection += 1
        elif (ra, y) in edges and (ra + 1, y) not in edges and corner:
            if (ra, y + corners[corner]) not in edges:
                num_intersection += 1
            corner = None

    cache[point] = num_intersection % 2 != 0

    return num_intersection % 2 != 0


def part1(input):
    edges = {}

    current_point = (0, 0)

    for direction, meters, color in input:
        dx, dy = dirs[direction]
        for _ in range(meters):
            current_point = (current_point[0] + dx, current_point[1] + dy)
            edges[current_point] = color

    min_x = min([x for x, _ in edges])
    max_x = max([x for x, _ in edges])
    min_y = min([y for _, y in edges])
    max_y = max([y for _, y in edges])

    print(min_x, max_x, min_y, max_y)

    total = len(edges)
    for y in range(min_y, max_y + 1):
        for x in range(min_x, max_x + 1):
            # if (x, y) in edges:
            #     print("#", end="")
            if (x, y) not in edges and is_inside_polygon((x, y), edges, min_x):
                # print("#", end="")
                total += 1
                # else:
                #    print(".", end="")
        # print()

    print("Part 1: ", total)


hex_to_dir = {"0": "R", "1": "D", "2": "L", "3": "U"}


def part2(input):
    plan = []
    for _, _, color in input:
        meters = int(color[1:-1], 16)
        dir = hex_to_dir[color[-1]]
        plan.append((dir, meters))

    edges = {}
    hlines = {}

    current_point = (0, 0)
    for direction, meters in plan:
        dx, dy = dirs[direction]
        next_point = (
            current_point[0] + (dx * meters),
            current_point[1] + (dy * meters),
        )
        edges[(current_point, direction)] = next_point
        if direction in "RL":
            a = current_point[0] if direction == "R" else next_point[0]
            b = next_point[0] if direction == "R" else current_point[0]
            hlines.setdefault(current_point[1], set()).update([a, b])
        current_point = next_point

    order_h_lines = sorted([y for y in hlines])

    total_area = 0
    vertex_points = sorted(hlines.get(order_h_lines[0]))
    top_edge_y = order_h_lines[0]
    order_h_lines = order_h_lines[1:]
    overlap_lines = [(vertex_points[0], vertex_points[1])]

    for y in order_h_lines:
        height = y - top_edge_y + 1

        for i in range(0, len(vertex_points), 2):
            l_point = vertex_points[i]
            r_point = vertex_points[i + 1]
            width = r_point - l_point + 1
            total_area += height * width

        tmp_vertex_points = sorted(hlines.get(y))
        for nv in tmp_vertex_points:
            if nv in vertex_points:
                vertex_points.remove(nv)
            else:
                vertex_points.append(nv)

        vertex_points.sort()

        n_overlap_lines = []
        for i in range(0, len(vertex_points), 2):
            edge = (vertex_points[i], vertex_points[i + 1])
            n_overlap_lines.append(edge)

            for ol in overlap_lines:
                tmp_a = edge if edge[0] <= ol[0] else ol
                tmp_b = edge if tmp_a == ol else ol
                total_area -= max(
                    0, min(tmp_a[1], tmp_b[1]) - max(tmp_a[0], tmp_b[0]) + 1
                )

        overlap_lines = n_overlap_lines
        top_edge_y = y

    print(total_area)


if __name__ == "__main__":
    plan = []
    for line in open(0).read().strip().splitlines():
        direction, meters, color = line.split()

        meters = int(meters)
        color = color[1:-1]
        plan.append((direction, meters, color))

    # part1(plan)
    part2(plan)
