nums = []

with open("aoc10.inp") as rf:
    for l in rf:
        nums.append(int(l))


prev = 0
diffs = {1: 0, 2: 0, 3: 0}
nums.append(max(nums) + 3)
for n in sorted(nums):
    diffs[n-prev] += 1
    prev = n

print(diffs)
