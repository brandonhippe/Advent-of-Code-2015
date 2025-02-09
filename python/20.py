import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
def part1(data):
    """ 2015 Day 20 Part 1
    """

    num = int(data[0])
    houses = [0] * (1 + num // 10)
    for elf in range(1, num // 10 + 1):
        for i in range(elf, num // 10 + 1, elf):
            houses[i] += 10 * elf

    for i, h in enumerate(houses):
        if h >= num:
            break

    return i


def part2(data):
    """ 2015 Day 20 Part 2
    """

    num = int(data[0])
    houses = [0] * (1 + num // 10)
    for elf in range(1, num // 10 + 1):
        for i in range(elf, min(num // 10, elf * 50) + 1, elf):
            houses[i] += 11 * elf

    for i, h in enumerate(houses):
        if h >= num:
            break

    return i


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
        print(f"\nPart 1:\nFirst house to receive minimum presents: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nFirst house to receive minimum presents: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)