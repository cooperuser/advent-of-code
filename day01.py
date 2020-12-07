from day import Day

class Day01(Day):
	day = 1
	title = "Report Repair"

	def setup(self, lines) -> None:
		self.numbers = [int(line) for line in lines]
		self.length = len(self.numbers)

	def part1(self) -> int:
		for i in range(self.length):
			a = self.numbers[i]
			b = 2020 - a
			if b in self.numbers:
				return a * b
		return -1

	def part2(self) -> int:
		for i in range(self.length):
			for j in range(i + 1, self.length):
				a = self.numbers[i]
				b = self.numbers[j]
				c = 2020 - a - b
				if c in self.numbers:
					return a * b * c
		return -1


if __name__ == "__main__":
	d = Day01()
	d.run()

