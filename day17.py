from __future__ import annotations
from typing_extensions import TypeAlias
from day import *
from typing import Set, Dict, Tuple

Vector: TypeAlias = "Tuple[int, int, int, int]"

def vec_add(a: Vector, b: Vector) -> Vector:
	return (a[0] + b[0], a[1] + b[1], a[2] + b[2], a[3] + b[3])

def vec_iter_adj(vector: Vector, cache: Dict[Vector, Set[Vector]],
				 adjacencies: Set[Vector]) -> Set[Vector]:
	if vector in cache:
		return cache[vector]
	neighbors = set()
	for offset in adjacencies:
		neighbors.add(vec_add(vector, offset))
	cache[vector] = neighbors
	return neighbors

ADJ3: Set[Vector] = set()
ADJ4: Set[Vector] = set()
for x in range(-1, 2):
	for y in range(-1, 2):
		for z in range(-1, 2):
			for w in range(-1, 2):
				if x == 0 and y == 0 and z == 0 and w == 0:
					continue
				if w == 0:
					ADJ3.add((x, y, z, 0))
				ADJ4.add((x, y, z, w))

class Day17(Day):
	day = 17
	title = "Conway Cubes"

	def setup(self, lines: List[str]) -> None:
		self.size = len(lines[0].strip())
		self.map = set()
		for j in range(len(lines)):
			line = lines[j]
			for i in range(len(line.strip())):
				if line[i] == '#':
					self.map.add((i, j, 0, 0))
	
	def run_simulation(self, adjacencies: Set[Vector], iterations: int):
		cache: Dict[Vector, Set[Vector]] = {}
		last = self.map
		for _ in range(iterations):
			new: Set[Vector] = set()
			left: Set[Vector] = set()

			for active in last:
				neighbors = 0
				for adj in vec_iter_adj(active, cache, adjacencies):
					if adj in last:
						neighbors += 1
					else:
						left.add(adj)
				if neighbors == 2 or neighbors == 3:
					new.add(active)

			for inactive in left:
				neighbors = 0

				for adj in vec_iter_adj(inactive, cache, adjacencies):
					neighbors += adj in last
				if neighbors == 3:
					new.add(inactive)

			last = new
		return len(last)

	def part1(self) -> int:
		return self.run_simulation(ADJ3, 6)

	def part2(self) -> int:
		return self.run_simulation(ADJ4, 6)


if __name__ == "__main__":
	d = Day17()
	d.test()

