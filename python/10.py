import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
def part1(data, iterations = 40):
    """ 2015 Day 10 Part 1

    1 becomes 11 (1 copy of digit 1).
    11 becomes 21 (2 copies of digit 1).
    21 becomes 1211 (one 2 followed by one 1).
    1211 becomes 111221 (one 1, one 2, and two 1s).
    111221 becomes 312211 (three 1s, two 2s, and one 1).

    >>> part1(['1'], 1)
    2
    >>> part1(['11'], 1)
    2
    >>> part1(['21'], 1)
    4
    >>> part1(['1211'], 1)
    6
    >>> part1(['111221'], 1)
    6

    """

    data = data[0]
    for _ in range(iterations):
        newData = ''
        i = 1
        d = data[0]
        count = 1
        while i < len(data):
            if data[i] != d:
                newData += f'{count}{d}'
                d = data[i]
                count = 1
            else:
                count += 1

            i += 1

        newData += f'{count}{d}'
        data = newData

    return len(data)


def part2(data):
    """ 2015 Day 10 Part 2
    """

    data = data[0]
    for _ in range(50):
        newData = ''
        i = 1
        d = data[0]
        count = 1
        while i < len(data):
            if data[i] != d:
                newData += f'{count}{d}'
                d = data[i]
                count = 1
            else:
                count += 1

            i += 1

        newData += f'{count}{d}'
        data = newData

    return len(data)


def main(input_path: Optional[Path | str]=None, verbose: bool=False) -> Tuple[Tuple[Any, float]]:
    if not input_path:
        if not (input_path := sys.argv[1] if len(sys.argv) > 1 else None):
            year, day = re.findall(r'\d+', str(__file__))[-2:]
            input_path = Path(Path(__file__).parent.parent.parent, "Inputs", f"{year}_{day}.txt")
    
    with open(input_path, encoding='UTF-8') as f:
        data = [line.strip('\n') for line in f.readlines()]
    
    with Timer() as p1_time:
        p1 = part1(data)

    if verbose:
        print(f"\nPart 1:\nLength: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nLength: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)