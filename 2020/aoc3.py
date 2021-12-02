mmap = []
size = 0

def find_map_trees(right, down):
    x, y = right, down
    num_trees = 0
    while y < len(mmap):
        num_trees += 1 if mmap[y][x % (size - 1)] else 0
        x += right
        y += down

    return num_trees

with open("aoc3.inp") as rf:
    for l in rf:
        mmap.append([True if x == "#" else False for x in l])

    size = len(mmap[0])

print(find_map_trees(1, 1) *
find_map_trees(3, 1) *
find_map_trees(5, 1) *
find_map_trees(7, 1) *
find_map_trees(1, 2))
