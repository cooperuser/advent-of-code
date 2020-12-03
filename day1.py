file = open("data/day1a", 'r')
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

if __name__ == "__main__":
    print(part1())

