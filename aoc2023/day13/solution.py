def mirror_diff(m_a, m_b):
    if len(m_a) != len(m_b):
        raise Exception("wrong len mirrors")

    diff = []
    for i, m in enumerate(m_a):
        if m != m_b[i]:
            diff.append((m, m_b[i], i))
    return diff

def mirror_check(pattern, mids, part2=False):
    results = [(0, True)]
    for mid in mids:
        i = mid
        j = mid + 1

        has_diff = False
        while True:
            diff = mirror_diff(pattern[i], pattern[j])
            if pattern[i] == pattern[j]:
                i -= 1
                j += 1
            elif part2 and len(diff) == 1 and not has_diff:
                has_diff = True
                i -= 1
                j +=1
            else:
                results.append((-1, False))
                break
            
            if i < 0 or j == len(pattern):
                break

        if i < 0 or j == len(pattern):
            results.append((mid+1, has_diff))
    return results

def update_pattern(pattern, row_i, col_i, ch):
    new_p = pattern[:]
    lp = list(new_p[row_i])
    lp[col_i] = ch
    new_p[row_i] = "".join(lp)
    
    return new_p

def find_mirror(pattern, part2=False):
    i = 0
    j = 1

    p_mids = []
    while j < len(pattern):
        diff = mirror_diff(pattern[i], pattern[j])
        if pattern[i] == pattern[j]:
            p_mids.append(i)
        elif len(diff) == 1 and part2:
            p_mids.append(i)
        i += 1
        j += 1

    results = mirror_check(pattern, p_mids, part2)

    results = [r for r, hd in results if part2 and hd]

    return max(results)

'''
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.


#.##..#
..##...
##..###
#....#.
.#..#.#
.#..#.#
#....#.
##..###
..##...
'''

def rotate_grid(pattern):
    grid = []
    for x in range(len(pattern[0])):
        row = []
        for y in range(len(pattern)):
            row.append(pattern[y][x])
        grid.append("".join(row))

    return grid

def part1(patterns):
    v = []
    h = []
    for pattern in patterns:
        # Vertical
        rv = find_mirror(rotate_grid(pattern))
        v.append(rv)
        # Horizontal
        rh = find_mirror(pattern)
        h.append(rh)

        # for l in pattern:
        #     print(l)

        # print()
        # print("Vertical: ",rv)
        # print("Horizontal: ", rh)
        # print()

    print("Part 1: ", (sum(h) * 100) + sum(v))

def part2(patterns):

    v = []
    h = []
    for pattern in patterns:
        # Vertical
        rv  = find_mirror(rotate_grid(pattern), True)
        v.append(rv)
        # Horizontal
        rh = find_mirror(pattern, True)
        h.append(rh)

    print("Part 2: ", (sum(h) * 100) + sum(v))
    pass


if __name__ == "__main__":
    patterns = []
    for pattern in open(0).read().strip().split("\n\n"):
        patterns.append([l for l in pattern.splitlines()])

    #  part1(patterns)
    part2(patterns)
