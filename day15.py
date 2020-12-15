from day import *
from copy import deepcopy

class Day15(Day):
	day = 15
	title = "Rambunctious Recitation"

	def setup(self, lines: List[str]) -> None:
		self.numbers = {}
		self.last = -1
		data = [int(c) for c in lines[0].split(',')]
		for i in range(len(data)):
			self.numbers[data[i]] = i + 1
			self.last = data[i]
	
	def play_game(self, turns: int) -> int:
		numbers = deepcopy(self.numbers)
		last = self.last
		for i in range(len(numbers), turns):
			last_time = numbers[last] if last in numbers else i
			numbers[last] = i
			last = i - last_time
		return last

	def part1(self) -> int:
		return self.play_game(2020)

	def part2(self) -> int:
		return self.play_game(30000000)


if __name__ == "__main__":
	d = Day15()
	d.test()

