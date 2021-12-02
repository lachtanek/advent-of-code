nums = []

with open("aoc1.inp") as rf:
    for l in rf:
        nums.append(int(l))


l = len(nums)
for j in range(l):
    for k in range(l):
        if j != k and nums[j] + nums[k] == 2020:
            print(nums[j], nums[k])
            break


l = len(nums)
for i in range(l):
    for j in range(l):
        for k in range(l):
            if i != j and j != k and nums[i] + nums[j] + nums[k] == 2020:
                print(nums[i] * nums[j] * nums[k])
                break
