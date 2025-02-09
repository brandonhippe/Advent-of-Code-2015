import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
from collections import defaultdict


def part1(data):
    """ 2015 Day 5 Part 1

    ugknbfddgicrmopn is nice because it has at least three vowels (u...i...o...), a double letter (...dd...), and none of the disallowed substrings.
    aaa is nice because it has at least three vowels and a double letter, even though the letters used by different rules overlap.
    jchzalrnumimnmhp is naughty because it has no double letter.
    haegwjzuvuyypxyu is naughty because it contains the string xy.
    dvszwmarrgswjxmb is naughty because it contains only one vowel.

    >>> part1(['ugknbfddgicrmopn', 'aaa', 'jchzalrnumimnmhp', 'haegwjzuvuyypxyu', 'dvszwmarrgswjxmb'])
    2

    """

    niceStrings = 0
    for string in data:
        if 'ab' in string or 'cd' in string or 'pq' in string or 'xy' in string:
            continue

        letterPos = defaultdict(lambda: [])
        for i, l in enumerate(string):
            letterPos[l].append(i)

        if sum(len(letterPos[v]) for v in 'aeiou') < 3:
            continue

        valid = False
        for pos in letterPos.values():
            for i in range(len(pos) - 1):
                if pos[i + 1] == pos[i] + 1:
                    valid = True
                    break

            if valid:
                break

        niceStrings += valid

    return niceStrings


def part2(data):
    """ 2015 Day 5 Part 2

    qjhvhtzxzqqjkmpb is nice because is has a pair that appears twice (qj) and a letter that repeats with exactly one letter between them (zxz).
    xxyxx is nice because it has a pair that appears twice and a letter that repeats with one between, even though the letters used by each rule overlap.
    uurcxstgmygtbstg is naughty because it has a pair (tg) but no repeat with a single letter between them.
    ieodomkazucvgmuy is naughty because it has a repeating letter with one between (odo), but no pair that appears twice.

    >>> part2(['qjhvhtzxzqqjkmpb', 'xxyxx', 'uurcxstgmygtbstg', 'ieodomkazucvgmuy'])
    2

    """

    niceStrings = 0
    for string in data:
        valid = False
        for i in range(len(string) - 2):
            if string[i] == string[i + 2]:
                valid = True
                break
        
        if valid:
            valid = False
            for i in range(len(string) - 2):
                for j in range(i + 2, len(string)):
                    if string[i:i + 2] == string[j:j + 2]:
                        valid = True
                        break

                if valid:
                    break

        niceStrings += valid

    return niceStrings


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
        print(f"\nPart 1:\nTotal number of nice strings: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")
        
    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nTotal number of nice strings: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)