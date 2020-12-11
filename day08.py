from __future__ import annotations
from day import *

class Instruction(object):
	ACC = 0
	JMP = 1
	NOP = 2
	ids = {"acc": 0, "jmp": 1, "nop": 2}
	names = ["acc", "jmp", "nop"]

	def __init__(self, name: str, arg: int) -> None:
		self.id = Instruction.ids[name] if name in Instruction.ids else 2
		self.argument = arg

	def get_flipped(self) -> Instruction:
		if self.id == Instruction.JMP:
			return Instruction(Instruction.names[Instruction.NOP], self.argument)
		if self.id == Instruction.NOP:
			return Instruction(Instruction.names[Instruction.JMP], self.argument)
		return self
	
	@staticmethod
	def parse(line: str) -> Instruction:
		raw = line.strip().split(' ')
		return Instruction(raw[0], int(raw[1]))


class Day08(Day):
	day = 8
	title = "Handheld Halting"

	def setup(self, lines: List[str]) -> None:
		self.code = []
		self.switchers = []
		self.successful = False
		self.code = [Instruction.parse(line) for line in lines]

	def execute(self, code: List[Instruction]) -> int:
		first_run = len(self.switchers) == 0
		visited = set()
		accumulator = 0
		pointer = 0

		while pointer not in visited:
			if pointer >= len(code):
				self.successful = True
				return accumulator

			visited.add(pointer)
			instruction = code[pointer]

			if first_run and instruction.id in [Instruction.JMP, Instruction.NOP]:
				self.switchers.append(pointer)

			if instruction.id == Instruction.ACC:
				accumulator += instruction.argument
				pointer += 1
			elif instruction.id == Instruction.JMP:
				pointer += instruction.argument
			elif instruction.id == Instruction.NOP:
				pointer += 1

		return accumulator

	def part1(self) -> int:
		return self.execute(self.code)

	def part2(self) -> int:
		for i in self.switchers:
			left = self.code[:i]
			left.append(self.code[i].get_flipped())
			right = self.code[i + 1:]
			new = left + right

			accumulator = self.execute(new)
			if (self.successful):
				return accumulator

		return -1


if __name__ == "__main__":
	d = Day08()
	d.test()

