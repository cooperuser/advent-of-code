from typing import Dict, Tuple
from typing_extensions import TypeAlias
from day import *
from re import match

PATTERN = "(mask|mem)(\\[([0-9]*)\\])? = ([0-9X]+)"

class Mask(object):
	def __init__(self, raw: str = '0'):
		self.mask_and = int(raw.replace('X', '1'), 2)
		self.mask_or = int(raw.replace('X', '0'), 2)
		self.data = set()
		for i in range(len(raw)):
			if raw[i] == "X":
				self.data.add(35 - i)


MemorySetter : TypeAlias = "Tuple[int, int]"
Segment: TypeAlias = "Tuple[Mask, List[MemorySetter]]"

class Day14(Day):
	day = 14
	title = ""

	def setup(self, lines: List[str]) -> None:
		self.segments: List[Segment] = []
		segment: Segment = (Mask(), [])
		for line in lines:
			m = match(PATTERN, line)
			g = m.groups()  # [command, None, address, value]
			if g[0] == "mask":
				self.segments.append(segment)
				segment = (Mask(g[3]), [])
			else:
				segment[1].append((int(g[2]), int(g[3])))
		self.segments.append(segment)
		self.segments = self.segments[1:]

	def part1(self) -> int:
		memory = {}
		for mask, items in self.segments:
			for address, value in items:
				memory[address] = value & mask.mask_and | mask.mask_or
		return Day14.sum_dict(memory)

	def part2(self) -> int:
		memory = {}
		for mask, items in self.segments:
			for address, value in items:
				addresses = set([address | mask.mask_or])
				for bit in mask.data:
					new = set()
					for addr in addresses:
						new.add(addr & (addr ^ 2**bit))
						new.add(addr | 2**bit)
					addresses = new
				for addr in addresses:
					memory[addr] = value
		return Day14.sum_dict(memory)

	@staticmethod
	def sum_dict(dict: Dict):
		total = 0
		for value in dict.values():
			total += value
		return total


if __name__ == "__main__":
	d = Day14()
	d.test()

