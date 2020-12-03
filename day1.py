from day import Day

class Day1(Day):
    day = 1
    title = "Report Repair"

    def setup(self, lines) -> None:
        self.numbers = [int(line) for line in lines]
        self.length = len(self.numbers)

    def part1(self) -> int:
        for i in range(self.length):
            for j in range(i + 1, self.length):
                a = self.numbers[i]
                b = self.numbers[j]
                if a + b == 2020:
                    return a * b
        return -1

    def part2(self) -> int:
        for i in range(self.length):
            for j in range(i + 1, self.length):
                for k in range(j + 1, self.length):
                    a = self.numbers[i]
                    b = self.numbers[j]
                    c = self.numbers[k]
                    if a + b + c == 2020:
                        return a * b * c
        return -1


if __name__ == "__main__":
    d = Day1()
    d.run()

