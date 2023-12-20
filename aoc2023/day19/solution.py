from collections import deque
from functools import reduce


def part1(workflows, ratings):
    start_workflow = workflows["in"]

    rating_sort = []
    for rating in ratings:
        c_workflow = start_workflow
        while True:
            next_workflow = None
            for key, value, cond, result in c_workflow[0]:
                if cond == "<" and rating[key] < value:
                    next_workflow = result
                    break
                elif cond == ">" and rating[key] > value:
                    next_workflow = result
                    break

            if not next_workflow:
                next_workflow = c_workflow[1]

            if next_workflow in "AR":
                rating_sort.append((next_workflow, rating))
                break
            c_workflow = workflows[next_workflow]

    result = [rating[k] for r, rating in rating_sort if r == "A" for k in rating]
    print("Part 1: ", sum(result))


def part2(workflows):
    q = deque(["in"])

    parent = {}

    paths = []

    def backtrace(k, list_cat):
        path = [(k, cat, rng) for cat, rng in list_cat]
        while path[-1][0] != "in":
            path += parent[path[-1][0]]
        path.reverse()
        return path

    while q:
        k = q.popleft()

        rules, default = workflows[k]

        opposite_rng = []
        for key, value, cond, result in rules:
            if result == "R":
                if cond == ">":
                    opposite_rng.append((k, key, (1, value)))
                else:
                    opposite_rng.append((k, key, (value, 4000)))

                continue

            if cond == ">":
                rng = (value + 1, 4000)
                opposite_rng.append((k, key, (1, value)))
            else:
                rng = (1, value - 1)
                opposite_rng.append((k, key, (value, 4000)))

            if result == "A":
                paths.append(
                    backtrace(
                        k,
                        [
                            (key, rng),
                            *[(cat, rng) for _, cat, rng in opposite_rng[:-1]],
                        ],
                    )
                )
                continue

            parent[result] = [(k, key, rng), *opposite_rng[:-1]]
            q.append(result)

        if default == "R":
            continue

        if default == "A":
            paths.append(backtrace(k, [(cat, rng) for _, cat, rng in opposite_rng]))
            continue

        parent[default] = opposite_rng
        q.append(default)

    total = 0

    for p in paths:
        categories = {key: (1, 4000) for key in "xmas"}
        for k, cat, (c, d) in p:
            a, b = categories[cat]
            if a < c and c < b:
                categories[cat] = (c, min(b, d))
            elif a > c and a < d:
                categories[cat] = (a, min(d, b))
            elif a == c:
                categories[cat] = (a, min(b, d))
            else:
                raise Exception(f"Wrong range {a}, {b} --- {c} {d}")

        values = []
        for k in categories:
            if not categories[k]:
                values.append(4000)
            else:
                a, b = categories[k]
                if a == b:
                    print(
                        "EQUAL: ",
                        a,
                    )
                values.append(b - a + 1)

        tmp = reduce((lambda x, y: x * y), values)
        total += tmp

    print("Part 2: ", total)


if __name__ == "__main__":
    wfs, rts = open(0).read().strip().split("\n\n")

    workflows = {}
    for wf in wfs.splitlines():
        key, rest = wf.split("{")
        rules = rest[:-1].split(",")
        default_rule = rules[-1]
        conditions = []
        for rule in rules[:-1]:
            condition, result = rule.split(":")
            if ">" in condition:
                ins, value = condition.split(">")
                parse_condition = (ins, int(value), ">", result)
            else:
                ins, value = condition.split("<")
                parse_condition = (ins, int(value), "<", result)

            conditions.append(parse_condition)
        workflows[key] = (conditions, default_rule)

    ratings = []
    for rating in rts.splitlines():
        rating = rating[1:-1]
        p_rating = {}
        for r in rating.split(","):
            k, v = r.split("=")
            p_rating[k] = int(v)

        ratings.append(p_rating)

    part1(workflows, ratings)
    part2(workflows)
