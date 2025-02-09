import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import json


def part1(data):
    """ 2015 Day 12 Part 1

    [1,2,3] and {"a":2,"b":4} both have a sum of 6.
    [[[3]]] and {"a":{"b":4},"c":-1} both have a sum of 3.
    {"a":[-1,1]} and [-1,{"a":1}] both have a sum of 0.
    [] and {} both have a sum of 0.

    >>> part1(['[1,2,3]', '{"a":2,"b":4}', '[[[3]]]', '{"a":{"b":4},"c":-1}', '{"a":[-1,1]}', '[-1,{"a":1}]', '[]', '{}'])
    18

    """

    total = 0
    for line in data:
        total += numSum(json.loads(line), False)

    return total


def part2(data):
    """ 2015 Day 12 Part 2

    [1,2,3] still has a sum of 6.
    [1,{"c":"red","b":2},3] now has a sum of 4, because the middle object is ignored.
    {"d":"red","e":[1,2,3,4],"f":5} now has a sum of 0, because the entire structure is ignored.
    [1,"red",5] has a sum of 6, because "red" in an array has no effect.

    >>> part2(['[1,2,3]', '[1,{"c":"red","b":2},3]', '{"d":"red","e":[1,2,3,4],"f":5}', '[1,"red",5]'])
    16

    """

    total = 0
    for line in data:
        total += numSum(json.loads(line), True)

    return total
    

def numSum(data, p2):
    if isinstance(data, dict):
        return sum(numSum(d, p2) for d in data.values()) if not p2 or "red" not in data.values() else 0
    elif isinstance(data, list):
        return sum(numSum(d, p2) for d in data)
    elif isinstance(data, int):
        return data
    else:
        return 0


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
        print(f"\nPart 1:\nSum of numbers: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nSum of numbers after ignoring red: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)