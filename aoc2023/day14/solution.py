
def tilt_vertical(reflector, is_south = False):
    reflector = reflector[::-1] if is_south else reflector
    for i in range(1, len(reflector)):
        for j in range(len(reflector[0])):
            ti = i
            while ti > 0 and reflector[ti-1][j] == ".":
                if reflector[ti][j] == "O":
                    reflector[ti-1][j] = "O"
                    reflector[ti][j] = "."
                ti -= 1
    return reflector[::-1] if is_south else reflector

def tilt_horizontal(reflect, is_east = False):
    for j in range(len(reflector)):
        r = reflector[j][::-1] if is_east else reflector[j]
        for i in range(1, len(r)):
            ti = i
            while ti > 0 and r[ti-1] == ".":
                if r[ti] == "O":
                    r[ti] = "."
                    r[ti-1] = "O"
                ti -= 1
        reflector[j] = r[::-1] if is_east else r

    return reflector

def part1(reflector):
    reflector = tilt_vertical(reflector)

    count = 0

    row_value = len(reflector)

    for r in reflector:
        count += r.count("O") * row_value
        row_value -= 1

    print("Part1: ", count)


def calc_load_north(reflector):
    count = 0

    row_value = len(reflector)

    for r in reflector:
        count += r.count("O") * row_value
        row_value -= 1
    return count

def p_reflector(reflector):
    for r in reflector:
        print("".join(r))

def most_frequent(List):
    return max(set(List), key = List.count)

def part2(reflector):
    cache = {}
    # Order: N W S E
    goal = 1000000000
    cycles = 0
    v_hundreds = {}
    while cycles < 500:
        reflector = tilt_vertical(reflector)
        reflector = tilt_horizontal(reflector)
        reflector = tilt_vertical(reflector, is_south= True)
        reflector = tilt_horizontal(reflector, is_east= True)

        c_n = calc_load_north(reflector)
        if c_n not in cache:
            cache[c_n] = [cycles]
        else:
            cache[c_n].append(cycles)

        if cycles > 480 and cycles < 500:
            v_hundreds[cycles] = c_n
        cycles += 1

    most_diff = []
    for k in cache:
        v = cache[k]
        most_diff += [b-a for a, b in list(zip(v, v[1:]))]
        
    increment = most_frequent(most_diff)

    k_v = None
    for k in v_hundreds:
        if (goal - k) % increment == 0:
            k_v = k - 1
            break

    print("Part2: ", v_hundreds[k_v])


'''
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
'''
if __name__ == "__main__":
    reflector = []
    for line in open(0).read().strip().splitlines():
        reflector.append([*line])
    
    # part1(reflector)
    part2(reflector)
