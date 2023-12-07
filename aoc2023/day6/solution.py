def part1():
    times, distances = open(0).read().splitlines()

    times = list(map(int, times.split(": ")[1].split()))
    distances = list(map(int, distances.split(": ")[1].split()))

    dwts = []
    for time in times:
        dwt = []
        for i in range(time):
            dwt.append(i * (time - i))
        dwts.append(dwt)

    results = []
    for i, distance in enumerate(distances):
        results.append(len(list(filter(lambda a: a > distance, dwts[i]))))

    result = 1
    for r in results:
        result = result * r

    print(result)


def part2():
    times, distances = open(0).read().splitlines()

    time = int("".join(times.split(": ")[1].split()))
    distance = int("".join(distances.split(": ")[1].split()))

    dwt = []
    for i in range(time):
        dwt.append(i * (time - i))

    result = len(list(filter(lambda a: a > distance, dwt)))

    print(result)


if __name__ == "__main__":
    part2()
