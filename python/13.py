import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
from itertools import permutations
import re


def part1(data):
    """ 2015 Day 13 Part 1

    >>> part1(['Alice would gain 54 happiness units by sitting next to Bob.', 'Alice would lose 79 happiness units by sitting next to Carol.', 'Alice would lose 2 happiness units by sitting next to David.',    'Bob would gain 83 happiness units by sitting next to Alice.', 'Bob would lose 7 happiness units by sitting next to Carol.', 'Bob would lose 63 happiness units by sitting next to David.', 'Carol would lose 62 happiness units by sitting next to Alice.', 'Carol would gain 60 happiness units by sitting next to Bob.', 'Carol would gain 55 happiness units by sitting next to David.', 'David would gain 46 happiness units by sitting next to Alice.', 'David would lose 7 happiness units by sitting next to Bob.', 'David would gain 41 happiness units by sitting next to Carol.'])
    330

    """

    lines = []
    for i, line in enumerate(data):
        line = re.sub('lose ', 'gain -', re.sub('\.', '', line)).split(' ')
        lines.append([line[0], int(line[3]), line[-1]])

    neighbors = {line[0]: {} for line in lines}
    for line in lines:
        neighbors[line[0]][line[-1]] = line[1]

    best = float('-inf')
    for p in permutations(neighbors.keys()):
        total = 0
        for i, name in enumerate(p):
            total += neighbors[name][p[(i + 1) % len(p)]]
            total += neighbors[name][p[(i - 1) % len(p)]]

        best = max(best, total)

    return best


def part2(data):
    """ 2015 Day 13 Part 2
    """

    lines = []
    for i, line in enumerate(data):
        line = re.sub('lose ', 'gain -', re.sub('\.', '', line)).split(' ')
        lines.append([line[0], int(line[3]), line[-1]])

    neighbors = {line[0]: {} for line in lines}
    for line in lines:
        neighbors[line[0]][line[-1]] = line[1]

    neighbors['Me'] = {k: 0 for k in neighbors.keys()}
    for k in neighbors.keys():
        neighbors[k]['Me'] = 0

    best = float('-inf')
    for p in permutations(neighbors.keys()):
        total = 0
        for i, name in enumerate(p):
            total += neighbors[name][p[(i + 1) % len(p)]]
            total += neighbors[name][p[(i - 1) % len(p)]]

        best = max(best, total)

    return best


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
        print(f"\nPart 1:\nChange in happiness for optimal seating arrangement: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nChange in happiness for optimal seating arrangement: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)