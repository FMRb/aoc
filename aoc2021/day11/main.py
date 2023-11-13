

class Grid:
    adjacent_cords = [(1, 0), (0, 1), (1, 1), (-1, 0),
                      (0, -1), (-1, -1), (1, -1), (-1, 1)]

    def __init__(self, data):
        self.grid = []
        self.flashes = 0

        for line in data:
            row = [int(x) for x in line]
            self.grid.append(row)
        self.height = len(self.grid)
        self.width = len(self.grid[0])

    def gainEnergy(self):
        cell_to_flash = []
        for i in range(self.height):
            for j in range(self.width):
                self.grid[i][j] += 1
                if self.grid[i][j] > 9:
                    cell_to_flash.append((i, j))
        self._flash_cell(cell_to_flash)

    def increase_adjacents(self, i, j, already_flash):
        flash_adjacents = []
        for (a_j, a_i) in Grid.adjacent_cords:
            n_x = a_j + j
            n_y = a_i + i
            if n_x < 0 or n_x >= self.width or n_y < 0 or n_y >= self.height:
                continue
            if (n_y, n_x) in already_flash:
                continue

            self.grid[n_y][n_x] += 1
            if self.grid[n_y][n_x] > 9:
                flash_adjacents.append((n_y, n_x))
        return flash_adjacents

    def _flash_cell(self, cell_to_flash):
        already_flash = set()

        while len(cell_to_flash) > 0:
            i, j = cell_to_flash.pop()
            if (i, j) in already_flash:
                continue

            new_flash_cells = self.increase_adjacents(i, j, already_flash)
            self.flashes += 1
            cell_to_flash = [*cell_to_flash, *new_flash_cells]
            already_flash.add((i, j))
            self.grid[i][j] = 0

    def all_flashes(self):
        return sum(list(map(sum, self.grid))) == 0

    def __str__(self):
        s = ''
        for i in range(self.height):
            for j in range(self.width):
                s += f'{self.grid[i][j]}'
            s += '\n'
        return s


def part1(data, steps):

    grid = Grid(data)

    for i in range(steps):
        # print(f'Step: #{i+1}')
        grid.gainEnergy()
        # print(grid)
    print(grid.flashes)


def infinite_grid_energy(grid):
    step = 1
    while True:
        grid.gainEnergy()
        yield (grid, step)
        step += 1


def part2(data):
    grid = Grid(data)

    for (g, step) in infinite_grid_energy(grid):
        if g.all_flashes():
            print(step)
            break


if __name__ == '__main__':
    data = [line.strip() for line in open('data.in').readlines()]
    # part1(data, 100)
    part2(data)
