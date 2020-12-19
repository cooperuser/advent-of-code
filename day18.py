from __future__ import annotations
from typing_extensions import TypeAlias
from day import *

Problem: TypeAlias = 'str | List[Problem]'

def parse_problem(problem: Problem) -> Problem:
	length = len(problem)
	out: List[Problem] = []
	i = 0
	while i < length:
		c = problem[i]
		if c.isnumeric():
			out.append(c)
		elif c == '(':
			depth = 1
			end = 0
			for j in range(i + 1, length):
				e = problem[j]
				if e == '(':
					depth += 1
				elif e == ')':
					depth -= 1
				if depth == 0:
					end = j
					break
			out.append(parse_problem(problem[i + 1 : end]))
			i = end
		else:
			out.append(c)
		i += 1
	return out

def wrap_addition(problem: Problem) -> Problem:
	if type(problem) == str:
		return problem
	length = len(problem)
	if length <= 3:
		return problem
	out: List[Problem] = []
	out = [wrap_addition(p) for p in problem]
	i = 1
	while i < length:
		if out[i] == '+':
			out[i - 1 : i + 2] = [[out[i - 1], '+', out[i + 1]]]
			length -= 2
		else:
			i += 2
	return out

class Day18(Day):
	day = 18
	title = "Operation Order"

	def setup(self, lines: List[str]) -> None:
		self.problems = []
		self.advanced = []
		for line in lines:
			problem = parse_problem(line.strip().replace(' ', ''))
			self.problems.append(problem)
			self.advanced.append(wrap_addition(problem))
	
	def solve(self, problem):
		answer: int = 0
		if type(problem[0]) != str:
			answer = self.solve(problem[0])
		else:
			answer = int(problem[0])

		i = 1
		while i < len(problem):
			c = problem[i]
			if c == '*':
				answer *= self.solve(problem[i + 1])
			elif c == '+':
				answer += self.solve(problem[i + 1])
			i += 2

		return answer

	def part1(self) -> int:
		total = 0
		for problem in self.problems:
			total += self.solve(problem)
		return total

	def part2(self) -> int:
		total = 0
		for problem in self.advanced:
			total += self.solve(problem)
		return total


if __name__ == "__main__":
	d = Day18()
	d.test()

