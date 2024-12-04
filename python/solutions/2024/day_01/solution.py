# Generated using @xavdid's AoC Python Template: https://github.com/xavdid/advent-of-code-python-template

# puzzle prompt: https://adventofcode.com/2024/day/1

from ...base import StrSplitSolution, answer


def get_cols(input: list[str]):
    left = []
    right = []

    for line in input:
        left.append(int(line.split()[0]))
        right.append(int(line.split()[1]))

    left.sort()
    right.sort()

    return left, right


class Solution(StrSplitSolution):
    _year = 2024
    _day = 1

    @answer(1970720)
    def part_1(self) -> int:
        (left, right) = get_cols(self.input)
        zipped = zip(left, right)
        sum = 0

        for l, r in zipped:
            sum += abs(l - r)

        return sum

    @answer(17191599)
    def part_2(self) -> int:
        (left, right) = get_cols(self.input)
        occurrences = {}

        for n in right:
            if n in occurrences:
                occurrences[n] += 1
            else:
                occurrences[n] = 1

        sum = 0

        for n in left:
            if n in occurrences:
                sum += n * occurrences[n]

        return sum
