import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
class Component:
    def __init__(self, s):
        self.left, self.right = s.split(' -> ')
        self.left = self.left.split(' ')
        for i, l in enumerate(self.left):
            try:
                self.left[i] = int(l)
            except ValueError:
                pass

    def determineOutput(self, circuit, wires):
        if len(self.left) == 1:
            if self.left[0] in circuit:
                if self.left[0] not in wires:
                    wires[self.left[0]] = circuit[self.left[0]].determineOutput(circuit, wires)

                return wires[self.left[0]] % 65536
            else:
                return self.left[0] % 65536
        elif len(self.left) == 2:
            if self.left[1] in circuit:
                if self.left[1] not in wires:
                    wires[self.left[1]] = circuit[self.left[1]].determineOutput(circuit, wires)

                return (~wires[self.left[1]]) % 65536
            else:
                return (~self.left[1]) % 65536
        elif len(self.left) == 3:
            if self.left[0] in circuit:
                if self.left[0] not in wires:
                    wires[self.left[0]] = circuit[self.left[0]].determineOutput(circuit, wires)

                v1 = wires[self.left[0]]
            else:
                v1 = self.left[0]

            if self.left[2] in circuit:
                if self.left[2] not in wires:
                    wires[self.left[2]] = circuit[self.left[2]].determineOutput(circuit, wires)

                v2 = wires[self.left[2]]
            else:
                v2 = self.left[2]

            if self.left[1] == 'AND':
                return (v1 & v2) % 65536
            elif self.left[1] == 'OR':
                return (v1 | v2) % 65536
            elif self.left[1] == 'LSHIFT':
                return (v1 << v2) % 65536
            elif self.left[1] == 'RSHIFT':
                return (v1 >> v2) % 65536
            

def part1(data):
    """ 2015 Day 7 Part 1

    >>> part1(['123 -> x', '456 -> y', 'x AND y -> d', 'x OR y -> e', 'x LSHIFT 2 -> f', 'y RSHIFT 2 -> g', 'NOT x -> h', 'NOT y -> i'])
    {'d': 72, 'e': 507, 'f': 492, 'g': 114, 'h': 65412, 'i': 65079, 'x': 123, 'y': 456}

    """

    circuit = {}
    for line in data:
        c = Component(line)
        circuit[c.right] = c

    return circuit['a'].determineOutput(circuit, {}) if 'a' in circuit else {k: circuit[k].determineOutput(circuit, {}) for k in sorted(circuit.keys())}


def part2(data):
    """ 2015 Day 7 Part 2
    """

    circuit = {}
    for line in data:
        c = Component(line)
        circuit[c.right] = c

    wires = {'b': circuit['a'].determineOutput(circuit, {})}

    return circuit['a'].determineOutput(circuit, wires)


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
        print(f"\nPart 1:\nSignal provided to wire A: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nSignal provided to wire A: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)