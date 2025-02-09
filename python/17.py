import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
from functools import cache


def part1(data, amt = 150):
    """ 2015 Day 17 Part 1

    >>> part1(['20', '15', '10', '5', '5'], 25)
    4

    """

    containers = tuple(sorted([int(line) for line in data], reverse=True))
    containerCounts = {c: len([c1 for c1 in containers if c1 == c]) for c in containers}
    possibleWays = makeSize(containers, amt)

    totalWays = 0
    for pos in possibleWays:
        counts = {c: len([c1 for c1 in pos if c1 == c]) for c in pos}
        valid = True
        for c, count in zip(counts.keys(), counts.values()):
            if count > containerCounts[c]:
                valid = False
                break

        if valid:
            additional = 1
            for c in pos:
                if counts[c] == 1 and containerCounts[c] == 2:
                    additional *= 2

            totalWays += additional

    return totalWays


def part2(data, amt = 150):
    """ 2015 Day 17 Part 2

    >>> part2(['20', '15', '10', '5', '5'], 25)
    3

    """

    containers = tuple(sorted([int(line) for line in data], reverse=True))
    containerCounts = {c: len([c1 for c1 in containers if c1 == c]) for c in containers}
    possibleWays = makeSize(containers, amt)

    smallest = float('inf')
    for pos in possibleWays:
        counts = {c: len([c1 for c1 in pos if c1 == c]) for c in pos}
        valid = True
        for c, count in zip(counts.keys(), counts.values()):
            if count > containerCounts[c]:
                valid = False
                break

        if valid:
            additional = 1
            for c in pos:
                if counts[c] == 1 and containerCounts[c] == 2:
                    additional *= 2

            if len(pos) < smallest:
                smallest = len(pos)
                smallestWays = additional
            elif len(pos) == smallest:
                smallestWays += additional

    return smallestWays


@cache
def makeSize(containers, s):
    if s == 0:
        return [set()]

    allWays = set()
    for c in containers:
        if c <= s:
            allWays |= {tuple(sorted(list(w) + [c], reverse=True)) for w in makeSize(containers, s - c)}

    return allWays


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
        print(f"\nPart 1:\nCombinations of containers that can hold exactly 150 liters: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nCombinations of fewest containers that can hold exactly 150 liters: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)