class Cell:
    def __init__(self, value):
        self.value = int(value)
        self.marked = False

    def get_value(self):
        return self.value

    def get_marked(self):
        return self.marked

    def set_marked(self):
        self.marked = True


class Board:
    def __init__(self, data):
        self.cells = []
        self.win = False
        for line in data:
            self.cells.append([Cell(x) for x in line.split(' ') if len(x) > 0])

    def mark_cell(self, value):
        for row in self.cells:
            for cell in row:
                if cell.value == value:
                    cell.set_marked()

    def get_win(self):
        return self.win

    def is_win(self):
        if self.win:
            return True
        #  Checking column
        for i in range(5):
            all_marked = True
            for cell in self.cells:
                all_marked = all_marked and cell[i].get_marked()
            if all_marked:
                self.win = True
                return True
        # Checking row
        for row in self.cells:
            all_marked = True

            for cell in row:
                all_marked = all_marked and cell.get_marked()
            if all_marked:
                self.win = True
                return True

    def sum_unmarked(self):
        result = 0
        for row in self.cells:
            for cell in row:
                if not cell.get_marked():
                    result += cell.get_value()
        return result


def parse_boards(data):
    board_size = 5
    boards = []
    i = 0
    while len(data) > i:
        line = data[i]
        if not line.strip():
            i += 1
            continue
        board = Board(data[i:i+board_size])
        i += board_size
        boards.append(board)

    return boards


def part1(data):
    draw_numbers = map(int, data.pop(0).split(','))
    boards = parse_boards(data)

    for value in draw_numbers:
        for board in boards:
            board.mark_cell(value)
            if board.is_win():
                print(value * board.sum_unmarked())
                return


def len_board_pending_win(boards):
    count = 0

    for board in boards:
        if not board.get_win():
            count += 1
    return count


def part2(data):
    draw_numbers = map(int, data.pop(0).split(','))
    boards = parse_boards(data)

    for value in draw_numbers:
        for board in boards:
            board.mark_cell(value)
            if board.is_win() and len_board_pending_win(boards) == 0:
                print(value * board.sum_unmarked())
                return


if __name__ == '__main__':
    data = [x.strip() for x in open('data.in').readlines()]
    part1(data)
    part2(data)
