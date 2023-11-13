def group_columns(data):
    control_list = []
    for line in data:
        for i in range(len(line)):
            if len(control_list) <= i:
                control_list.append(line[i])
            else:
                control_list[i] += line[i]
    return control_list


def part1(data):
    gamma = ''
    epsilon = ''

    columns = group_columns(data)
    for column in columns:
        if column.count('0') > column.count('1'):
            gamma += '0'
            epsilon += '1'
        else:
            gamma += '1'
            epsilon += '0'

    print(int(gamma, 2) * int(epsilon, 2))


def filter_list(l, i, p):
    if len(l) == 1:
        return l
    return list(filter(lambda c: c[i] == p, l))


def part2(data):
    oxygen_candidates = data[:]
    co2_candidates = data[:]

    oxy_index = 0
    while len(oxygen_candidates) > 1:
        column = ' '.join(list(map(lambda l: l[oxy_index], oxygen_candidates)))

        if column.count('0') > column.count('1'):
            oxygen_candidates = filter_list(oxygen_candidates, oxy_index, '0')
        elif column.count('1') > column.count('0'):
            oxygen_candidates = filter_list(oxygen_candidates, oxy_index, '1')
        else:
            oxygen_candidates = filter_list(oxygen_candidates, oxy_index, '1')
        oxy_index += 1

    co2_index = 0
    while len(co2_candidates) > 1:
        column = ' '.join(list(map(lambda l: l[co2_index], co2_candidates)))
        if column.count('0') > column.count('1'):
            co2_candidates = filter_list(co2_candidates, co2_index, '1')
        elif column.count('1') > column.count('0'):
            co2_candidates = filter_list(co2_candidates, co2_index, '0')
        else:
            co2_candidates = filter_list(co2_candidates, co2_index, '0')
        co2_index += 1
    print(int(oxygen_candidates[0], 2) * int(co2_candidates[0], 2))


if __name__ == '__main__':
    data = [x.strip() for x in open('data.in').readlines()]
    # part1(data)
    part2(data)
