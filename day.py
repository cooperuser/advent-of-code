from typing import List
from termcolor import colored
import re
import time

SEPARATOR = "-- PART1 TEST INPUT ABOVE, PART2 TEST INPUT BELOW --\n"
PARAMETERS = "PARAMETERS: (.*)"
TEST = colored("-- TESTING --", "blue", attrs=["bold"])
PASS = colored(" PASS ", "grey", "on_green", attrs=["bold"])
FAIL = colored(" FAIL ", "white", "on_red", attrs=["bold"])
INFO = colored(" INFO ", "grey", "on_blue", attrs=["bold"])
QUIT = colored("Some tests failed, aborting.", attrs=["dark", "underline"])

def load_file(path: str) -> List[str]:
	lines = []
	try:
		file = open(path, 'r')
		lines = file.readlines()
		file.close()
	finally:
		return lines

def format_time(delay: float) -> str:
	ch = ""
	if delay < 10:
		delay *= 1000
		ch = 'μ'
	return "{:.2f}{}s".format(delay, ch)

def output_result(result: int, elapsed: float, text_day: str) -> None:
	if result != -1:
		print(INFO + text_day + "part1")
		print("\treceived: " + colored(str(result), "yellow"))
		print("\telapsed : " + colored(format_time(elapsed), attrs=["dark"]))
		print()

class Day(object):
	day = 0
	title = ""

	def __init__(self, parameters: List[str] = []):
		self.parameters = parameters

	def run(self) -> None:
		if not self.__class__.title:
			return

		print("Day {}: {}".format(self.__class__.day, self.__class__.title))

		lines = load_file("data/day" + str(self.__class__.day))
		self.setup(lines)

		part1 = self.part1()
		out1 = "NYI" if part1 == -1 else str(part1)
		print("\tPart 1: " + out1)
		part2 = self.part2()
		out2 = "NYI" if part2 == -1 else str(part2)
		print("\tPart 2: " + out2)

	def test(self) -> bool:
		test_lines = load_file("test/test" + str(self.__class__.day))
		parameters = []
		match = re.match(PARAMETERS, test_lines[0])
		if match:
			parameters = match.groups()[0].split(',')
			print(parameters)
			test_lines = test_lines[1:]
		if not test_lines:
			self.run()
			return False

		passing = 0
		title = self.__class__.title
		day = str(self.__class__.day)
		title = title if title else "Temporary Name"

		sep = test_lines.index(SEPARATOR)

		text_day = colored(" day{}.py/".format(day), attrs=["dark"])

		lines_part1 = test_lines[:sep - 1]
		if not lines_part1:
			self.run()
			return False
		expected_part1 = int(test_lines[sep - 1])
		raw_part2 = test_lines[-1]
		expected_part2 = int(raw_part2) if raw_part2.strip() else -1
		total = 0

		print(TEST + " Day {}: {}\n".format(self.__class__.day, title))

		tester = self.__class__(parameters)
		tester.setup(lines_part1)
		answer_part1 = tester.part1()
		if answer_part1 != -1 and expected_part1 != -1:
			total += 1
			if answer_part1 == expected_part1:
				print(PASS + text_day + "part1")
				print("\treceived: " + colored(str(answer_part1), "green"))
				passing += 1
			else:
				print(FAIL + text_day + "part1")
				print("\texpected: " + colored(str(expected_part1), "green"))
				print("\treceived: " + colored(str(answer_part1), "red"))
			print()

		answer_part2 = tester.part2()
		if answer_part2 != -1 and expected_part2 != -1:
			total += 1
			if answer_part2 == expected_part2:
				print(PASS + text_day + "part2")
				print("\treceived: " + colored(str(answer_part2), "green"))
				passing += 1
			else:
				print(FAIL + text_day + "part2")
				print("\texpected: " + colored(str(expected_part2), "green"))
				print("\treceived: " + colored(str(answer_part2), "red"))
			print()

		passing_color = "yellow"
		if passing == total:
			passing_color = "green"
		elif not passing:
			passing_color = "red"
		passing_count = colored(str(passing) + " passing", passing_color)
		passing_total = colored(", " + str(total) + " total", attrs=["dark"])
		print("Tests: " + passing_count + passing_total)

		if passing != total:
			print("\n" + QUIT)
			return False

		lines = load_file("data/day" + str(self.__class__.day))
		self.setup(lines)

		print()
		time_part1_start = time.time()
		submission_part1 = self.part1()
		time_part1_stop = time.time()
		time_part1 = time_part1_stop - time_part1_start
		output_result(submission_part1, time_part1, text_day)

		time_part2_start = time.time()
		submission_part2 = self.part2()
		time_part2_stop = time.time()
		time_part2 = time_part2_stop - time_part2_start
		output_result(submission_part2, time_part2, text_day)

		elapsed = time_part1 + time_part2
		print("Total time: " + colored(format_time(elapsed), attrs=["dark"]))

		return True

	def setup(self, lines: List[str]) -> None:
		pass

	def part1(self) -> int:
		return -1

	def part2(self) -> int:
		return -1

