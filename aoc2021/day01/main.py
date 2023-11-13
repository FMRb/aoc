filename = './input'


def increasing(l):
    increase_counter = 0
    prev_value = -1
    for value in l:
        if prev_value == -1:
            prev_value = value
            continue
        if prev_value < value:
            increase_counter += 1
        prev_value = value
    print(f'Increase value: {increase_counter}')


def p2():
    measuremens = []
    with open(filename, 'r') as file:
        lines = file.readlines()
        total_lines = len(lines)
        for line_index in range(total_lines):
            if line_index + 3 > total_lines:
                break
            valueA, valueB, valueC = int(lines[line_index]), int(
                lines[line_index+1]), int(lines[line_index+2])
            measuremens.append(valueA + valueB + valueC)
    increasing(measuremens)


def main():
    p2()
    # increase_counter = 0
    # prev_value = -1
    # with open(filename, 'r') as file:
    #     lines = file.readlines()

    #     for line in lines:
    #         value = int(line)
    #         if prev_value == -1:
    #             prev_value = value
    #             continue
    #         if prev_value < value:
    #             increase_counter += 1
    #         else:
    #             print(f'value: {value} - p_value: {prev_value}')
    #         prev_value = value
    # print(f'Increase value: {increase_counter}')
    # return increase_counter


if __name__ == '__main__':
    main()
