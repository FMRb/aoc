from collections import Counter, defaultdict

'''
Template:     NNCB
After step 1: NCNBCHB
After step 2: NBCCNBBBCBHCB
After step 3: NBBBCNCCNBBNBNBBCHBHHBCHB
After step 4: NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB

step 0
{
    NN -> 1
    NC -> 1
    CB -> 1
}

step 1
N .c. N .b. C .h. B
{
    NC -> 1
    CN -> 1
    NB -> 1
    BC -> 1
    CH -> 1
    HB -> 1
}

step 2
NBCCNBBBCBHCB
{
    NB -> 1
    BC -> 2
    CC
    CN
    NB
    BB -> 2
    CB
    HC
    CB

}
'''


def insertion_alg(compound, pair_insertions):
    new_compound = []
    for i in range(len(compound) - 1):
        polymer_a, polymer_b = compound[i], compound[i+1]
        polymer_c = pair_insertions[polymer_a+polymer_b]
        new_compound.append(polymer_a)
        new_compound.append(polymer_c)
    new_compound.append(compound[-1])
    return new_compound


def extend_dictionary_insertions(steps, pair_insertions, template_pair):
    i = 0
    compound = template_pair[:]
    while i < steps:
        compound = insertion_alg(compound, pair_insertions)
        i += 1
    return compound


def part1(polymer_template, pair_insertions):
    steps = 8
    i = 0
    compound = polymer_template
    while i < steps:
        compound = insertion_alg(compound, pair_insertions)
        print('----')
        print(''.join(compound))
        i += 1

    c = Counter(compound).most_common()
    print(c[0][1] - c[-1][1])


def generate_pair(template):
    d = {}
    for i in range(len(template) - 1):
        key = template[i]+template[i+1]
        d[key] = d.get(key, 0) + 1
    return d


def generate_combination(key, pair_insertions):
    # in:  NN
    # out: NC CN
    [polymer_a, polymer_b] = key
    polymer_c = pair_insertions[key]
    r = []
    return polymer_a+polymer_c, polymer_c+polymer_b


def part2(polymer_template, pair_insertions):
    steps = 40
    i = 0
    compound = generate_pair(polymer_template)
    while i < steps:
        d = {}
        for (key, value) in compound.items():
            pair_a, pair_b = generate_combination(key, pair_insertions)
            d[pair_a] = d.get(pair_a, 0) + value
            d[pair_b] = d.get(pair_b, 0) + value

        compound = d
        i += 1

    counter = {}
    for (key, value) in compound.items():
        [p_a, _] = key
        counter[p_a] = counter.get(p_a, 0) + value
    counter[polymer_template[-1]] = counter.get(polymer_template[-1]) + 1
    most_common = max(counter.values())
    least_common = min(counter.values())
    print(most_common-least_common)


if __name__ == '__main__':
    data = [x.strip() for x in open('data.in').readlines()]
    polymer_template = [x for x in data[0]]

    pair_insertions = dict([d.split(' -> ') for d in data[2:]])
    # part1(polymer_template, pair_insertions)

    part2(polymer_template, pair_insertions)
