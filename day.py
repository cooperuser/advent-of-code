class Day(object):
    day = 0
    title = ""

    def run(self) -> None:
        print("Day {}: {}".format(self.__class__.day, self.__class__.title))

        file = open("data/day" + str(self.__class__.day), 'r')
        lines = file.readlines()
        file.close()
        self.setup(lines)

        print("\tPart 1: " + str(self.part1()))
        print("\tPart 2: " + str(self.part2()))

    def setup(self, lines) -> None:
        pass

    def part1(self) -> int:
        return -1

    def part2(self) -> int:
        return -1

