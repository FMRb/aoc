import time
import re
from functools import reduce

def part1():
    input = open(0).read().split("\n\n")

    seeds = list(map(int,input.pop(0).split(": ")[1].split()))

    s_t_s = []
    s_t_f = []
    f_t_w = []
    w_t_l = []
    l_t_t = []
    t_t_h = []
    h_t_l = []

    start = time.process_time()
    convert_to = [s_t_s, s_t_f, f_t_w, w_t_l, l_t_t, t_t_h, h_t_l]
    for index,m in enumerate(input):
        formula = [list(map(int, line.split()))for line in m.splitlines()[1:]]
        use_list = convert_to[index]
        for dst, src, rng in formula:
            use_list.append((src, dst, rng))
    print(time.process_time() - start)


    result = []
    for seed in seeds:
        t_n = seed
        for m in convert_to:
            for cond in m:
                src, dst, rng = cond
                if t_n >= src and t_n < src+rng:
                    t_n = (t_n - src) + dst
                    break
        result.append(t_n)

    print(min(result))

def part2():
    input = open(0).read().split("\n\n")

    seeds = list(map(int,input.pop(0).split(": ")[1].split()))

    seeds = list(zip(seeds[::2], seeds[1::2]))

    convert_to = []
    for m in input:
        formula = [list(map(int, line.split()))for line in m.splitlines()[1:]]
        use_list = []
        for dst, src, rng in formula:
            use_list.append([src, src + rng - 1, dst - src])
        convert_to.append(sorted(use_list))

    def translate_map(convert, sb, se):
        new_seeds = []
        for base, end, delta in convert:
            # 1: +++
            # 2:     ---
            # OR
            # 1:     +++
            # 2: ---
            if se < base or sb > end:
                continue

            # 1:   +++++
            # 2:  ----
            # OR
            # 1:   ++++
            # 2:  ------
            if sb < base:
                new_seeds += [(sb, base - 1), (base+delta, min(end, se)+delta)]

            # 1: ++++++
            # 2:    -----
            # OR
            # 1: ++++++++
            # 2:   ----
            else:
                new_seeds += [(sb+delta, min(end, se) + delta)]

            if end > se:
                return new_seeds
            sb = end
        if not new_seeds:
            new_seeds = [(sb, se)]
        return new_seeds

    tmp_result = []
    for base_seed, l in seeds:
        checking_seeds = [(base_seed, base_seed + l)]
        for convert in convert_to:
            new_checking_seeds = []
            # seed_base - seed_end
            for s_b, s_e in checking_seeds:
                new_checking_seeds += translate_map(convert, s_b, s_e)
            checking_seeds = new_checking_seeds
        tmp_result.append(min(checking_seeds)[0])

    print(min(tmp_result))

if __name__ == "__main__":
    #part1()
    part2()
