

def part1():
    colorLimit = {
        "red": 12,
        "blue": 14,
        "green": 13
    }

    validGames = 0
    for index,line in enumerate(open(0)):
        game, cubes = line.strip().split(": ")


        flagValid = True
        for rounds in cubes.split('; '):
            colorCount = {
                "red": 0,
                "blue": 0,
                "green": 0
            }
            for round in rounds.split(', '):
                value, color = round.split()
                colorCount[color] += int(value)
                if colorCount[color] > colorLimit[color]:
                    flagValid = False
        if flagValid:
            validGames += index + 1

    print(validGames)

def part2():
    result = 0
    for line in open(0):
        game, cubes = line.strip().split(": ")
        colorFewest = {
            "red": 0,
            "blue": 0,
            "green": 0
        }
        for rounds in cubes.split('; '):
            for round in rounds.split(', '):
                value, color = round.split()
                value = int(value)
                if colorFewest[color] < value:
                    colorFewest[color] = value

        result += colorFewest["red"] * colorFewest["blue"] * colorFewest["green"]
    print(result)

if __name__ == '__main__':
    part2()


