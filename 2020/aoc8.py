program = []


def parse_instr(data):
    instr, arg = data.split(" ")
    return instr, int(arg[1:]) * (1 if arg[0] == "+" else -1)


with open("aoc8.inp") as rf:
    for l in rf:
        program.append([parse_instr(l.strip()), 0])

ic = 0
acc = 0
while ic < len(program):
    instr, arg = program[ic][0]

    if program[ic][1] > 3:
        print(list(filter(lambda x: x[1] > 2, program)))
        print(program[ic])
        print(acc)
        raise SystemExit(1)

    if instr == "acc":
        acc += arg
        ic += 1
    elif instr == "jmp":
        ic += arg
    elif instr == "nop":
        ic += 1

    print(program[ic])
    if ic < len(program):
        program[ic][1] += 1

print(acc)
