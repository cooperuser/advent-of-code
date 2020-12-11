from day import *
from copy import deepcopy

class Day11(Day):
	day = 11
	title = "Seating System"

	def setup(self, lines: List[str]) -> None:
		self.original = []
		for line in lines:
			row = []
			for c in line.strip():
				if c == ".":
					row.append(0)
				elif c == "L":
					row.append(1)
				elif c == "#":
					row.append(2)
			self.original.append(row)
		self.seats = deepcopy(self.original)
		self.width = len(self.seats[0])
		self.height = len(self.seats)

	def part1(self) -> int:
		changes = 1
		while changes != 0:
			changes = 0
			new = []
			for y in range(self.height):
				new.append([])
				for x in range(self.width):
					new[y].append(self.seats[y][x])
					adjacent = 0
					for j in range(-1, 2):
						for i in range(-1, 2):
							if j == 0 and i == 0:
								continue
							dy = y + j
							dx = x + i
							if dy < 0 or dy >= self.height or dx < 0 or dx >= self.width:
								continue
							adjacent += 1 if self.seats[dy][dx] == 2 else 0
					if self.seats[y][x] == 1 and adjacent == 0:
						new[y][x] = 2
						changes += 1
					elif self.seats[y][x] == 2 and adjacent >= 4:
						new[y][x] = 1
						changes += 1
			self.seats = new
		occupied = 0
		for y in range(self.height):
			for x in range(self.width):
				if self.seats[y][x] == 2:
					occupied += 1
		return occupied

	def part2(self) -> int:
		self.seats = deepcopy(self.original)
		changes = 1
		while changes != 0:
			changes = 0
			new = []
			for y in range(self.height):
				new.append([])
				for x in range(self.width):
					new[y].append(self.seats[y][x])
					adjacent = 0
					for j in range(-1, 2):
						for i in range(-1, 2):
							if j == 0 and i == 0:
								continue
							distance = 1
							while True:
								dy = y + j * distance
								dx = x + i * distance
								if dy < 0 or dy >= self.height or dx < 0 or dx >= self.width:
									break
								if self.seats[dy][dx] == 1:
									break
								if self.seats[dy][dx] == 2:
									adjacent += 1
									break
								distance += 1
					if self.seats[y][x] == 1 and adjacent == 0:
						new[y][x] = 2
						changes += 1
					elif self.seats[y][x] == 2 and adjacent >= 5:
						new[y][x] = 1
						changes += 1
			self.seats = new
		occupied = 0
		for y in range(self.height):
			for x in range(self.width):
				if self.seats[y][x] == 2:
					occupied += 1
		return occupied


if __name__ == "__main__":
	d = Day11()
	d.run()

