from day import *

class Day03(Day):
	day = 3
	title = "Toboggan Trajectory"

	def setup(self, lines: List[str]) -> None:
		self.map = []
		for line in lines:
			self.map.append([c == '#' for c in line.strip()])
		self.width = len(self.map[0])
		self.height = len(self.map)

	def countTrees(self, slope) -> int:
		count = 0
		x = 0
		dx = slope[0]
		dy = slope[1]
		for i in range(dy, self.height, dy):
			line = self.map[i]
			x = (x + dx) % self.width
			if line[x]:
				count += 1
		return count

	def part1(self) -> int:
		return self.countTrees([3, 1])

	def part2(self) -> int:
		slopes = [[1, 1], [3, 1], [5, 1], [7, 1], [1, 2]]
		product = 1
		for slope in slopes:
			product *= self.countTrees(slope)
		return product


if __name__ == "__main__":
	d = Day03()
	d.test()

