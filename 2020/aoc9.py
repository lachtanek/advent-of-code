nums = []

with open("aoc9.inp") as rf:
    for l in rf:
        nums.append(int(l))


def is_sum(n, of):
    for x in of:
        for y in of[1:]:
            if x != y and x + y == n:
                return True

    return False


def can_be_sum(n, ns):
    for j, x in enumerate(ns):
        n -= x
        if n == 0:
            rng = ns[:j]
            print(min(rng) + max(rng))
            raise SystemExit(0)


for i, num in enumerate(nums[25:]):
    if not is_sum(num, nums[i:i+25]):
        mynum = num
        print(i, num)
        break


for k in range(i - 1):
    can_be_sum(mynum, nums[k:min(i - 1, k + 20)])
