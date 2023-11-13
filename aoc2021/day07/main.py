def part1(data):
    positions = list(map(int, data[0].split(',')))

    cache = {}

    for i in range(len(positions)):
        ref = positions[i]
        if cache.get(ref) != None:
            continue
        other_positions = positions[:i] + positions[i+1:]
        fuel = 0
        for p in other_positions:
            fuel += abs(ref - p)
        cache[ref] = fuel

    print(min(list(cache.values())))


'''
2 to 5

2 -> 3 = 1
3 -> 4 = 2
4 -> 5 = 3



𝑛-th triangular
'''


def part2(data):
    positions = list(map(int, data[0].split(',')))

    fuels = []
    m = max(positions)
    for i in range(m):
        fuel = 0
        for p in positions:
            n = abs(i - p)
            ntrin = int((n*(n+1)) / 2)
            fuel += ntrin
        fuels.append(fuel)

    print(min(fuels))


if __name__ == '__main__':
    data = [x.strip() for x in open('data.in').readlines()]
    # part1(data)
    part2(data)
