import sys
import heapq


class Vertex:
    def __init__(self, node):
        self.id = node
        self.distance = sys.maxsize
        self.adjacent = {}
        self.visited = False
        self.previous = None

    def add_neighbor(self, neighbor, weight=0):
        self.adjacent[neighbor] = weight

    def get_weight(self, neighbor):
        return self.adjacent[neighbor]

    def get_distance(self):
        return self.distance

    def set_distance(self, dist):
        self.distance = dist

    def set_visited(self):
        self.visited = True

    def set_previous(self, previous):
        self.previous = previous

    def __lt__(self, nxt):
        return self.distance < nxt.distance

    def __str__(self):
        return f'vertex: {self.id} - adjacent: {[(k.id,v) for k,v in self.adjacent.items()]}'


class Graph:
    def __init__(self):
        self.vert_dict = {}
        self.num_vertices = 0

    def __iter__(self):
        return iter(self.vert_dict.values())

    def get_vertex(self, n):
        return self.vert_dict.get(n)

    def add_vertex(self, node):
        self.num_vertices += 1
        new_vertex = Vertex(node)
        self.vert_dict[node] = new_vertex
        return new_vertex

    def add_edge(self, frm, to, cost=0):
        if frm not in self.vert_dict:
            self.add_vertex(frm)
        if to not in self.vert_dict:
            self.add_vertex(to)

        self.vert_dict[frm].add_neighbor(self.vert_dict[to], cost)

    def __str__(self):
        r = ''
        for value in self.vert_dict.values():
            r += f'{value}\n'
        return r


def parseGraph(data):
    adjacent = [(1, 0), (0, 1), (-1, 0), (0, -1)]
    graph = Graph()
    h = len(data)
    w = len(data[0])
    for j in range(h):
        for i in range(w):
            frm = (j, i)
            for y, x in adjacent:
                a_y = j + y
                a_x = i + x
                if a_y >= 0 and a_y < h and a_x >= 0 and a_x < w:
                    to = (a_y, a_x)
                    cost = int(data[a_y][a_x])
                    graph.add_edge(frm, to, cost)
    return graph, w, h


def shortest(v, path):
    ''' make shortest path from v.previous'''
    if v.previous:
        path.append(v.previous.get_id())
        shortest(v.previous, path)
    return


def minDistance(queue):
    dist = sys.maxsize
    min_v = None

    for v in queue:
        if v.get_distance() < dist and v.visited == False:
            dist = v.get_distance()
            min_v = v
    return min_v


def dijkstra(graph, start):
    # Set the distance for the start node to zero

    start.set_distance(0)
    unvisited_queue = [start]
    while len(unvisited_queue):
        current = heapq.heappop(unvisited_queue)
        current.set_visited()

        for next in current.adjacent:
            if next.visited:
                continue
            new_dist = current.get_distance() + current.get_weight(next)
            if new_dist < next.get_distance():
                next.set_distance(new_dist)
                next.set_previous(current)
                heapq.heappush(unvisited_queue, next)


def shortest(v, path):
    if v.previous:
        path.append((v.previous.id, v.previous.get_distance()))
        shortest(v.previous, path)
    return path


def extend_grid(data):
    grid = [[int(x) for x in line] for line in data]
    new_grid = []
    for i in range(5):
        for row in grid:
            new_row = []
            for j in range(5):
                n = [(((x + i + j) - 1) % 9) + 1 for x in row]
                new_row = [*new_row, *n]
            new_grid.append(new_row)

    return new_grid


def part1(data):
    graph, w, h = parseGraph(data)
    dijkstra(graph, graph.get_vertex((0, 0)))

    target = graph.get_vertex((h-1, w-1))
    print(target.get_distance())


def part2(data):
    grid = extend_grid(data)
    # for r in grid:
    #     print(''.join([str(i) for i in r]))
    #     print()
    graph, w, h = parseGraph(grid)
    dijkstra(graph, graph.get_vertex((0, 0)))

    target = graph.get_vertex((h-1, w-1))
    print(target.get_distance())


if __name__ == '__main__':
    data = [x.strip() for x in open('data.in').readlines()]
    # part1(data)
    part2(data)
