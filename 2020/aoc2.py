nums = []


def parse_policy(policy):
    counts, letter = policy.split(" ")
    return (letter, *counts.split("-"))


def check_password(policy, password):
    letter, mmin, mmax = parse_policy(policy)
    letter_times = password.count(letter)
    return letter_times >= int(mmin) and letter_times <= int(mmax)


def check_password_new(policy, password):
    letter, pos1, pos2 = parse_policy(policy)
    l1, l2 = password[int(pos1) - 1], password[int(pos2) - 1]
    return (l1 == letter or l2 == letter) and l1 != l2


valid = new_valid = 0
with open("aoc2.inp") as rf:
    for l in rf:
        data = l.strip().split(": ")
        if check_password(*data):
            valid += 1

        if check_password_new(*data):
            new_valid += 1


print(valid, new_valid)
