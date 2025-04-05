import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import re
from itertools import product


class Ingredient:
    def __init__(self, text):
        self.capacity, self.durability, self.flavor, self.texture, self.calories = [int(x) for x in re.findall(r'-?\d+', text)]


def part1(data):
    """ 2015 Day 15 Part 1

    >>> part1(['Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8', 'Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3'])
    62842880
    """

    ingredients = [Ingredient(line) for line in data]
    best = 0

    for split in product(range(0, 101), repeat=len(ingredients) - 1):
        counts = [split[0]] + [split[i] - split[i - 1] for i in range(1, len(split))] + [100 - split[-1]]
        if min(counts) < 0:
            continue
        
        c = sum(ingredients[i].capacity * counts[i] for i in range(len(ingredients)))
        d = sum(ingredients[i].durability * counts[i] for i in range(len(ingredients)))
        f = sum(ingredients[i].flavor * counts[i] for i in range(len(ingredients)))
        t = sum(ingredients[i].texture * counts[i] for i in range(len(ingredients)))

        if c < 0:
            c = 0
        if d < 0:
            d = 0
        if f < 0:
            f = 0
        if t < 0:
            t = 0

        best = max(best, c * d * f * t)

    return best


def part2(data):
    """ 2015 Day 15 Part 2

    >>> part2(['Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8', 'Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3'])
    57600000
    """

    ingredients = [Ingredient(line) for line in data]
    best = 0

    for split in product(range(0, 101), repeat=len(ingredients) - 1):
        counts = [split[0]] + [split[i] - split[i - 1] for i in range(1, len(split))] + [100 - split[-1]]
        if min(counts) < 0:
            continue
        
        c = sum(ingredients[i].capacity * counts[i] for i in range(len(ingredients)))
        d = sum(ingredients[i].durability * counts[i] for i in range(len(ingredients)))
        f = sum(ingredients[i].flavor * counts[i] for i in range(len(ingredients)))
        t = sum(ingredients[i].texture * counts[i] for i in range(len(ingredients)))
        cals = sum(ingredients[i].calories * counts[i] for i in range(len(ingredients)))

        if c < 0:
            c = 0
        if d < 0:
            d = 0
        if f < 0:
            f = 0
        if t < 0:
            t = 0

        if cals == 500:
            best = max(best, c * d * f * t)

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
        print(f"\nPart 1:\nScore of best cookie: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nScore of best 500 calorie cookie: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)