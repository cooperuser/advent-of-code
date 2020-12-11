from typing_extensions import TypeAlias, Tuple
from day import *
from copy import deepcopy

SeatingChart: TypeAlias = "List[List[int]]"
Neighborhood: TypeAlias = "List[List[List[Tuple[int]]]]"
IDS = {'.': 0, 'L': 1, '#': 2}
DIRECTIONS = [
	[-1, -1], [0, -1], [1, -1],
	[-1,  0],          [1,  0],
	[-1,  1], [0,  1], [1,  1]
]

class Day11(Day):
	day = 11
	title = "Seating System"

	def setup(self, lines: List[str]) -> None:
		self.seats = []
		for line in lines:
			self.seats.append([IDS[c] for c in line.strip()])
		self.width = len(self.seats[0])
		self.height = len(self.seats)
		self.adjacencies = []
		self.sightlines = []
		self.set_neighbors()
	
	def out_of_bounds(self, x, y) -> bool:
		return x < 0 or x >= self.width or y < 0 or y >= self.height

	def set_neighbors(self) -> None:
		for y in range(self.height):
			adjacencies_row = []
			sightlines_row = []
			for x in range(self.width):
				adjacencies = []
				sightlines = []
				for dir in DIRECTIONS:
					for distance in range(1, max(self.width, self.height)):
						dx = x + dir[0] * distance
						dy = y + dir[1] * distance
						if self.out_of_bounds(dx, dy):
							break
						if self.seats[dy][dx]:
							spot = (dx, dy)
							sightlines.append(spot)
							if distance == 1:
								adjacencies.append(spot)
							break
				adjacencies_row.append(adjacencies)
				sightlines_row.append(sightlines)
			self.adjacencies.append(adjacencies_row)
			self.sightlines.append(sightlines_row)
	
	def get_seats(self, neighbors: Neighborhood, limit: int) -> SeatingChart:
		seats = deepcopy(self.seats)
		changed = True
		while changed:
			changed = False
			new = []
			for y in range(self.height):
				new.append([])
				for x in range(self.width):
					old = seats[y][x]
					new[y].append(old)
					if not old:
						continue
					count = 0
					for spot in neighbors[y][x]:
						if seats[spot[1]][spot[0]] == 2:
							count += 1
					if old == 1 and count == 0:
						new[y][x] = 2
						changed = True
					elif old == 2 and count >= limit:
						new[y][x] = 1
						changed = True
			seats = new
		return seats
	
	def part1(self) -> int:
		return Day11.count_occupied(self.get_seats(self.adjacencies, 4))
	
	def part2(self) -> int:
		return Day11.count_occupied(self.get_seats(self.sightlines, 5))

	@staticmethod
	def count_occupied(seats: SeatingChart) -> int:
		occupied = 0
		for row in seats:
			for seat in row:
				occupied += seat == 2
		return occupied


if __name__ == "__main__":
	d = Day11()
	d.test()

