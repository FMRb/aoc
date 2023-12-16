from itertools import combinations


def expand_universe(data, times=1):
    tmp_data = []
    for line in data:
        if line.count("#") == 0:
            for _ in range(times):
                tmp_data.append(line[:])
        tmp_data.append(line)

    data = tmp_data

    data = list(zip(*data))

    tmp_data = []
    for line in data:
        if line.count("#") == 0:
            for _ in range(times):
                tmp_data.append(line[:])
        tmp_data.append(line)

    data = list(zip(*tmp_data))

    return data


def expanding(data, is_vertical=False):
    expanding_indeces = []

    if is_vertical:
        data = list(zip(*data))

    for i, row in enumerate(data):
        if row.count("#") == 0:
            expanding_indeces.append(i)
    return expanding_indeces


def part1(data):
    data = expand_universe(data)
    galaxies = []
    for y, row in enumerate(data):
        for x, ch in enumerate(row):
            if ch == "#":
                galaxies.append((x, y))

    galaxies_pairs = list(combinations(galaxies, 2))

    dist = []
    total = 0
    for g_a, g_b in galaxies_pairs:
        d = abs(g_a[0] - g_b[0]) + abs(g_a[1] - g_b[1])
        dist.append((g_a, g_b, d))
        total += d

    print("Part 1: ", total)


def part2(data):
    galaxies = []
    for y, row in enumerate(data):
        for x, ch in enumerate(row):
            if ch == "#":
                galaxies.append((x, y))

    expanding_horizontal = expanding(data)
    expanding_vertical = expanding(data, is_vertical=True)
    galaxies_pairs = list(combinations(galaxies, 2))

    expansion_times = 999999
    total = 0
    for g_a, g_b in galaxies_pairs:
        xa, ya = g_a
        xb, yb = g_b

        for exp_index in expanding_horizontal:
            if g_a[1] > exp_index:
                ya += expansion_times
            if g_b[1] > exp_index:
                yb += expansion_times

        for exp_index in expanding_vertical:
            if g_a[0] > exp_index:
                xa += expansion_times
            if g_b[0] > exp_index:
                xb += expansion_times

        d = abs(xa - xb) + abs(ya - yb)
        total += d

    print("Part 2: ", total)
    pass


if __name__ == "__main__":
    data = []
    for line in open(0).read().strip().splitlines():
        data.append([*line])

    # part1(data)
    part2(data)
