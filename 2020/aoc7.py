from typing import List, Tuple


bags = {}

class Vertex:
    def __init__(self, node):
        self.id = node
        self.parents: List[Tuple[Vertex, int]] = []
        self.children: List[Tuple[Vertex, int]] = []

    def __str__(self):
        return f"Vertex {self.id}"

    def add_neighbor(self, neighbor, weight, direction):
        if direction:
            self.children.append((neighbor, weight))
        else:
            self.parents.append((neighbor, weight))

    # def get_weight(self, neighbor):
    #     return self.adjacent[neighbor]

class Graph:
    def __init__(self):
        self.vert_dict = {}
        self.num_vertices = 0

    def __iter__(self):
        return iter(self.vert_dict.values())

    def add_vertex(self, node):
        self.num_vertices = self.num_vertices + 1
        new_vertex = Vertex(node)
        self.vert_dict[node] = new_vertex
        return new_vertex

    def get_vertex(self, n) -> Vertex:
        if n in self.vert_dict:
            return self.vert_dict[n]
        else:
            return None

    def add_edge(self, frm, to, cost = 0):
        if frm not in self.vert_dict:
            self.add_vertex(frm)
        if to not in self.vert_dict:
            self.add_vertex(to)

        self.vert_dict[frm].add_neighbor(self.vert_dict[to], cost, True)
        self.vert_dict[to].add_neighbor(self.vert_dict[frm], cost, False)

    def get_vertices(self):
        return self.vert_dict.keys()

with open("aoc7.inp") as rf:
    for l in rf:
        if l:
            parent_bag, contents_data = l.split("contain")
            contents_lst = contents_data.replace(".", "").split(", ")
            contents = []
            for c in contents_lst:
                num, bag = c.strip().split(" ", 1)
                num = int(num) if num != "no" else 0
                bag = bag.replace(" bags", "").replace(" bag", "")
                contents.append((bag, num))

            bags[parent_bag.replace("bags", "").strip()] = contents

graph = Graph()

for parent, contents in bags.items():
    for bag, cost in contents:
        graph.add_edge(parent, bag, cost)

# Solution 1
visited = []
stack = [graph.get_vertex('shiny gold')]
while stack:
    vertex = stack.pop()

    if vertex in visited:
        continue

    visited.append(vertex)
    for parent, _ in vertex.parents:
        stack.append(parent)

# Solution 2
def get_score(vertices, mult):
    score = 0

    for v, c in vertices:
        if c == 0:
            continue

        score += c * mult
        score += get_score(v.children, c * mult)

    return score

print(get_score(graph.get_vertex('shiny gold').children, 1))
