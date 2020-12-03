from day import Day
from re import match

class Day02(Day):
	day = 2
	title = "Password Philosophy"

	def setup(self, lines) -> None:
		self.lines = lines
		self.pattern = "([0-9]+)-([0-9]+) ([a-z]): ([a-z]+)"

	def part1(self) -> int:
		valid = 0
		for line in self.lines:
			g = match(self.pattern, line).groups()
			count = g[3].count(g[2])
			if count >= int(g[0]) and count <= int(g[1]):
				valid += 1
		return valid

	def part2(self) -> int:
		valid = 0
		for line in self.lines:
			g = match(self.pattern, line).groups()
			a = int(g[0]) - 1
			b = int(g[1]) - 1
			character = g[2]
			password = g[3]
			if (password[a] == character) != (password[b] == character):
				valid += 1
		return valid


if __name__ == "__main__":
	d = Day02()
	d.run()

