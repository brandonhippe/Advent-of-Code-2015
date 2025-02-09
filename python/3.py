import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
from collections import defaultdict


DIRS = {'^': (0, 1),'v': (0, -1), '<': (-1, 0), '>': (1, 0)}


def part1(data):
    """2015 Day 3 Part 1

    > delivers presents to 2 houses: one at the starting location, and one to the east.
    ^>v< delivers presents to 4 houses in a square, including twice to the house at his starting/ending location.
    ^v^v^v^v^v delivers a bunch of presents to some very lucky children at only 2 houses.

    >>> part1('>')
    2
    >>> part1('^>v<')
    4
    >>> part1('^v^v^v^v^v')
    2

    """

    pos = (0, 0)
    visited = defaultdict(lambda: 0)
    visited[pos] = 1

    for d in data:
        pos = tuple(p + o for p, o in zip(pos, DIRS[d]))
        visited[pos] += 1

    return len(visited)


def part2(data):
    """2015 Day 3 Part 2
    
    ^v delivers presents to 3 houses, because Santa goes north, and then Robo-Santa goes south.
    ^>v< now delivers presents to 3 houses, and Santa and Robo-Santa end up back where they started.
    ^v^v^v^v^v now delivers presents to 11 houses, with Santa going one direction and Robo-Santa going the other.
    
    >>> part2('^>')
    3
    >>> part2('^>v<')
    3
    >>> part2('^v^v^v^v^v')
    11

    """

    pos = [(0, 0), (0, 0)]
    visited = defaultdict(lambda: 0)
    visited[pos[0]] = 2

    for i in range(0, len(data), 2):
        for n in range(2):
            pos[n] = tuple(p + o for p, o in zip(pos[n], DIRS[data[i + n]]))
            visited[pos[n]] += 1

    return len(visited)


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
        print(f"\nPart 1:\nNumber of houses that receive at least 1 present: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")
        
    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nNumber of houses that receive at least 1 present: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)