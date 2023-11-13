from collections import defaultdict


class Graph:
    def __init__(self):
        self.edges = defaultdict(list)

    def add_edge(self, src, dst):
        self.edges[src].append(dst)
        self.edges[dst].append(src)

    # start,A,b,A,b,A,c,A,end

    def _DFSRecur(self, node, end_node, num_visited, visited, path_counter):
        if node.islower():
            visited[node] += 1

        if node == end_node:
            visited[node] = visited[node] - 1
            return path_counter + 1
        else:
            cave_small_visited = [(x, y)
                                  for (x, y) in visited.items() if y == num_visited]
            for connection in self.edges[node]:
                if connection == 'start' and visited[connection] > 0:
                    continue
                if connection == 'end' and visited[connection] > 0:
                    continue
                if visited[connection] >= (num_visited - 1) and len(cave_small_visited) > 0:
                    continue

                if visited[connection] < num_visited:
                    path_counter = self._DFSRecur(connection,
                                                  end_node, num_visited, visited, path_counter)

        visited[node] = visited[node] - 1
        return path_counter

    def DFT(self, start_node, end_node, num_visited):
        visited = defaultdict(int)
        return self._DFSRecur(start_node, end_node, num_visited, visited, 0)

    def __str__(self):
        return f'{self.edges.items()}'


def part1(data):
    data = [x.split('-') for x in data]
    graph = Graph()
    for [src, dst] in data:
        graph.add_edge(src, dst)

    paths = graph.DFT('start', 'end', 1)
    print(paths)


def part2(data):
    data = [x.split('-') for x in data]
    graph = Graph()
    for [src, dst] in data:
        graph.add_edge(src, dst)

    paths = graph.DFT('start', 'end', 2)
    print(paths)


if __name__ == '__main__':
    data = [line.strip() for line in open('data.in').readlines()]
    part1(data)
    part2(data)
