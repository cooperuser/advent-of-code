file = open("data/day1", 'r')
numbers = [int(line) for line in file.readlines()]
length = len(numbers)

def part1():
    for i in range(length):
        for j in range(i + 1, length):
            a = numbers[i]
            b = numbers[j]
            if a + b == 2020:
                return a * b
    return -1

def part2():
    for i in range(length):
        for j in range(i + 1, length):
            for k in range(j + 1, length):
                a = numbers[i]
                b = numbers[j]
                c = numbers[k]
                if a + b + c == 2020:
                    return a * b * c
    return -1

if __name__ == "__main__":
    print(part1())
    print(part2())

