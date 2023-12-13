def mirror_diff(m_a, m_b):
    if len(m_a) != len(m_b):
        raise Exception("wrong len mirrors")

    diff = []
    for i, m in enumerate(m_a):
        if m != m_b[i]:
            diff.append((m, m_b[i], i))
    return diff

def mirror_check(pattern, mids, smudge, part2=False):
    results = [0]
    for mid in mids:
        i = mid
        j = mid + 1

        while True:
            if pattern[i] == pattern[j]:
                i -= 1
                j += 1
            else:
                if part2 and not smudge:
                    diff = mirror_diff(pattern[i], pattern[j])
                    if len(diff) == 1:
                        smudge = diff[0]
                        _, diff_b, i_d = diff[0]
                        pattern = update_pattern(pattern, i, i_d, diff_b)
                        i -= 1
                        j +=1
                        if i < 0 or j == len(pattern):
                            break
                        else:
                            continue
                    
                results.append(-1)
                break
            
            if i < 0 or j == len(pattern):
                break

        if i < 0 or j == len(pattern):
            results.append(mid+1)
    return results

def update_pattern(pattern, row_i, col_i, ch):
    new_p = pattern[:]
    lp = list(new_p[row_i])
    lp[col_i] = ch
    new_p[row_i] = "".join(lp)
    
    return new_p

def find_mirror(pattern, smudge, part2=False):
    i = 0
    j = 1

    p_mids = []
    new_smudge = None
    while j < len(pattern):
        diff = mirror_diff(pattern[i], pattern[j])
        if pattern[i] == pattern[j]:
            p_mids.append(i)
        elif len(diff) == 1 and part2 and not smudge and not new_smudge:
            new_smudge = (diff[0], i)
            p_mids.append(i)
        i += 1
        j += 1

    results = mirror_check(pattern, p_mids, smudge, part2)
    if new_smudge and part2:
        d_a, d_b, d_i = new_smudge[0]
        pattern_a = update_pattern(pattern, new_smudge[1], d_i, d_b)
        result_a = mirror_check(pattern_a, p_mids, new_smudge, True)

        pattern_b = update_pattern(pattern, new_smudge[1]+1, d_i, d_a)
        result_b = mirror_check(pattern_b, p_mids, new_smudge, True)
        
        r = max(results)
        result_a = [ra for ra in result_a if ra != r]
        result_b = [rb for rb in result_b if rb != r]
        ra = max(result_a)
        rb = max(result_b)
        # print(f"ra: {ra}  rb: {rb}  r:{r}")
        if ra > rb:
            return pattern_a, ra, new_smudge
        else:
            return pattern_b, rb, new_smudge

    return pattern, max(results), smudge

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
        smudge = None
        # Vertical
        _,rv,_ = find_mirror(rotate_grid(pattern), smudge)
        v.append(rv)
        # Horizontal
        _,rh,_ = find_mirror(pattern, smudge)
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
        smudge = None
        # Horizontal
        p_h, rh, smudge = find_mirror(pattern, smudge, True)

        # Vertical
        if rh == 0:
            p_v, rv, _ = find_mirror(rotate_grid(pattern), None, True)
            p_r = rotate_grid(pattern)
            print("V", end="\n\n")
            for i, p in enumerate(p_v):
                print(f"{p}      {p_r[i]}")
        else:
            p_v, rv, _ = find_mirror(rotate_grid(p_h),smudge, True)
            p_r = rotate_grid(pattern)
            print("V_H", end="\n\n")

            for i, p in enumerate(p_v):
                print(f"{p}      {p_r[i]}")

        if rh > 0:
            h.append(rh)
            v.append(0)
        else:
            v.append(rv)
            h.append(0)

        print("H", end="\n\n")
        for i,p in enumerate(p_h):
            print(f"{p}      {pattern[i]}")


        print()
        print("Vertical: ",rv)
        print("Horizontal: ", rh)
        print()

    print("Part 2: ", (sum(h) * 100) + sum(v))
    pass


if __name__ == "__main__":
    patterns = []
    for pattern in open(0).read().strip().split("\n\n"):
        patterns.append([l for l in pattern.splitlines()])

    #part1(patterns)
    part2(patterns)
