import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
def part1(data, iterations = 100):
    """ 2015 Day 18 Part 1

    >>> part1(['.#.#.#', '...##.', '#....#', '..#...', '#.#..#', '####..'], 4)
    4
    """

    positions = set()
    for y, line in enumerate(data):
        for x, l in enumerate(line):
            if l == '#':
                positions.add((x, y))

    for _ in range(iterations):
        positions = iterate(positions, len(data))

    return len(positions)


def part2(data, iterations = 100):
    """ 2015 Day 18 Part 2

    >>> part2(['.#.#.#', '...##.', '#....#', '..#...', '#.#..#', '####..'], 5)
    17
    """

    corners = {(0, 0), (0, len(data) - 1), (len(data) - 1, 0), (len(data) - 1, len(data) - 1)}
    positions = set()
    for y, line in enumerate(data):
        for x, l in enumerate(line):
            if l == '#':
                positions.add((x, y))

    positions = positions.union(corners)

    for _ in range(iterations):
        positions = iterate(positions, len(data))
        positions = positions.union(corners)

    return len(positions)


def iterate(positons, maxDim):
    newPositions = set()
    for y in range(maxDim):
        for x in range(maxDim):
            neighbors = 0
            for yOff in range(-1, 2):
                for xOff in range(-1, 2):
                    if xOff == 0 and yOff == 0:
                        continue

                    if (x + xOff, y + yOff) in positons:
                        neighbors += 1

            if ((x, y) in positons and (neighbors == 2 or neighbors == 3)) or ((x, y) not in positons and neighbors == 3):
                newPositions.add((x, y))

    return newPositions


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
        print(f"\nPart 1:\nLights on after 100 iterations: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nLights on after 100 iterations with corners stuck on: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)