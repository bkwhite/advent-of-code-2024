# Generated using @xavdid's AoC Python Template: https://github.com/xavdid/advent-of-code-python-template

# puzzle prompt: https://adventofcode.com/2024/day/2

from ...base import StrSplitSolution, answer


def safe_numbers(numbers: list[int]) -> bool:
    last_number = 0
    last_diff = 0
    safe = True

    for index, n in enumerate(numbers):
        if index == 0:
            last_number = n
            continue

        diff = last_number - n
        unsafe_difference = abs(diff) < 1 or abs(diff) > 3
        unsafe_direction = (diff > 0 and last_diff < 0) or (diff < 0 and last_diff > 0)

        last_number = n
        last_diff = diff

        if unsafe_difference or unsafe_direction:
            safe = False
            break

    return safe


def count_safe_reports(input: list[str], tolerate: bool):
    safe_reports = 0

    for line in input:
        numbers = list(map(int, line.split()))
        is_safe = safe_numbers(numbers)

        if not is_safe and not tolerate:
            continue

        if tolerate:
            for index in range(len(numbers)):
                without = numbers.copy()
                without.pop(index)

                if safe_numbers(without):
                    safe_reports += 1
                    break
        else:
            safe_reports += 1

    return safe_reports


class Solution(StrSplitSolution):
    _year = 2024
    _day = 2

    @answer(472)
    def part_1(self) -> int:
        return count_safe_reports(self.input, False)

    @answer(520)
    def part_2(self) -> int:
        return count_safe_reports(self.input, True)
