
def part1():
    result = 0
    for line in open(0):
        win_num, have_num = line.strip().split(": ")[1].split(" | ")

        win_num = list(map(int,win_num.split()))
        have_num = list(map(int, have_num.split()))

        points = 0
        for n in win_num:
            if n in have_num:
                points += 1

        result += int(2**(points-1))

    print(result)

def part2():
    result = 0
    sc = {}
    for index, line in enumerate(open(0)):
        win_num, have_num = line.strip().split(": ")[1].split(" | ")

        win_num = list(map(int,win_num.split()))
        have_num = list(map(int, have_num.split()))
        sc.setdefault(index, 0)

        sc[index] = sc[index] + 1

        matches = 0
        for n in win_num:
            if n in have_num:
                matches += 1
                if sc.get(index+matches) != None:
                    sc[index+matches] = sc.get(index+matches) + sc[index]
                else:
                    sc.setdefault(index+matches, sc[index])

    print(sum(sc.values()))

if __name__ == "__main__":
    # part1()
    part2()
