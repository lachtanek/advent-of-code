from copy import deepcopy
from itertools import chain


def try_build_pattern(pattern: str, towels: list[str]) -> list[list[str]]:
    def towel_to_patterns(towel: str):
        if len(towel) >= len(pattern):
            return [[towel]]
        else:
            return [
                [towel, *ts] for ts in try_build_pattern(pattern[len(towel) :], towels)
            ]

    print("build", pattern, towels)
    usable_towels = [
        towel_to_patterns(towel) for towel in towels if pattern.startswith(towel)
    ]

    return list(chain.from_iterable(usable_towels))


def run():
    with open("19.inp") as rf:
        data = rf.read()

    towels = data.splitlines()[0].split(", ")

    patterns = data.splitlines()[2:]

    base_towels = set()
    for towel in towels:
        new_towels = deepcopy(towels)
        new_towels.remove(towel)

        buildable_from = try_build_pattern(towel, new_towels)

        if not buildable_from:
            base_towels.add(towel)
        else:
            for ts in buildable_from:
                for t in ts:
                    base_towels.add(t)

    doable = 0
    total = 0

    for pattern in patterns:
        n_patterns = len(try_build_pattern(pattern, base_towels))
        total += n_patterns

        print("done", pattern)
        if n_patterns > 0:
            doable += 1

    print("doable patterns: ", doable)

    print("new towels: ", len(base_towels), len(towels))


run()
