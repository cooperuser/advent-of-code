from day import *

class Day06(Day):
	day = 6
	title = "Custom Customs"
	alphabet = "abcdefghijklmnopqrstuvwxyz"

	def setup(self, lines: List[str]) -> None:
		self.groups = []
		group = []
		for line in lines:
			if line.strip() == "":
				self.groups.append(group)
				group = []
				continue
			group.append(line.strip())
		self.groups.append(group)

	def part1(self) -> int:
		count = 0
		for group in self.groups:
			combined = "".join(group)
			unique = set()
			for c in Day06.alphabet:
				if c in combined:
					unique.add(c)
			count += len(unique)
		return count

	def part2(self) -> int:
		count = 0
		for group in self.groups:
			unique = set()
			for c in Day06.alphabet:
				for person in group:
					if c not in person:
						break
				else:
					unique.add(c)
			count += len(unique)
		return count


if __name__ == "__main__":
	d = Day06()
	d.test()

