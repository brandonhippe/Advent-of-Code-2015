import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import hashlib
import multiprocessing


def part1(data):
    """ 2015 Day 4 Part 1

    If your secret key is abcdef, the answer is 609043, because the MD5 hash of abcdef609043 starts with five zeroes (000001dbbfa...), and it is the lowest such number to do so.
    If your secret key is pqrstuv, the lowest number it combines with to make an MD5 hash starting with five zeroes is 1048970; that is, the MD5 hash of pqrstuv1048970 looks like 000006136ef....

    >>> part1(['abcdef'])
    609043
    >>> part1(['pqrstuv'])
    1048970
    """

    p = multiprocessing.Pool()
    start_val = 0
    step_size = multiprocessing.cpu_count() * 10000
    while not any(results := p.starmap(hash_zeroes, [(data[0], 5, i, i + step_size, 1) for i in range(start_val, start_val + step_size + 1, step_size // multiprocessing.cpu_count())])):
        start_val += step_size

    return min(r for r in results if r)


def part2(data):
    """ 2015 Day 4 Part 2
    """

    results = multiprocessing.Queue()
    step = multiprocessing.cpu_count()
    threads = [multiprocessing.Process(target=hash_zeroes_mt, args=(data[0], 6, i, step, results)) for i in range(step)]

    for t in threads:
        t.start()

    result = results.get()

    # kill everything still churning
    for t in threads:
        t.terminate()

    return result


def hash_zeroes(data, zerosStart, start, end, increment):
    word = data[:]
    for i in range(start, end, increment):
        if not int(hashlib.md5(f'{word}{i}'.encode()).hexdigest()[:zerosStart], 16):
            return i
    
    return None


def hash_zeroes_mt(data, zerosStart, iStart, increment, results):
    word = data[:]
    i = iStart
    while int(hashlib.md5(f'{word}{i}'.encode()).hexdigest()[:zerosStart], 16):
        i += increment

    results.put(i)


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
        print(f"\nPart 1:\nFirst number to produce hash starting with at least 5 zeros: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")
        
    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nFirst number to produce hash starting with at least 6 zeros: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)