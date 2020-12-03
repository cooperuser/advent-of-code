class Day(object):
    day = 0
    title = ""

    def run(self) -> None:
        if not self.__class__.title:
            return

        print("Day {}: {}".format(self.__class__.day, self.__class__.title))

        file = open("data/day" + str(self.__class__.day), 'r')
        lines = file.readlines()
        file.close()
        self.setup(lines)

        part1 = self.part1()
        out1 = "NYI" if part1 == -1 else str(part1)
        print("\tPart 1: " + out1)
        part2 = self.part2()
        out2 = "NYI" if part2 == -1 else str(part2)
        print("\tPart 2: " + out2)

    def setup(self, lines) -> None:
        pass

    def part1(self) -> int:
        return -1

    def part2(self) -> int:
        return -1

