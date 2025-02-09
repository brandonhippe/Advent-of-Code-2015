import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import re


class Sue:
    def __init__(self, text):
        self.characteristics = set(re.split(', ', re.split(': ', text, 1)[1]))


def part1(data):
    """ 2015 Day 16 Part 1
    """

    d = [Sue(line) for line in data]
    identifiers = {"children: 3", "cats: 7", "samoyeds: 2", "pomeranians: 3", "akitas: 0", "vizslas: 0", "goldfish: 5", "trees: 3", "cars: 2", "perfumes: 1"}
    possible = []
    for i, s in enumerate(d):
        if len(identifiers.union(s.characteristics)) == len(identifiers):
            possible.append(i + 1)

    return possible[0]


def part2(data):
    """ 2015 Day 16 Part 2
    """

    d = [Sue(line) for line in data]
    identifiers = {"children": [eq, 3], "cats": [gt, 7], "samoyeds": [eq, 2], "pomeranians": [lt, 3], "akitas": [eq, 0], "vizslas": [eq, 0], "goldfish": [lt, 5], "trees": [gt, 3], "cars": [eq, 2], "perfumes": [eq, 1]}
    possible = []
    for i, s in enumerate(d):
        s.characteristics = {c.split(':')[0]: int(c.split(' ')[1]) for c in s.characteristics}
        
        valid = True
        for k, v in zip(s.characteristics.keys(), s.characteristics.values()):
            if not identifiers[k][0](v, identifiers[k][1]):
                valid = False
                break

        if valid:
            possible.append(i + 1)

    return possible[0]


def gt(a, b):
    return a > b


def lt(a, b):
    return a < b


def eq(a, b):
    return a == b


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
        print(f"\nPart 1:\nSusan that got gift: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nActual Susan that got gift: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)