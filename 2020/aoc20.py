from collections import Counter

tiles = {}
borders = []


def parse_borders(tile):
    def to_binary(line):
        return int("".join("1" if x == "#" else "0" for x in line), 2)

    def edge(idx):
        return [row[idx] for row in tile]

    return {
        "t": to_binary(tile[0]),
        "b": to_binary(tile[-1]),
        "l": to_binary(edge(0)),
        "r": to_binary(edge(-1)),
    }


def compare_all(n1, n2):
    def rev(n):
        return int("{0:010b}".format(n)[::-1], 2)

    def inv(n):
        return n ^ 0b1111111111

    return n1 == n2 or n1 == inv(n2) or n1 == rev(n2)


with open("aoc20.inp") as rf:
    tile = []
    for l in rf:
        if len(tile) == 11:
            header = tile.pop(0)
            bs = parse_borders(tile)
            id = header.split(" ")[1].replace(":", "")
            tiles[id] = {"tile": tile, "borders": bs}

            for b, data in bs.items():
                borders.append((id, b, data))

            tile = []

        if l != "\n":
            tile.append(l.strip())


opposite_side = {"b": "t", "t": "b", "l": "r", "r": "l"}
missing = []
for id, side, border_data in borders:
    if not any(
        compare_all(border_data, other_data)
        for other_id, _, other_data in borders
        if other_id != id
    ):
        missing.append(id)


print([(c, n) for c, n in Counter(missing).items() if n > 1])
