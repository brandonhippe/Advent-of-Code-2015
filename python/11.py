import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
def part1(data):
    """ 2015 Day 11 Part 1

    >>> part1(['abcdefgh'])
    'abcdffaa'
    >>> part1(['ghijklmn'])
    'ghjaabcc'

    """

    pw = increment([ord(c) - ord('a') for c in data[0]])

    while not valid(pw):
        pw = increment(pw)

    return ''.join(chr(c + ord('a')) for c in pw)


def part2(data):
    """ 2015 Day 11 Part 2
    """

    pw = increment([ord(c) - ord('a') for c in part1(data)])

    while not valid(pw):
        pw = increment(pw)

    return ''.join(chr(c + ord('a')) for c in pw)


def valid(pw):
    v = False
    for i in range(len(pw) - 2):
        if pw[i + 1] == pw[i] + 1 and pw[i + 2] == pw[i] + 2:
            v = True
            break

    if not v:
        return False

    repeatCount = 0
    i = 0
    while i < len(pw) - 1:
        if pw[i] == pw[i + 1]:
            repeatCount += 1
            i += 1

        i += 1

    if repeatCount < 2:
        return False

    return True


def increment(pw):
    for i, c in enumerate(pw):
        if chr(c + ord('a')) in 'ilo':
            pw[i] += 1
            i += 1
            while i < len(pw):
                pw[i] = 0
                i += 1

            return pw

    ix = -1
    while abs(ix) <= len(pw):
        while True:
            pw[ix] += 1
            pw[ix] %= 26
            if chr(pw[ix] + ord('a')) not in 'ilo':
                break

        if pw[ix] == 0:
            ix -= 1
        else:
            break

    return pw


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
        print(f"\nPart 1:\nNew Password: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nNew Password: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)