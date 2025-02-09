import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
def part1(data):
    """2015 Day 2 Part 1

    A present with dimensions 2x3x4 requires 2*6 + 2*12 + 2*8 = 52 square feet of wrapping paper plus 6 square feet of slack, for a total of 58 square feet.
    A present with dimensions 1x1x10 requires 2*1 + 2*10 + 2*10 = 42 square feet of wrapping paper plus 1 square foot of slack, for a total of 43 square feet.

    >>> part1([[2, 3, 4]])
    58
    >>> part1([[1, 1, 10]])
    43

    """

    return sum(wrappingPaper(box) for box in data)


def part2(data):
    """2015 Day 2 Part 2

    A present with dimensions 2x3x4 requires 2+2+3+3 = 10 feet of ribbon to wrap the present plus 2*3*4 = 24 feet of ribbon for the bow, for a total of 34 feet.
    A present with dimensions 1x1x10 requires 1+1+1+1 = 4 feet of ribbon to wrap the present plus 1*1*10 = 10 feet of ribbon for the bow, for a total of 14 feet.

    >>> part2([[2, 3, 4]])
    34
    >>> part2([[1, 1, 10]])
    14

    """

    return sum(ribbon(box) for box in data)


def wrappingPaper(box):
    total = box[0] * box[1]
    for i in range(len(box) - 1):
        for j in range(i + 1, len(box)):
            total += 2 * box[i] * box[j]

    return total


def ribbon(box):
    return 2 * (box[0] + box[1]) + volume(box)


def volume(box):
    total = 1
    for side in box:
        total *= side

    return total


def main(input_path: Optional[Path | str]=None, verbose: bool=False) -> Tuple[Tuple[Any, float]]:
    if not input_path:
        if not (input_path := sys.argv[1] if len(sys.argv) > 1 else None):
            year, day = re.findall(r'\d+', str(__file__))[-2:]
            input_path = Path(Path(__file__).parent.parent.parent, "Inputs", f"{year}_{day}.txt")
    
    with open(input_path, encoding='UTF-8') as f:
        data = [line.strip('\n') for line in f.readlines()]
        
    boxes = [list(sorted(int(x) for x in re.findall('\d+', line))) for line in data]
       
    with Timer() as p1_time:
        p1 = part1(boxes)

    if verbose:
        print(f"\nPart 1:\nTotal wrapping paper needed: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(boxes)

    if verbose:
        print(f"\nPart 2:\nTotal ribbon needed: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]
    

if __name__ == "__main__":
    main(verbose=True)