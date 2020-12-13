from __future__ import annotations
from copy import copy, deepcopy
from day import *

class Vector(object):
	def __init__(self, x: int = 0, y:int = 0):
		self.x = x
		self.y = y

	def distance(self) -> int:
		return abs(self.x) + abs(self.y)

	def rotate(self, amount: int) -> None:
		turns = amount % 4
		if turns == 1:
			self.x, self.y = -self.y, self.x
		if turns == 2:
			self.x, self.y = -self.x, self.y
		if turns == 3:
			self.x, self.y = self.y, -self.x

	def clone(self) -> Vector:
		return Vector(self.x, self.y)

	def __add__(self, other: Vector) -> Vector:
		return Vector(self.x + other.x, self.y + other.y)

	def __mul__(self, other: int) -> Vector:
		return Vector(self.x * other, self.y * other)

	def __repr__(self) -> str:
		return "Vector({}, {})".format(self.x, self.y)


class Direction:
	NORTH = Vector(0, 1)
	SOUTH = Vector(0, -1)
	EAST = Vector(1, 0)
	WEST = Vector(-1, 0)


DIRECTION_MAPPING = {
	'N': Direction.NORTH,
	'S': Direction.SOUTH,
	'E': Direction.EAST,
	'W': Direction.WEST
}

class Day12(Day):
	day = 12
	title = "Rain Risk"

	def setup(self, lines: List[str]) -> None:
		self.instructions = []
		for line in lines:
			a = line[0]
			b = int(line[1:].strip())
			self.instructions.append((a, b))

	def both(self, position: Vector, waypoint: Vector) -> int:
		part1 = waypoint.distance() == 1
		for command, argument in self.instructions:
			if command in DIRECTION_MAPPING:
				dir = DIRECTION_MAPPING[command] * argument
				if part1:
					position += dir
				else:
					waypoint += dir
			elif command == "L":
				waypoint.rotate(argument // 90)
			elif command == "R":
				waypoint.rotate(-argument // 90)
			elif command == "F":
				position += waypoint * argument
		return position.distance()

	def part1(self) -> int:
		return self.both(Vector(0, 0), Direction.EAST.clone())

	def part2(self) -> int:
		return self.both(Vector(0, 0), Vector(10, 1))


if __name__ == "__main__":
	d = Day12()
	d.test()

