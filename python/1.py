import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
from collections import defaultdict


def part1(data):
    """2015 Day 1 Part 1

    (()) and ()() both result in floor 0.
    ((( and (()(()( both result in floor 3.
    ))((((( also results in floor 3.
    ()) and ))( both result in floor -1 (the first basement level).
    ))) and )())()) both result in floor -3.
    
    >>> part1('(())')
    0
    >>> part1('()()')
    0
    >>> part1('(((')
    3
    >>> part1('(()(()(')
    3
    >>> part1('))(((((')
    3
    >>> part1('())')
    -1
    >>> part1('))(')
    -1
    >>> part1(')))')
    -3
    >>> part1(')())())')
    -3
    """

    return findFloor(data)


def part2(data):
    """2015 Day 1 Part 2

    ) causes him to enter the basement at character position 1.
    ()()) causes him to enter the basement at character position 5.

    >>> part2(')')
    1
    >>> part2('()())')
    5
    """

    for n in range(1, len(data) + 1):
        if findFloor(data[:n]) < 0:
            break

    return n


def findFloor(data):
    counts = defaultdict(lambda: 0)
    for d in data:
        counts[d] += 1

    return counts['('] - counts[')']


def main(input_path: Optional[Path | str]=None, verbose: bool=False) -> Tuple[Tuple[Any, float]]:
    if not input_path:
        if not (input_path := sys.argv[1] if len(sys.argv) > 1 else None):
            year, day = re.findall(r'\d+', str(__file__))[-2:]
            input_path = Path(Path(__file__).parent.parent.parent, "Inputs", f"{year}_{day}.txt")
    
    with open(input_path, encoding='UTF-8') as f:
        data = f.readline().strip('\n')
        
    with Timer() as p1_time:
        p1 = part1(data)

    if verbose:
        print(f"\nPart 1:\nFloor Santa ends up on: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")
        
    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nPosition of first character that sends Santa to the basement: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)