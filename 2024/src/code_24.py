from copy import deepcopy


def sim(
    states: dict[str, int], wires: dict[str, tuple[str, str, str]]
) -> dict[str, int]:
    states = deepcopy(states)

    while any(v is None for k, v in states.items() if k[0] == "z"):
        for out, (in1, op, in2) in wires.items():
            if states[in1] is not None and states[in2] is not None:
                if op == "AND":
                    states[out] = states[in1] and states[in2]
                elif op == "OR":
                    states[out] = states[in1] or states[in2]
                elif op == "XOR":
                    states[out] = states[in1] ^ states[in2]
                else:
                    raise ValueError(f"{op} not known")

    return states


def run(inp: str):
    def none_state(v):
        if v not in states:
            states[v] = None

    states: dict[str, int] = {}
    wires: dict[str, tuple[str, str, str]] = {}
    reading_states = True

    for line in inp.splitlines():
        if not line:
            reading_states = False
            continue

        if reading_states:
            data = line.split(": ")
            states[data[0]] = int(data[1])
        else:
            inn, out = line.split(" -> ")
            data = tuple(inn.split(" "))

            none_state(data[0])
            none_state(data[2])
            none_state(out)

            wires[out] = data

    def get_reg(name, exp):
        exp_inv = (exp[2], exp[1], exp[0])
        try:
            return next(k for k, v in wires.items() if v == exp or v == exp_inv)
        except StopIteration:
            print("fail to find", name, exp)
            raise

    # manually finding the errors using this script and writing them down, fuck automating this :)
    carry = None
    for i in range(0, 45):
        a_xor_b = get_reg(f"a_xor_b_{i}", (f"x{i:02}", "XOR", f"y{i:02}"))
        and_1 = get_reg(f"and_1_{i}", (f"x{i:02}", "AND", f"y{i:02}"))

        if carry:
            sum_i = get_reg(f"sum_i_{i}", (a_xor_b, "XOR", carry))

            if sum_i != f"z{i:02}":
                print(f"wrong sum_i_{i}:", sum_i, "is", (a_xor_b, "XOR", carry))
                raise Exception()

            and_2 = get_reg(f"and_2_{i}", (a_xor_b, "AND", carry))
            carry = get_reg(f"carry_{i}", (and_1, "OR", and_2))
        else:
            carry = and_1

    res_states = sim(states, wires)
    zs = sorted(((k, v) for k, v in res_states.items() if k[0] == "z"), reverse=True)
    n = int("".join(str(v) for _, v in zs), 2)

    print("result", n)


with open("24.inp") as rf:
    run(rf.read())
