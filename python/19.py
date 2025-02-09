import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import re


def part1(data):
    """ 2015 Day 19 Part 1

    >>> part1(['H => HO', 'H => OH', 'O => HH', '', 'HOH'])
    4
    >>> part1(['H => HO', 'H => OH', 'O => HH', '', 'HOHOHO'])
    7
    """

    string = data[-1]
    rules = {line.split(' => ')[0]: [] for line in data[:-2]}
    for line in data[:-2]:
        line = line.split(' => ')
        rules[line[0]].append(line[1])

    oneStep = set()
    for r in rules.keys():
        end = 0
        o = re.search(r, string)
        while o:
            start, newend = o.span()
            start += end
            end += newend
            for rep in rules[r]:
                oneStep.add(string[:start] + rep + string[end:])

            o = re.search(r, string[end:])

    return len(oneStep)


def part2(data):
    """ 2015 Day 19 Part 2

    >>> part2(['e => H', 'e => O', 'H => HO', 'H => OH', 'O => HH', '', 'HOH'])
    3
    >>> part2(['e => H', 'e => O', 'H => HO', 'H => OH', 'O => HH', '', 'HOHOHO'])
    6
    """

    string = data[-1]
    rules = {line.split(' => ')[0]: [] for line in data[:-2]}
    for line in data[:-2]:
        line = line.split(' => ')
        rules[line[0]].append(line[1])

    backtrackRules = {}
    for k, v in rules.items():
        for s in v:
            backtrackRules[s] = k

    return reverseMoleculeCreation(string, sorted(list(zip(backtrackRules.keys(), backtrackRules.values())), key=lambda e: len(e[0]), reverse=True))


def reverseMoleculeCreation(molecule, backtrackRules):
    if molecule == 'e':
        return 0
    
    for k, v in backtrackRules:
        if k in molecule:
            result = reverseMoleculeCreation(molecule.replace(k, v, 1), backtrackRules)
            if result is not None:
                return result + 1

    return None


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
        print(f"\nPart 1:\nNumber of molecules that can be made with a single replacement: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nFewest steps to create molecule: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)