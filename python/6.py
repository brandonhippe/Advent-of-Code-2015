import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import re
from collections import defaultdict


def part1(data):
    """ 2015 Day 6 Part 1

    turn on 0,0 through 999,999 would turn on (or leave on) every light.
    toggle 0,0 through 999,0 would toggle the first line of 1000 lights, turning off the ones that were on, and turning on the ones that were off.
    turn off 499,499 through 500,500 would turn off (or leave off) the middle four lights.

    >>> part1(['turn on 0,0 through 999,999', 'toggle 0,0 through 999,0', 'turn off 499,499 through 500,500'])
    998996

    """

    onLights = set()
    for line in data:
        lowX, lowY, highX, highY = [int(x) for x in re.findall(r'\d+', line)]
        for y in range(lowY, highY + 1):
            for x in range(lowX, highX + 1):
                if (x, y) in onLights:
                    if 'off' in line or 'toggle' in line:
                        onLights.remove((x, y))
                else:
                    if 'on' in line or 'toggle' in line:
                        onLights.add((x, y))

    return len(onLights)


def part2(data):
    """ 2015 Day 6 Part 2

    turn on 0,0 through 0,0 would increase the total brightness by 1.
    toggle 0,0 through 999,999 would increase the total brightness by 2000000.

    >>> part2(['turn on 0,0 through 0,0', 'toggle 0,0 through 999,999'])
    2000001

    """

    onLights = defaultdict(lambda: 0)
    for line in data:
        lowX, lowY, highX, highY = [int(x) for x in re.findall(r'\d+', line)]
        for y in range(lowY, highY + 1):
            for x in range(lowX, highX + 1):
                if 'on' in line:
                    onLights[(x, y)] += 1
                elif 'off' in line:
                    onLights[(x, y)] -= 1
                else:
                    onLights[(x, y)] += 2

                if onLights[(x, y)] < 0:
                    onLights[(x, y)] = 0

    return sum(onLights.values())


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
        print(f"Part 1:\nNumber of lights lit: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")
        
    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"Part 2:\nTotal brightness: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)