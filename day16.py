from typing_extensions import TypeAlias
from day import *
from typing import Dict, Set

PATTERN = "^(.+): ([0-9]+)-([0-9]+) or ([0-9]+)-([0-9]+)$"
TARGETS = "departure"

Bounds: TypeAlias = "List[int]"

def in_range(rule: Bounds, value: int) -> bool:
	if value >= rule[0] and value <= rule[1]:
		return True
	if value >= rule[2] and value <= rule[3]:
		return True
	return False


class Day16(Day):
	day = 16
	title = "Ticket Translation"

	def setup(self, lines: List[str]) -> None:
		self.rules: Dict[str, Bounds] = {}
		self.ticket = []
		self.nearby = []
		self.valid = {}
		i = 0
		while True:
			line = lines[i]
			if line == "\n":
				i += 2
				break
			m = re.match(PATTERN, line)
			g = m.groups()
			self.rules[g[0]] = [int(num) for num in g[1:]]
			i += 1
		self.ticket = [int(num) for num in lines[i].strip().split(',')]
		for j, other in enumerate(lines[i+3:]):
			ticket = [int(num) for num in other.strip().split(',')]
			self.valid[j] = ticket
			self.nearby.append(ticket)
		self.indices = len(self.ticket)

	def get_product(self, indices: Dict[str, int]) -> int:
		product = 1
		for name, index in indices.items():
			if name.startswith(TARGETS):
				product *= self.ticket[index]
		return product

	def part1(self) -> int:
		total = 0
		for ticket_id, ticket in enumerate(self.nearby):
			for number in ticket:
				for rule in self.rules.values():
					if in_range(rule, number):
						break
				else:
					total += number
					del self.valid[ticket_id]
					break
		return total

	def part2(self) -> int:
		possible: Dict[str, Set[int]] = {}
		for name, rule in self.rules.items():
			possible[name] = set()
			for index in range(self.indices):
				for ticket in self.valid.values():
					if not in_range(rule, ticket[index]):
						break
				else:
					possible[name].add(index)

		counts: Dict[int, Set[str]] = {}
		for name, valid in possible.items():
			if (length := len(valid)) not in counts:
				counts[length] = set()
			counts[length].add(name)

		actual: Dict[str, int] = {}
		used: Set[int] = set()
		for i in range(len(counts)):
			names = counts[i + 1]
			for name in names:
				for index in possible[name]:
					if index in used:
						continue
					used.add(index)
					actual[name] = index
					break

		return self.get_product(actual)


if __name__ == "__main__":
	d = Day16()
	d.test()

