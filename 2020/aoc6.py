from functools import reduce

data = []

with open("aoc6.inp") as rf:
    sets = []
    for l in rf:
        if l == "\n":
            data.append(sets)
            sets = []
        else:
            sets.append(set([c for c in l.strip()]))


a1 = a2 = 0
for sets in data:
    a1 += len(reduce(lambda s1, s2: s1 | s2, sets))
    a2 += len(reduce(lambda s1, s2: s1 & s2, sets))


print(a1, a2)
