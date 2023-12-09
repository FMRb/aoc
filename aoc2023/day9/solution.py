# 
# 0 3 6 9 12 15
# 1 3 6 10 15 21
# 10 13 16 21 30 45
#

def find_sequence(sequence, ext, part2=False):
    n_s = []

    for i in range(len(sequence) - 1):
        n_s.append(sequence[i+1]-sequence[i])
    if all([n == 0 for n in n_s]):
        if part2:
            r = -1 * sequence[0]
            while len(ext) > 0:
                r = -1 * (r + ext.pop())
            return -1 * r
        else:
            ext.append(sequence[-1])
            return sum(ext)
    
    if part2:
        ext.append(sequence[0])
    else:
        ext.append(sequence[-1])
    return find_sequence(n_s, ext, part2)




def part1(sequences):

    next_nums = []
    for sequence in sequences:
        next_nums.append(find_sequence(sequence, []))

    print("Part 1: ", sum(next_nums))


def part2(sequences):
    next_nums = []
    for sequence in sequences:
        next_nums.append(find_sequence(sequence, [], True))

    print("Part 2: ", sum(next_nums))

if __name__ == "__main__":
    input = [list(map(int, line.split())) for line in open(0).read().splitlines()]
    part1(input)
    part2(input)
