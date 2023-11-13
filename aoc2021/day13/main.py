def createSheet(dots):
    sheet_width, sheet_height = find_max_number(dots)

    sheet = [[0 for i in range(sheet_width + 1)]
             for j in range(sheet_height + 1)]

    for [x, y] in dots:
        sheet[y][x] = 1
    return sheet


def find_max_number(dots):
    max_x = 0
    max_y = 0
    for [x, y] in dots:
        max_x = max(x, max_x)
        max_y = max(y, max_y)
    return max_x, max_y


def print_sheet(sheet):
    for j in sheet:
        for v in j:
            if v == 0:
                print('.', end='')
            else:
                print('#', end='')
        print()


def fold_up(sheet, value):
    top_part = sheet[:value] if len(sheet) % 2 == 0 else sheet[:value]
    bottom_part = sheet[:value] if int(
        len(sheet)/2) == value else sheet[:value+1]
    bottom_part.reverse()

    try:
        assert len(top_part) == len(bottom_part), "Not possible to fold up"
    except:
        print('EXCEPTION WARNING!!')
        print(f'sheet len {len(sheet)}')
        print(f'top: {len(top_part)}')
        print(f'bottom: {len(bottom_part)}')
    for j in range(len(bottom_part)):
        for i in range(len(bottom_part[j])):
            top_part[j][i] += bottom_part[j][i]
    return top_part


def fold_left(sheet, value):
    left_sheet = []
    right_sheet = []
    fold_value = value if len(sheet[0]) % 2 == 0 else value

    for j in range(len(sheet)):
        left_sheet.append(sheet[j][:fold_value])
        r = sheet[j][value+1:]
        r.reverse()
        right_sheet.append(r)

    assert len(left_sheet) == len(right_sheet), "Not possible to fold left"
    for j in range(len(right_sheet)):
        assert len(left_sheet[j]) == len(
            right_sheet[j]), "Not possible to fold left, row"
        for i in range(len(right_sheet[j])):
            left_sheet[j][i] += right_sheet[j][i]
    return left_sheet


def part1(dots, folding):
    sheet = createSheet(dots)

    for fold in folding:
        [coord, value] = fold.split('=')
        if coord == 'y':
            sheet = fold_up(sheet, int(value))
        elif coord == 'x':
            sheet = fold_left(sheet, int(value))
        break

    count = 0
    for j in sheet:
        for v in j:
            if v > 0:
                count += 1
    print(count)


def part2(dots, folding):
    sheet = createSheet(dots)

    for fold in folding:
        [coord, value] = fold.split('=')
        if coord == 'y':
            sheet = fold_up(sheet, int(value))
        elif coord == 'x':
            sheet = fold_left(sheet, int(value))

    print()
    print('Result:')
    print_sheet(sheet)


if __name__ == '__main__':
    data = [x.strip() for x in open('data.in').readlines()]
    dots = []
    folding = []
    for line in data:
        if 'fold along' in line:
            folding.append(line[len('fold along '):])
            continue
        if line == '':
            continue
        dots.append(line)
    dots = [list(map(int, d.split(','))) for d in dots]

    # part1(dots, folding)
    part2(dots, folding)
