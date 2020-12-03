from re import match

file = open("data/day2", 'r')
lines = file.readlines()
pattern = "([0-9]+)-([0-9]+) ([a-z]): ([a-z]+)"

def part1():
    valid = 0
    for line in lines:
        g = match(pattern, line).groups()
        count = g[3].count(g[2])
        if count >= int(g[0]) and count <= int(g[1]):
            valid += 1
    return valid

def part2():
    pass

if __name__ == "__main__":
    print(part1())
    print(part2())

