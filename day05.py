from day import Day
from re import match

class Day05(Day):
	day = 5
	title = "Binary Boarding"
	pattern = "([BF]{7})([LR]{3})"

	def setup(self, lines) -> None:
		self.lines = lines
		self.min = -1
		self.max = 0
		self.seats = set()
		for line in self.lines:
			groups = match(Day05.pattern, line.strip()).groups()
			a = groups[0].replace('F', '0').replace('B', '1')
			b = groups[1].replace('L', '0').replace('R', '1')
			row = int(a, 2)
			col = int(b, 2)
			value = row * 8 + col
			self.seats.add(value)
			if value > self.max:
				self.max = value
			if self.min == -1 or value < self.min:
				self.min = value

	def part1(self) -> int:
		return self.max

	def part2(self) -> int:
		for seat in self.seats:
			if seat == self.min or seat == self.max:
				continue
			if seat + 1 not in self.seats:
				return seat + 1
			if seat - 1 not in self.seats:
				return seat - 1
		return -1


if __name__ == "__main__":
	d = Day05()
	d.run()

