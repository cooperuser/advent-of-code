from day import *
from re import match

class Bag(object):
	def __init__(self, name: str):
		self.name: str = name
		self.bags: dict[str, int] = {}

	def addBag(self, name: str, count: int):
		self.bags[name] = count

	def canHold(self, name: str, bags: dict):
		if name in self.bags:
			return True
		for bag in self.bags:
			if bags[bag].canHold(name, bags):
				return True
		return False

	def countBags(self, bags: dict):
		total = 1
		for bag in self.bags:
			total += self.bags[bag] * bags[bag].countBags(bags)
		return total


class Day07(Day):
	day = 7
	title = "Handy Haversacks"
	patternLine = "([a-z]+ [a-z]+) bags contain (.+)."
	patternBag = "([0-9]+) ([a-z]+ [a-z]+)"
	target = "shiny gold"

	def setup(self, lines: List[str]) -> None:
		self.bags: dict[str, Bag] = {}
		for line in lines:
			g = match(Day07.patternLine, line.strip()).groups()
			contents = g[1]
			bag = Bag(g[0])
			self.bags[g[0]] = bag
			if contents == "no other bags":
				continue
			for bagString in contents.split(", "):
				b = match(Day07.patternBag, bagString).groups()
				bag.addBag(b[1], int(b[0]))

	def part1(self) -> int:
		able = 0
		for name in self.bags:
			if self.bags[name].canHold(Day07.target, self.bags):
				able += 1
		return able

	def part2(self) -> int:
		return self.bags[Day07.target].countBags(self.bags) - 1


if __name__ == "__main__":
	d = Day07()
	d.test()

