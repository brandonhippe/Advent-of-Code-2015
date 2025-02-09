import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import re


SHOP = """
Weapons:    Cost  Damage  Armor
Dagger        8     4       0
Shortsword   10     5       0
Warhammer    25     6       0
Longsword    40     7       0
Greataxe     74     8       0

Armor:      Cost  Damage  Armor
Leather      13     0       1
Chainmail    31     0       2
Splintmail   53     0       3
Bandedmail   75     0       4
Platemail   102     0       5

Rings:      Cost  Damage  Armor
Damage +1    25     1       0
Damage +2    50     2       0
Damage +3   100     3       0
Defense +1   20     0       1
Defense +2   40     0       2
Defense +3   80     0       3
"""


def part1(data):
    """ 2015 Day 21 Part 1
    """

    boss_hp, boss_damage, boss_armor = [[int(x) for x in re.findall('\d+', line)][0] for line in data[:3]]
    
    shop = {"Weapons:": {}, "Armor:": {}, "Rings:": {}}
    for line in SHOP.splitlines():
        if len(line) == 0:
            continue

        line = re.split('\s\s+', line)
        if ':' in line[0]:
            current = line[0]
        else:
            shop[current][line[0]] = [int(x) for x in line[1:]]

    cheapest = float('inf')
    for w in list(shop["Weapons:"].keys()):
        for a in list(shop["Armor:"].keys()) + [None]:
            for r1 in list(shop["Rings:"].keys()) + [None]:
                for r2 in list(shop["Rings:"].keys()) + [None]:
                    if r1 == r2:
                        continue
                    
                    cost, damage, armor = [shop["Weapons:"][w][i] + (shop["Armor:"][a][i] if a is not None else 0) + (shop["Rings:"][r1][i] if r1 is not None else 0) + (shop["Rings:"][r2][i] if r2 is not None else 0) for i in range (3)]
                    if cost < cheapest and simulate([100, damage, armor], [boss_hp, boss_damage, boss_armor]):
                        cheapest = cost

    return cheapest


def part2(data):
    """ 2015 Day 21 Part 2
    """

    boss_hp, boss_damage, boss_armor = [[int(x) for x in re.findall('\d+', line)][0] for line in data[:3]]
    
    shop = {"Weapons:": {}, "Armor:": {}, "Rings:": {}}
    for line in SHOP.splitlines():
        if len(line) == 0:
            continue

        line = re.split('\s\s+', line)
        if ':' in line[0]:
            current = line[0]
        else:
            shop[current][line[0]] = [int(x) for x in line[1:]]

    expensive = float('-inf')
    for w in list(shop["Weapons:"].keys()):
        for a in list(shop["Armor:"].keys()) + [None]:
            for r1 in list(shop["Rings:"].keys()) + [None]:
                for r2 in list(shop["Rings:"].keys()) + [None]:
                    if r1 == r2:
                        continue
                    
                    cost, damage, armor = [shop["Weapons:"][w][i] + (shop["Armor:"][a][i] if a is not None else 0) + (shop["Rings:"][r1][i] if r1 is not None else 0) + (shop["Rings:"][r2][i] if r2 is not None else 0) for i in range (3)]

                    if cost > expensive and not simulate([100, damage, armor], [boss_hp, boss_damage, boss_armor]):
                        expensive = cost
    
    return expensive


def simulate(you, boss):
    hp, damage, armor = you
    boss_hp, boss_damage, boss_armor = boss

    while min(hp, boss_hp) > 0:
        boss_hp -= max(1, damage - boss_armor)
        if boss_hp <= 0:
            break

        hp -= max(1, boss_damage - armor)

    return hp > 0


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
        print(f"\nPart 1:\nCheapest equipment to win: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nMost expensive equipment to lose: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)