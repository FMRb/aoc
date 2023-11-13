'''
0(6):     1(2):   2(5):   3(5):  4(4):
 aaaa    ....    aaaa    aaaa    ....
b    c  .    c  .    c  .    c  b    c
b    c  .    c  .    c  .    c  b    c
 ....    ....    dddd    dddd    dddd
e    f  .    f  e    .  .    f  .    f
e    f  .    f  e    .  .    f  .    f
 gggg    ....    gggg    gggg    ....

5(5):     6(6):   7(3):   8(7):   9(6):
 aaaa    aaaa    aaaa    aaaa    aaaa
b    .  b    .  .    c  b    c  b    c
b    .  b    .  .    c  b    c  b    c
 dddd    dddd    ....    dddd    dddd
.    f  e    f  .    f  e    f  .    f
.    f  e    f  .    f  e    f  .    f
 gggg    gggg    ....    gggg    gggg

Rule 0: only len(6) after discovering #9 and #6
Rule 1: len(2) (unique length)
Rule 2: only len(5) remaining number
Rule 3: len(5) and contains chars7[]
Rule 4: len(4) (unique length)
Rule 5: len(5) and contains 3 common chars4[]
Rule 6: len(6) and not contain chars7[]
Rule 7: len(3) (unique length)
Rule 8: len(7) (unique length)
Rule 9: len(6) and contains chars3[]

'''


def part1(data):
    digits = [line.split('|')[1] for line in data]

    counter = 0
    for d in digits:
        for digit in d.strip().split(' '):
            l = len(digit)
            if l == 2 or l == 3 or l == 4 or l == 7:
                counter += 1
    print(counter)
    return


'''
Rule 0: only len(6) after discovering #9 and #6
Rule 1: len(2) (unique length)
Rule 2: only len(5) remaining number
Rule 3: len(5) and contains chars7[]
Rule 4: len(4) (unique length)
Rule 5: len(5) and contains 3 common chars4[]
Rule 6: len(6) and not contain chars7[]
Rule 7: len(3) (unique length)
Rule 8: len(7) (unique length)
Rule 9: len(6) and contains chars3[]
'''


def chars_in_string(chars, text):
    for char in chars:
        if not char in text:
            return False
    return True


def count_chars_in_string(chars, text):
    count = 0
    for char in chars:
        if char in text:
            count += 1

    return count


def find_in_list(l, condition, first_result=True):
    results = []
    for x in l:
        if condition(x):
            if first_result:
                return x
            results.append(x)
    return results


def parse_signal_patterns(patterns):
    signal_patterns = {}

    one_pattern = find_in_list(patterns, lambda x: len(x) == 2)
    signal_patterns[''.join(sorted(one_pattern))] = 1
    patterns.remove(one_pattern)

    four_pattern = find_in_list(patterns, lambda x: len(x) == 4)
    signal_patterns[''.join(sorted(four_pattern))] = 4
    patterns.remove(four_pattern)

    seven_pattern = find_in_list(patterns, lambda x: len(x) == 3)
    signal_patterns[''.join(sorted(seven_pattern))] = 7
    patterns.remove(seven_pattern)

    eight_pattern = find_in_list(patterns, lambda x: len(x) == 7)
    signal_patterns[''.join(sorted(eight_pattern))] = 8
    patterns.remove(eight_pattern)

    patterns_len_five = find_in_list(patterns, lambda x: len(x) == 5, False)
    three_pattern = find_in_list(
        patterns_len_five, lambda x: chars_in_string(seven_pattern, x))
    signal_patterns[''.join(sorted(three_pattern))] = 3
    patterns_len_five.remove(three_pattern)
    patterns.remove(three_pattern)

    five_pattern = find_in_list(
        patterns_len_five, lambda x: count_chars_in_string(four_pattern, x) == 3)
    signal_patterns[''.join(sorted(five_pattern))] = 5
    patterns_len_five.remove(five_pattern)
    patterns.remove(five_pattern)

    two_pattern = patterns_len_five.pop()
    assert(len(patterns_len_five) == 0)
    signal_patterns[''.join(sorted(two_pattern))] = 2
    patterns.remove(two_pattern)

    # only patterns len 6 remaining
    assert(len(patterns) == 3)

    nine_pattern = find_in_list(
        patterns, lambda x:  chars_in_string(three_pattern, x))
    signal_patterns[''.join(sorted(nine_pattern))] = 9
    patterns.remove(nine_pattern)

    six_pattern = find_in_list(
        patterns, lambda x: not chars_in_string(seven_pattern, x))
    signal_patterns[''.join(sorted(six_pattern))] = 6
    patterns.remove(six_pattern)

    assert(len(patterns) == 1)
    cero_pattern = patterns.pop()
    signal_patterns[''.join(sorted(cero_pattern))] = 0

    return signal_patterns


def part2(data):
    total = 0
    for line in data:
        patterns = line.split(' | ')

        signal_pattern = parse_signal_patterns(patterns[0].split(' '))
        result = ''
        for output in patterns[1].split(' '):
            key = ''.join(sorted(output))
            result += f'{signal_pattern.get(key)}'

        total += int(result)
    print(total)
    return


if __name__ == '__main__':
    data = [x.strip() for x in open('data.in').readlines()]
    part1(data)
    part2(data)
