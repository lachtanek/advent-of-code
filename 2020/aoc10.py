nums = []

with open("aoc10.inp") as rf:
    for l in rf:
        nums.append(int(l))


prev = 0
diffs = {1: 0, 2: 0, 3: 0}
nums.append(max(nums) + 3)
nums = sorted(nums)
arrs = {}
for i, n in enumerate(nums):
    arrs[n] = [x for x in nums[i+1:i+4] if x - n <= 3]

print(len(nums), sum([1 for k, v in arrs.items() if len(v) == 1]))
