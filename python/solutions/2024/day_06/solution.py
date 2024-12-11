# Generated using @xavdid's AoC Python Template: https://github.com/xavdid/advent-of-code-python-template

# puzzle prompt: https://adventofcode.com/2024/day/6

# I have learned why python doesn't like recursion =

import sys
import copy
import concurrent.futures

from ...base import StrSplitSolution, answer

from ...utils.example import add_tuples, get_next_char, set_char_at_index

GUARD_CHARS = ["^", ">", "V", "<"]


def parse_input(input: list[str]) -> tuple[list[list[int]], list[str]]:
    width = len(input[0])
    height = len(input)

    grid = [[0 for _ in range(width)] for _ in range(height)]

    for y_index, line in enumerate(input):
        for x_index, c in enumerate(line):
            grid[y_index][x_index] = c

    return grid


def parse_input2(input: list[str]) -> tuple[str, int]:
    grid = ""
    size = len(input[0])

    for line in input:
        grid = grid + line

    return (grid, size)


def print_grid_state(grid: str, size: int):
    for i, c in enumerate(grid):
        if i > 0 and i % size == 0:
            print()
        print(c, end="")
    print()


def write_grid_state_to_file(grid: str, size: int, filename: str):
    with open(filename, "w") as file:
        for i, c in enumerate(grid):
            if i > 0 and i % size == 0:
                file.write("\n")
            file.write(c)
        file.write("\n")


def pos_to_index(size: int, pos: tuple[int, int]) -> int:
    (x, y) = pos
    return y * size + x


def index_to_pos(size: int, index: int) -> tuple[int, int]:
    x = index % size
    y = index // size
    return (x, y)


def get_char_at(grid: str, size: int, pos: tuple[int, int]) -> str:
    return grid[pos_to_index(size, pos)]


def set_char_at(grid: str, size: int, pos: tuple[int, int], char: str):
    (x, y) = pos
    return set_char_at_index(grid, y * size + x, char)


def get_guard_position(
    grid: str,
    size: int,
) -> tuple[tuple[int, int], tuple[int, int], str]:
    for gc in GUARD_CHARS:
        if gc in grid:
            pos = index_to_pos(size, grid.index(gc))
            return (pos, get_guard_direction(gc), gc)


def get_guard_direction(gc: str):
    match gc:
        case "^":
            return (0, -1)
        case ">":
            return (1, 0)
        case "<":
            return (-1, 0)
        case "V":
            return (0, 1)


def rotate_guard(grid: str, size: int, pos: tuple[int, int]):
    gc = get_char_at(grid, size, pos)
    next_gc = get_next_char(GUARD_CHARS, gc)
    return set_char_at(grid, size, pos, next_gc)


def get_next_pos(grid: str, size: int, pos: tuple[int, int]):
    (x, y) = pos

    if x < 0 or y < 0 or x > (size - 1) or y > (size - 1):
        return None

    return get_char_at(grid, size, pos)


def count_char_in_grid(grid: str, char: str):
    return grid.count(char)


def get_all_positions(grid: str, size: int, char: str):
    positions: list[tuple[int, int, int]] = []

    for i, c in enumerate(grid):
        if c == char:
            (x, y) = index_to_pos(size, i)
            positions.append((x, y, i))

    return positions


def walk(init_grid: str, size: int):
    (g_pos, g_dir, gc) = get_guard_position(init_grid, size)
    grid = init_grid

    while g_pos:
        next_pos = add_tuples(g_pos, g_dir)
        next_char = get_next_pos(grid, size, next_pos)

        if next_char and next_char != "#":
            if g_pos:
                grid = set_char_at(grid, size, g_pos, "X")

            grid = set_char_at(grid, size, next_pos, gc)
        elif next_char == "#":
            grid = rotate_guard(grid, size, g_pos)
        else:
            grid = set_char_at(grid, size, g_pos, "X")
            break

        (g_pos, g_dir, gc) = get_guard_position(grid, size)
        continue

    return grid


def has_cycle(init_grid: str, size: int, index) -> bool:
    (g_pos, g_dir, gc) = get_guard_position(init_grid, size)
    grid = init_grid
    hit_dict = {}

    while g_pos:
        next_pos = add_tuples(g_pos, g_dir)
        next_char = get_next_pos(grid, size, next_pos)

        if next_char and next_char != "#" and next_char != "O":
            if g_pos:
                grid = set_char_at(grid, size, g_pos, "X")

            grid = set_char_at(grid, size, next_pos, gc)
        elif next_char == "#" or next_char == "O":
            (x, y) = g_pos
            grid = rotate_guard(grid, size, g_pos)

            if "{}-{}-{}".format(x, y, gc) in hit_dict:
                return True
            else:
                hit_dict["{}-{}-{}".format(x, y, gc)] = True
        else:
            grid = set_char_at(grid, size, g_pos, "X")
            return False

        (g_pos, g_dir, gc) = get_guard_position(grid, size)

        continue


class Solution(StrSplitSolution):
    _year = 2024
    _day = 6

    @answer(5269)
    def part_1(self) -> int:
        input = parse_input2(self.input)
        (grid, size) = input
        walked = walk(grid, size)
        count = count_char_in_grid(walked, "X")
        return count

    @answer(1957)
    def part_2(self) -> int:
        input = parse_input2(self.input)
        (grid, size) = input
        walked = walk(grid, size)
        (guard_position, _, _) = get_guard_position(grid, size)
        walked_positions = get_all_positions(walked, size, "X")

        def count_cycles():
            iii = 0
            cycle_count = 0
            for position in walked_positions:
                iii += 1
                (x, y, index) = position
                pos = (x, y)

                if pos == guard_position:
                    continue

                this_grid = grid
                this_grid = set_char_at(this_grid, size, pos, "O")

                if has_cycle(this_grid, size, index):
                    cycle_count += 1

            return cycle_count

        return count_cycles()

    # @answer((1234, 4567))
    # def solve(self) -> tuple[int, int]:
    #     pass
