
filename = './input.in'


def part2():
    horizontal = 0
    depth = 0
    aim = 0

    with open(filename, 'r') as file:
        lines = file.readlines()
        for line in lines:
            step = line.split()
            value = int(step[1])
            if step[0] == 'forward':
                horizontal += value
                depth += aim * value
            if step[0] == 'down':
                aim += value
            if step[0] == 'up':
                aim -= value
    print(horizontal*depth)


def main():
    horizontal = 0
    depth = 0

    with open(filename, 'r') as file:
        lines = file.readlines()
        for line in lines:
            step = line.split()
            value = int(step[1])
            if step[0] == 'forward':
                horizontal += value
            if step[0] == 'down':
                depth += value
            if step[0] == 'up':
                depth -= value
    print(horizontal*depth)


if __name__ == '__main__':
    part2()
