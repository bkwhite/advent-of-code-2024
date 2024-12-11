# Generated using @xavdid's AoC Python Template: https://github.com/xavdid/advent-of-code-python-template

# puzzle prompt: https://adventofcode.com/2024/day/6

# I have learned why python doesn't like recursion =

import sys

from ...base import StrSplitSolution, answer

from ...utils.example import add_tuples, get_next_char

GUARD_CHARS = ["^", ">", "V", "<"]


def parse_input(input: list[str]) -> tuple[list[list[int]], list[str]]:
    width = len(input[0])
    height = len(input)
    grid = [[0 for _ in range(width)] for _ in range(height)]

    for y_index, line in enumerate(input):
        for x_index, c in enumerate(line):
            grid[y_index][x_index] = c

    return grid


def print_grid_state(grid: list[list[str]]):
    print()
    for row in grid:
        print("".join(str(x) for x in row))
    print()


def get_char_at(grid: list[list[str]], pos: tuple[int, int]):
    (y, x) = pos
    return grid[x][y]


def set_char_at(grid: list[list[str]], pos: tuple[int, int], char: str):
    (y, x) = pos
    grid[x][y] = char


def get_guard_position(grid: list[list[str]]):
    for y, row in enumerate(grid):
        for gc in GUARD_CHARS:
            if gc in row:
                return (row.index(gc), y)


def get_guard_direction(grid: list[list[str]]):
    pos = get_guard_position(grid)
    gc = get_char_at(grid, pos)

    match gc:
        case "^":
            return (0, -1)
        case ">":
            return (1, 0)
        case "<":
            return (-1, 0)
        case "V":
            return (0, 1)


def get_guard_char(grid: list[list[str]]):
    pos = get_guard_position(grid)
    if pos:
        return get_char_at(grid, pos)


def rotate_guard(grid: list[list[str]], pos: tuple[int, int]):
    pos = get_guard_position(grid)
    gc = get_char_at(grid, pos)
    next_gc = get_next_char(GUARD_CHARS, gc)
    set_char_at(grid, pos, next_gc)


def get_next_pos(grid: list[list[str]], pos: tuple[int, int]):
    size = len(grid)
    (x, y) = pos

    if x < 0 or y < 0 or x > (size - 1) or y > (size - 1):
        return None

    return get_char_at(grid, pos)


def count_char_in_grid(grid: list[list[str]], char: str):
    count = 0

    for y_index, line in enumerate(grid):
        for x_index, c in enumerate(line):
            if grid[y_index][x_index] == char:
                count += 1

    return count


def move(grid: list[list[str]], iteration: int):
    pos = get_guard_position(grid)
    direction = get_guard_direction(grid)

    next_pos = add_tuples(pos, direction)
    next_char = get_next_pos(grid, next_pos)

    if next_char and next_char != "#":
        gc = get_guard_char(grid)

        if pos:
            iteration += 1
            set_char_at(grid, pos, "X")

        set_char_at(grid, next_pos, gc)
    elif next_char == "#":
        rotate_guard(grid, pos)
    else:
        print(iteration)
        set_char_at(grid, pos, "X")
        return False

    return move(grid, iteration)


class Solution(StrSplitSolution):
    _year = 2024
    _day = 6

    # @answer(41)
    def part_1(self) -> int:
        grid = parse_input(self.input)
        sys.setrecursionlimit(10000)
        move(grid, 0)
        count = count_char_in_grid(grid, "X")
        return count

    # @answer(1234)
    def part_2(self) -> int:
        grid = parse_input(self.input)
        pass

    # @answer((1234, 4567))
    # def solve(self) -> tuple[int, int]:
    #     pass
