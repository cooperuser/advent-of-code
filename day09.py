from day import *

class Day09(Day):
	day = 9
	title = "Encoding Error"

	def setup(self, lines: List[str]) -> None:
		self.preamble = 25
		if self.parameters:
			self.preamble = int(self.parameters[0])
		self.lines = [int(line.strip()) for line in lines]
	
	def find_error(self, preamble: int):
		queue = []
		pointer = 0
		for _ in range(preamble):
			queue.append(self.lines[pointer])
			pointer += 1
		for _ in range(len(self.lines) - preamble):
			if not Day09.is_sum(self.lines[pointer], queue):
				return pointer
			queue.pop(0)
			queue.append(self.lines[pointer])
			pointer += 1
		return -1

	def part1(self) -> int:
		self.index = self.find_error(self.preamble)
		self.answer = self.lines[self.index]
		return self.answer

	def part2(self) -> int:
		low = 0
		high = 1
		total = self.lines[low]
		while total < self.answer:
			total += self.lines[high]
			high += 1
			while total > self.answer:
				total -= self.lines[low]
				low += 1
			if total == self.answer:
				subarray = self.lines[low:high]
				return min(subarray) + max(subarray)
		return -1

	@staticmethod
	def is_sum(number: int, queue: List[int]) -> bool:
		for i in range(len(queue) - 1):
			if number - queue[i] in queue:
				return True
		return False
	

if __name__ == "__main__":
	d = Day09()
	d.test()

