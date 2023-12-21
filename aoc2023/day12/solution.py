import itertools

def detect_group(condition):
    g_record = []
    count = 0
    for c in condition:
        if c == '.' and count > 0:
            g_record.append(count)
            count = 0
        elif c == '#':
            count += 1
    if count > 0:
        g_record.append(count)

    return g_record

# ????.######..#####.????.######..#####.????.######..#####.????.######..#####.????.######..#####. 1,6,5,1,6,5
def part1(springs):

    results = []
    for condition, records in springs:
        
        unknows = [i for i, c in enumerate(condition) if c == "?"]
        n = len(unknows)

        combinations = [list(i) for i in itertools.product(['.', '#'], repeat=n)]

        agmts = 0
        for comb in combinations:
            new_cond = condition[:]
            for i, i_u in enumerate(unknows):
                new_cond[i_u] = comb[i]
            if detect_group(new_cond) == records:
                agmts += 1

        results.append(agmts)
    print("Part 1: ", sum(results))

cache = {}
def compute_arrangements(condition, records):
    key = condition + "".join(map(str,records))
    if cache.get(key) != None:
         return cache.get(key)

    if len(condition) == 0:
        return 1 if len(records) == 0 else 0
    
    if len(records) == 0:
        return 0 if "#" in condition else 1

    results = 0
    if condition[0] in ".?":
        results += compute_arrangements(condition[1:], records[:])

    if condition[0] in "#?":
        if len(condition) >= records[0] and "." not in condition[:records[0]] and (len(condition) == records[0] or condition[records[0]] != "#"):
            results += compute_arrangements(condition[records[0]+1:], records[1:])

    cache[key] = results
    return results

def part2(springs):
    total = 0
    for condition, records in springs:
        condition = "".join(list(condition) + ["?", *condition]*4)
        records *= 5
        cache.clear()
        total += compute_arrangements(condition, records)
    print("Part 2: ", total)

if __name__ == "__main__":
    springs = []
    for line in open(0).read().strip().splitlines():
        condition, records = line.split()
        records = list(map(int, records.split(",")))
        springs.append((condition, records))

    #part1(springs)
    part2(springs)


