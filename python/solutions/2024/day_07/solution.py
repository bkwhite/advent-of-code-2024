# Generated using @xavdid's AoC Python Template: https://github.com/xavdid/advent-of-code-python-template

# puzzle prompt: https://adventofcode.com/2024/day/7

import re

from typing import NewType

from ...utils.example import set_char_at_index

from ...base import StrSplitSolution, answer

LINE_REGEX = r"(\d+): ((?:\d+ *)+)"

ParsedInput = NewType("ParsedInput", tuple[int, list[int]])


def parse_input(input: list[str]) -> list[ParsedInput]:
    parsed = []

    for line in input:
        m = re.search(LINE_REGEX, line)
        parsed.append((int(m.group(1)), list(map(int, m.group(2).split(" ")))))

    return parsed


def calc_possibilities(length: int):
    if length == 2:
        return 1
    return ((length - 1) * 4) - 2


def build_possibilities(line: ParsedInput, concatenation: bool):
    (_, numbers) = line
    possibilities: list[str] = []

    for n in numbers:
        if len(possibilities) == 0:
            possibilities.append(str(n))
            continue

        p = []

        for possibility in possibilities:
            p.append("{}+{}".format(possibility, n))
            p.append("{}*{}".format(possibility, n))
            if concatenation:
                p.append("{}|{}".format(possibility, n))

        possibilities = p

    return list(set(possibilities))


def calculate_line(tokens: list[str]):
    operation = None
    value = 0

    for token in tokens:

        if token == "+" or token == "*" or token == "|":
            operation = token
            continue

        if operation:
            b = int(token)

            if operation == "+":
                value += b
            elif operation == "*":
                value *= b
            elif operation == "|":
                value = int(str(value) + str(b))

        elif value == 0:
            value = int(token)

        operation = None

    return value


def check_line(possibilities: list[str], test: int):
    something_works = False

    for possibility in possibilities:
        tokens = re.split(r"([\+\*\|])", possibility)
        if calculate_line(tokens) == test:
            something_works = True
            break

    return something_works


class Solution(StrSplitSolution):
    _year = 2024
    _day = 7

    # @answer(3749)
    # @answer(5030892084481)
    def part_1(self) -> int:
        count = 0
        parsed = parse_input(self.input)

        for p in parsed:
            (test, _) = p
            possibilities = build_possibilities(p, False)

            if check_line(possibilities, test):
                count += test

        return count

    # @answer(11387)
    # @answer(91377448644679)
    def part_2(self) -> int:
        count = 0
        parsed = parse_input(self.input)

        for p in parsed:
            (test, _) = p
            possibilities = build_possibilities(p, True)

            if check_line(possibilities, test):
                count += test

        return count

        pass

    # @answer((1234, 4567))
    # def solve(self) -> tuple[int, int]:
    #     pass
