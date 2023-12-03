import re
import math

re_chars = re.compile("^(?!(\\d|\\.)).*")

check_cords = [(1,0), (0,1), (1,1),(-1,0), (0,-1), (-1,-1), (1, -1), (-1, 1)]
def getValidNumber(m, x, y):
    n = []

    flagValid = False
    while m[x][y].isdigit():
        n.append(m[x][y])
        for cx, cy in check_cords:
            if x+cx < 0 or x+cx >= len(m) or y+cy < 0 or y+cy >= len(m[0]):
                continue
            if re_chars.match(m[x+cx][y+cy]):
                flagValid = True
        y += 1
        if y >= len(m[0]):
            break

    print(f"Output y: {y}")
    return (int("".join(n)), y, flagValid)

def part1():
    result = 0
    m = []
    for line in open(0):
        m.append([*line.strip()])

    x = 0
    y = 0
    while x < len(m) and y < len(m[0]):
        if m[x][y].isdigit():
            num,y, isValid = getValidNumber(m, x, y)
            if isValid:
                result += num
            y = y % len(m[0])
            if y == 0:
                x+= 1
        else:
            y = (y + 1) % len(m[0])
            if y == 0:
                x += 1
    print(result)

visit_stars = {}
def fillValidGear(m, x, y): 
    n = []
    starCoord = None
    while m[x][y].isdigit():
        n.append(m[x][y])
        for cx, cy in check_cords:
            if x+cx < 0 or x+cx >= len(m) or y+cy < 0 or y+cy >= len(m[0]):
                continue
            if m[x+cx][y+cy] == "*":
                starCoord = (x+cx, y+cy)
        y += 1
        if y >= len(m[0]):
            break

    num = int("".join(n))
    if starCoord != None:
        visit_stars.setdefault(starCoord, []).append(num)

    return y

def part2():
    result = 0
    m = []
    for line in open(0):
        m.append([*line.strip()])

    x = 0
    y = 0
    while x < len(m) and y < len(m[0]):
        if m[x][y].isdigit():
            y = fillValidGear(m, x, y)
            y = y % len(m[0])
            if y == 0:
                x+= 1
        else:
            y = (y + 1) % len(m[0])
            if y == 0:
                x += 1
    for n in list(visit_stars.values()):
        if len(n) > 1:
            result += math.prod(n)

    print(result)

if __name__ == '__main__':
    # part1()
    part2()
