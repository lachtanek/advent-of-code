import re

rgx = re.compile(r"(\(|\)|\*|\+|[\w]+)")
data = []
OP_PLUS = 0
OP_MUL = 1
NUM = 2
GROUP = 3


def parse_expr(expr):
    ops = []

    # for i, c in enumerate(expr):
    i = 0
    while i < len(expr):
        c = expr[i]
        if c == "+":
            ops.append((OP_PLUS, None))
        elif c == "*":
            ops.append((OP_MUL, None))
        elif c in "0123456789":
            ops.append((NUM, int(c)))
        elif c == "(":
            result, pos = parse_expr(expr[i + 1 :])
            i += pos + 1
            ops.append((GROUP, result))
        elif c == ")":
            return ops, i

        i += 1

    return ops


def eval_expr(expr):
    new = []
    for t, x in expr:
        if t == GROUP:
            new.append((NUM, eval_expr(x)))
        else:
            new.append((t, x))

    _, base = new[0]
    assert _ == NUM

    for i in range(1, len(new), 2):
        op, x = new[i : i + 2]
        if op[0] == OP_PLUS:
            assert x[0] == NUM
            base += x[1]
        elif op[0] == OP_MUL:
            assert x[0] == NUM
            base *= x[1]
        else:
            assert False

    return base


def eval_expr2(expr):
    expr = prepare_expr(expr)
    new = []
    for t, x in expr:
        if t == GROUP:
            new.append((NUM, eval_expr2(prepare_expr(x))))
        else:
            new.append((t, x))

    _, base = new[0]
    assert _ == NUM

    for i in range(1, len(new), 2):
        op, x = new[i : i + 2]
        if op[0] == OP_PLUS:
            assert x[0] == NUM
            base += x[1]
        elif op[0] == OP_MUL:
            assert x[0] == NUM
            base *= x[1]
        else:
            assert False

    return base


# 1 + 2 * 3 -> (1 + 2) * 3
# 1 * 2 + 3 * 4 -> 1 * (2 + 3) * 4
def prepare_expr(expr):
    new = []

    assert len(expr) % 2 == 1

    if len(expr) == 1:
        return expr

    i = 1
    while i < len(expr):
        if expr[i][0] == OP_MUL:
            return [
                (GROUP, prepare_expr(expr[:i])),
                (OP_MUL, None),
                (GROUP, prepare_expr(expr[i + 1 :])),
            ]
        else:
            i += 2

    return expr


s = 0
exprs = []
with open("aoc18.inp") as rf:
    for l in rf:
        data = parse_expr(l.strip())
        exprs.append(data)

print(sum(map(eval_expr, exprs)))
print(sum(map(eval_expr2, exprs)))
