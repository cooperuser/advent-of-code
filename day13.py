from day import *
from math import gcd

class Day13(Day):
	day = 13
	title = "Shuttle Search"

	def setup(self, lines: List[str]) -> None:
		self.earliest = int(lines[0].strip())
		self.busses = {}
		self.lcm = 1
		for index, raw in enumerate(lines[1].strip().split(',')):
			if raw != 'x':
				bus = int(raw)
				self.busses[bus] = index
				self.lcm = self.lcm * bus // gcd(self.lcm, bus)

	def part1(self) -> int:
		smallest = -1
		bus = 0
		for b in self.busses:
			times = self.earliest // b + 1
			wait = times * b - self.earliest
			if wait < smallest or smallest == -1:
				smallest = wait
				bus = b
		return bus * smallest

	def part2(self) -> int:
		lcm = 0
		offset = 0
		for bus, index in self.busses.items():
			if not index:
				lcm = bus
			for n in range(1, bus):
				if (offset + lcm*n - index) % bus == 0:
					offset += lcm*n
					lcm *= bus // gcd(lcm, bus)
					break
		return self.lcm - offset


if __name__ == "__main__":
	d = Day13()
	d.test()

