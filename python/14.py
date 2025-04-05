import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import re


class Reindeer:
    def __init__(self, text):
        self.name = text.split(' ')[0]
        self.speed, self.duration, self.rest = [int(x) for x in re.findall(r'\d+', text)]

    def distance(self, time):
        d = 0
        while time >= self.duration:
            d += self.speed * self.duration
            time -= self.duration + self.rest

        if time < 0:
            time = 0

        return d + self.speed * time


def part1(data):
    """ 2015 Day 14 Part 1

    >>> part1(['Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.', 'Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.'])
    2660
    """

    return max(Reindeer(r).distance(2503) for r in data)


def part2(data):
    """ 2015 Dat 14 Part 2

    >>> part2(['Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.', 'Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.'])
    1558
    """

    deers = [Reindeer(r) for r in data]
    points = {r.name: 0 for r in deers}
    for t in range(1, 2504):
        best = float('-inf')
        winner = None
        for r in deers:
            d = r.distance(t)
            if d > best:
                best = d
                winner = r.name

        points[winner] += 1

    return max(points.values())


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
        print(f"\nPart 1:\nDistance of winning Reindeer: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nMost points: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)