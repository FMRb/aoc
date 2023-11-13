
# Scores
SCORE_CHECKS = {')': 3, ']': 57, '}': 1197, '>': 25137}


def part1(data):
    open_chars = ['(', '[', '{', '<']
    close_char_dict = {'(': ')', '[': ']', '{': '}', '<': '>'}
    result = 0
    for line in data:
        stack = []
        for char in line:
            if char in open_chars:
                stack.append(char)
            else:
                open_char = stack.pop()
                if close_char_dict[open_char] != char:
                    result += SCORE_CHECKS[char]
                    break
    print(result)


SCORE_INCOMPLETE = {')': 1, ']': 2, '}': 3, '>': 4}


def part2(data):
    open_chars = ['(', '[', '{', '<']
    close_char_dict = {'(': ')', '[': ']', '{': '}', '<': '>'}
    results = []
    for line in data:
        stack = []
        corrupted = False
        for char in line:
            if char in open_chars:
                stack.append(char)
            else:
                open_char = stack.pop()
                if close_char_dict[open_char] != char:
                    corrupted = True
        if corrupted or len(stack) == 0:
            continue
        score = 0
        while len(stack) > 0:
            close_char = close_char_dict[stack.pop()]
            score = score * 5
            score += SCORE_INCOMPLETE[close_char]

        results.append(score)

    results = sorted(results)
    print(results[int(len(results)/2)])


if __name__ == '__main__':
    data = [x.strip() for x in open('data.in').readlines()]
    # part1(data)
    part2(data)
