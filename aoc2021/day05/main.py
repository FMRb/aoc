# format x1,y1 -> x2,y2

def parse(data):
    segments = []
    for line in data:
        segment = []
        d = line.split(' ')  # ['x1,y1','->', 'x2,y2']
        x1, y1 = int(d[0].split(',')[0]), int(d[0].split(',')[1])
        x2, y2 = int(d[2].split(',')[0]), int(d[2].split(',')[1])
        segment.append((x1, y1))
        segment.append((x2, y2))
        segments.append(segment)
    return segments


def part1(data):
    segments = parse(data)
    diagram = []
    size = 1000
    for _ in range(size):
        for _ in range(size):
            diagram.append(0)

    for segment in segments:
        (x1, y1) = segment[0]
        (x2, y2) = segment[1]
        if x1 == x2:
            ymax = int(max(y1, y2))
            ymin = int(min(y1, y2))
            for i in range(ymax - ymin + 1):
                diagram[((i+ymin)*size)+x1] += 1
        if y1 == y2:
            xmax = int(max(x1, x2))
            xmin = int(min(x1, x2))
            for i in range(xmax - xmin + 1):
                diagram[(y1*size)+(i+xmin)] += 1

    print(len(list(filter(lambda c: c > 1, diagram))))
    return


def part2(data):
    segments = parse(data)
    diagram = []
    size = 1000
    for _ in range(size):
        for _ in range(size):
            diagram.append(0)

    for segment in segments:
        (x1, y1) = segment[0]
        (x2, y2) = segment[1]
        if x1 == x2:
            ymax = int(max(y1, y2))
            ymin = int(min(y1, y2))
            for i in range(ymax - ymin + 1):
                diagram[((i+ymin)*size)+x1] += 1
            continue
        if y1 == y2:
            xmax = int(max(x1, x2))
            xmin = int(min(x1, x2))
            for i in range(xmax - xmin + 1):
                diagram[(y1*size)+(i+xmin)] += 1
            continue
        # diagonal
        assert abs(x1 - x2) == abs(y1 - y2)
        x_pos = x1 < x2
        y_pos = y1 < y2
        i = 0
        j = 0
        while x1 + i != x2:
            diagram[((j+y1)*size)+(i+x1)] += 1
            i = i + 1 if x_pos else i - 1
            j = j + 1 if y_pos else j - 1
        diagram[((j+y1)*size)+(i+x1)] += 1

    print(len(list(filter(lambda c: c > 1, diagram))))
    return


if __name__ == '__main__':
    data = [x.strip() for x in open('data.in').readlines()]
    # part1(data)
    part2(data)
