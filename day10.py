from day import *

class Day10(Day):
	day = 10
	title = "Adapter Array"

	def setup(self, lines: List[str]) -> None:
		self.cache = {len(lines): 1}
		self.adapters = set()
		self.sorted = [0]
		self.maximum = 0
		for line in lines:
			number = int(line.strip())
			self.adapters.add(number)
			if number > self.maximum:
				self.maximum = number

	def part1(self) -> int:
		differences = [0, 0, 1]
		current = 0
		while current < self.maximum:
			for i in range(1, 4):
				if current + i in self.adapters:
					current += i
					self.sorted.append(current)
					differences[i - 1] += 1
					break
		return differences[0] * differences[2]

	def part2(self, index: int = 0) -> int:
		if index in self.cache:
			return self.cache[index]
		successors = self.sorted[index + 1:index + 4]
		count = 0
		number = self.sorted[index] + 1
		for target in range(number, number + 3):
			for s in range(len(successors)):
				if target == successors[s]:
					count += self.part2(index + s + 1)
		self.cache[index] = count
		return count


if __name__ == "__main__":
	d = Day10()
	d.run()

