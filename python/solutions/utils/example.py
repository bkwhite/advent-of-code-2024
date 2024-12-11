# add whatever utilities you'll find useful across multiple solutions
# import them in a solution using:
# from ...utils.example import add


def add(a: int, b: int):
    return a + b


def add_tuples(a: tuple, b: tuple):
    return tuple(map(lambda i, j: i + j, a, b))


def get_next_char(char_list: list[str], current_char: str):
    index = char_list.index(current_char)
    next_index = (index + 1) % len(char_list)
    return char_list[next_index]
