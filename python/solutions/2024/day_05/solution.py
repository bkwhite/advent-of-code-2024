# Generated using @xavdid's AoC Python Template: https://github.com/xavdid/advent-of-code-python-template

# puzzle prompt: https://adventofcode.com/2024/day/5

from ...base import StrSplitSolution, answer


def parse_input(input: list[str]) -> tuple[map, list[str]]:
    top_section = True
    rules = {}
    updates = []

    for line in input:
        if line == "":
            top_section = False
            continue

        if top_section:
            left = line.split("|")[0]
            right = line.split("|")[1]

            if not rules.get(left):
                rules[left] = {"before": []}
            if not rules.get(right):
                rules[right] = {"before": []}

            rules[left]["before"].append(int(right))

        else:
            updates.append(line)

    return (rules, updates)


def check_update(update: str, rules):
    is_okay = True
    prev = -1

    for current in update.split(","):
        if int(prev) in rules[current]["before"]:
            is_okay = False
        prev = current

    return is_okay


def should_be_before(prev: int, current: str, rules: map):
    if prev in rules[str(current)]["before"]:
        return True
    return False


def bubble_sort(list, rules):
    for n in range(len(list) - 1, 0, -1):
        swapped = False

        for i in range(n):
            if should_be_before(list[i], list[i + 1], rules):
                list[i], list[i + 1] = list[i + 1], list[i]
                swapped = True

        if not swapped:
            break


class Solution(StrSplitSolution):
    _year = 2024
    _day = 5

    # @answer(143)
    def part_1(self) -> int:
        (rules, updates) = parse_input(self.input)
        sum = 0

        for update in updates:
            if check_update(update, rules):
                sum += int(update.split(",")[len(update.split(",")) // 2])

        return sum

    @answer(123)
    def part_2(self) -> int:
        (rules, updates) = parse_input(self.input)
        sum = 0
        bad_updates = []

        for update in updates:
            if not check_update(update, rules):
                bad_updates.append(update)

        for update in bad_updates:
            list = [int(x) for x in str(update).split(",")]
            bubble_sort(list, rules)
            sum += int(list[len(list) // 2])

        return sum

    # @answer((1234, 4567))
    # def solve(self) -> tuple[int, int]:
    #     pass
