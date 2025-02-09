import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
from collections import defaultdict
import re


def part1(data):
    """ 2015 Day 23 Part 1

    >>> part1(['inc b', 'jio b, +2', 'tpl b', 'inc b'])
    2
    """

    instructions = [re.split(',? ', line.strip('\n')) for line in data]

    regs = defaultdict(lambda: 0)
    while 0 <= regs['PC'] < len(instructions):
        ins = instructions[regs['PC']]
        OPS[ins[0]](regs, ins[1:])

        regs['PC'] += 1

    return regs['b']


def part2(data):
    """ 2015 Day 23 Part 2
    """

    instructions = [re.split(',? ', line.strip('\n')) for line in data]

    regs = defaultdict(lambda: 0)
    regs['a'] = 1
    while 0 <= regs['PC'] < len(instructions):
        ins = instructions[regs['PC']]
        OPS[ins[0]](regs, ins[1:])

        regs['PC'] += 1

    return regs['b']


def hlf(regs, r):
    regs[r[0]] /= 2


def tpl(regs, r):
    regs[r[0]] *= 3


def inc(regs, r):
    regs[r[0]] += 1


def jmp(regs, o):
    regs['PC'] += int(o[0]) - 1


def jie(regs, vals):
    if regs[vals[0]] % 2 == 0:
        regs['PC'] += int(vals[1]) - 1


def jio(regs, vals):
    if regs[vals[0]] == 1:
        regs['PC'] += int(vals[1]) - 1


OPS = {'hlf': hlf, 'tpl': tpl, 'inc': inc, 'jmp': jmp, 'jie': jie, 'jio': jio}


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
        print(f"\nPart 1:\nValue in register b after program runs: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nValue in register b after program runs: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)