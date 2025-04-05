import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import re


def part1(data):
    """ 2015 Day 22 Part 1
    """

    return simulateP1([50, 500], [[int(x) for x in re.findall(r'\d+', line)][0] for line in data], [])


def part2(data):
    """ 2015 Day 22 Part 2
    """

    return simulateP2([50, 500], [[int(x) for x in re.findall(r'\d+', line)][0] for line in data], [])


def applyEffects(hp, mana, boss_hp, effects):
    newEffects = []
    armor = 0
    for e, timer in effects:
        if e == "Shield":
            armor += 7
        elif e == "Poison":
            boss_hp -= 3
        elif e == "Recharge":
            mana += 101

        timer -= 1
        if timer > 0:
            newEffects.append([e, timer])

    return [hp, mana, armor, boss_hp, newEffects]


def simulateP1(you, boss, effects):
    hp, mana = you
    boss_hp, boss_damage = boss

    if min(hp, boss_hp) <= 0 or mana < 53:
        return 0 if boss_hp <= 0 else float('inf')

    bestCost = float('inf')
    for e in ["Magic Missile", "Drain", "Shield", "Poison", "Recharge"]:
        new_hp, new_mana, new_armor, new_boss_hp, newEffects = applyEffects(hp, mana, boss_hp, effects)
        cost = 0
        if e == "Magic Missile" and new_mana >= 53:
            new_boss_hp -= 4
            new_mana -= 53
            cost = 53
        elif e == "Drain" and new_mana >= 73:
            new_boss_hp -= 2
            new_hp += 2
            new_mana -= 73
            cost = 73
        elif e == "Shield" and new_mana >= 113 and "Shield" not in [effect[0] for effect in newEffects]:
            newEffects.append(["Shield", 6])
            new_mana -= 113
            cost = 113
        elif e == "Poison" and new_mana >= 173 and "Poison" not in [effect[0] for effect in newEffects]:
            newEffects.append(["Poison", 6])
            new_mana -= 173
            cost = 173
        elif e == "Recharge" and new_mana >= 229 and "Recharge" not in [effect[0] for effect in newEffects]:
            newEffects.append(["Recharge", 5])
            new_mana -= 229
            cost = 229
        else:
            continue 

        if min(new_hp, new_boss_hp) > 0:
            new_hp, new_mana, new_armor, new_boss_hp, newEffects = applyEffects(new_hp, new_mana, new_boss_hp, newEffects)

        if min(new_hp, new_boss_hp) > 0:
            new_hp -= max(1, boss_damage - new_armor)

        cost += simulateP1([new_hp, new_mana], [new_boss_hp, boss_damage], newEffects)
        if cost < bestCost:
            bestCost = cost

    return bestCost


def simulateP2(you, boss, effects):
    hp, mana = you
    boss_hp, boss_damage = boss

    if min(hp, boss_hp) <= 0 or mana < 53:
        return 0 if boss_hp <= 0 else float('inf')

    bestCost = float('inf')
    for e in ["Magic Missile", "Drain", "Shield", "Poison", "Recharge"]:
        new_hp, new_mana, new_armor, new_boss_hp, newEffects = applyEffects(hp, mana, boss_hp, effects)
        cost = 0
        if e == "Magic Missile" and new_mana >= 53:
            new_boss_hp -= 4
            new_mana -= 53
            cost = 53
        elif e == "Drain" and new_mana >= 73:
            new_boss_hp -= 2
            new_hp += 2
            new_mana -= 73
            cost = 73
        elif e == "Shield" and new_mana >= 113 and "Shield" not in [effect[0] for effect in newEffects]:
            newEffects.append(["Shield", 6])
            new_mana -= 113
            cost = 113
        elif e == "Poison" and new_mana >= 173 and "Poison" not in [effect[0] for effect in newEffects]:
            newEffects.append(["Poison", 6])
            new_mana -= 173
            cost = 173
        elif e == "Recharge" and new_mana >= 229 and "Recharge" not in [effect[0] for effect in newEffects]:
            newEffects.append(["Recharge", 5])
            new_mana -= 229
            cost = 229
        else:
            continue 

        if min(new_hp, new_boss_hp) > 0:
            new_hp, new_mana, new_armor, new_boss_hp, newEffects = applyEffects(new_hp, new_mana, new_boss_hp, newEffects)

        if min(new_hp, new_boss_hp) > 0:
            new_hp -= max(1, boss_damage - new_armor)

        cost += simulateP2([new_hp - 1, new_mana], [new_boss_hp, boss_damage], newEffects)
        if cost < bestCost:
            bestCost = cost

    return bestCost


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
        print(f"\nPart 1:\nLeast mana to win: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nLeast mana to win on hard: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)