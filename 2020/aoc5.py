seats = []

def parse_seat(s):
    r, c = s[0:7], s[7:]
    rn = int("".join("1" if l == "B" else "0" for l in r), 2)
    cn = int("".join("1" if l == "R" else "0" for l in c), 2)
    return rn * 8 + cn

with open("aoc5.inp") as rf:
    for l in rf:
        seats.append(parse_seat(l.strip()))


print(list(sorted(seats)))

for i in range(len(seats)):
    if i + 48 not in seats:
        print(i + 48)
