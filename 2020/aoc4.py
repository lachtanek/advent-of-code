import re
passports = []
hcl_pat = re.compile(r"^#[0-9a-f]{6}$")
pid_pat = re.compile(r"^[0-9]{9}$")

def validate_height(v):
    val, unit = v[0:-2], v[-2:]
    if unit == "cm":
        return 150 <= int(val) <= 193
    elif unit == "in":
        return 59 <= int(val) <= 76
    else:
        return False

fields = {
    "byr": lambda v: int(v) >= 1920 and int(v) <= 2002,
    "iyr": lambda v: int(v) >= 2010 and int(v) <= 2020,
    "eyr": lambda v: int(v) >= 2020 and int(v) <= 2030,
    "hgt": validate_height,
    "hcl": lambda v: hcl_pat.match(v) is not None,
    "ecl": lambda v: v in ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"],
    "pid": lambda v: pid_pat.match(v) is not None,
}
optionals = ["cid"]


with open("aoc4.inp") as rf:
    passport = {}
    for l in rf:
        if l != "\n":
            for data in l.strip().split(" "):
                k, v = data.split(":")
                passport[k] = v
        else:
            passports.append(passport)
            passport = {}


valids = 0
for p in passports:
    if all(k in p and validator(p[k]) for k, validator in fields.items()):
        valids += 1

print(valids)
