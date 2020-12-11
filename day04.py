from day import *
from re import match

class Day04(Day):
	day = 4
	title = "Passport Processing"
	eye_colors = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]

	def setup(self, lines: List[str]) -> None:
		self.passports = []
		passport = {}
		for line in lines:
			clean = line.strip()
			if clean == "":
				self.passports.append(passport)
				passport = {}
				continue
			for item in clean.split(' '):
				pair = item.split(':')
				if pair[0] == "cid":
					continue
				passport[pair[0]] = pair[1]
		self.passports.append(passport)

	def part1(self) -> int:
		valid = 0
		for passport in self.passports:
			if len(passport) == 7:
				valid += 1
		return valid

	def part2(self) -> int:
		valid = 0
		for passport in self.passports:
			if len(passport) != 7:
				continue
			byr = int(passport["byr"])
			iyr = int(passport["iyr"])
			eyr = int(passport["eyr"])
			hgt = passport["hgt"]
			hcl = passport["hcl"]
			ecl = passport["ecl"]
			pid = passport["pid"]
			if Day04.testBYR(byr) and \
				Day04.testIYR(iyr) and \
				Day04.testEYR(eyr) and \
				Day04.testHGT(hgt) and \
				Day04.testHCL(hcl) and \
				Day04.testECL(ecl) and \
				Day04.testPID(pid):
				valid += 1
		return valid

	@staticmethod
	def testBYR(value: int) -> bool:
		result = value >= 1920 and value <= 2002
		return result

	@staticmethod
	def testIYR(value: int) -> bool:
		result = value >= 2010 and value <= 2020
		return result

	@staticmethod
	def testEYR(value: int) -> bool:
		result = value >= 2020 and value <= 2030
		return result

	@staticmethod
	def testHGT(value: str) -> bool:
		heightMatch = match("([0-9]+)(in|cm)", value)
		if not heightMatch:
			return False
		groups = heightMatch.groups()
		height = int(groups[0])
		unit = groups[1]
		if unit == "cm" and (height < 150 or height > 193):
			return False
		if unit == "in" and (height < 59 or height > 76):
			return False
		return True

	@staticmethod
	def testHCL(value: str) -> bool:
		result = bool(match("#[0-9a-f]{6}", value))
		return result

	@staticmethod
	def testECL(value: str) -> bool:
		result = value in Day04.eye_colors
		return result

	@staticmethod
	def testPID(value: str) -> bool:
		result = bool(match("^[0-9]{9}$", value))
		return result


if __name__ == "__main__":
	d = Day04()
	d.test()

