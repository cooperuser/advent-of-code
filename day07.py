from day import Day
from re import match

bags = {}

class Bag(object):
	def __init__(self, name):
		self.name = name
		self.bags = {}
		self.checked = False

	def addBag(self, name, count):
		self.bags[name] = count

	def canHold(self, name):
		if name in self.bags:
			return True
		for bag in self.bags:
			if bags[bag].canHold(name):
				return True
		return False

	def countBags(self):
		total = 1
		for bag in self.bags:
			total += self.bags[bag] * bags[bag].countBags()
		return total



class Day07(Day):
	day = 7
	title = "Handy Haversacks"
	pattern = "([a-z]+ [a-z]+) bags contain (.+)."
	bagPattern = "([0-9]+) ([a-z]+ [a-z]+)"
	target = "shiny gold"

	def setup(self, lines) -> None:
		for line in lines:
			g = match(Day07.pattern, line.strip()).groups()
			contents = g[1]
			bag = Bag(g[0])
			bags[g[0]] = bag
			if contents == "no other bags":
				continue
			for bagString in contents.split(", "):
				b = match(Day07.bagPattern, bagString).groups()
				bag.addBag(b[1], int(b[0]))

	def part1(self) -> int:
		able = set()
		queue = []
		for bag in bags:
			if bags[bag].canHold(Day07.target):
				able.add(bag)
		return len(able)

	def part2(self) -> int:
		return bags[Day07.target].countBags() - 1


if __name__ == "__main__":
	d = Day07()
	d.run()

