import itertools
import pytest

def base_pattern(ith_element):
    base = [0, 1, 0, -1]
    expanded = ([i] * ith_element for i in base)
    full = itertools.cycle(itertools.chain.from_iterable(expanded))
    # offset by 1, removing first element
    return itertools.islice(full, 1, None)

def fft_phases(start):
    current = start
    while True:
        output = []
        for i, digit in enumerate(current):
            this_digit = 0
            base = base_pattern(i + 1)
            for left, right in zip(current, base):
                this_digit += int(left) * right
            numeric = abs(this_digit)
            output.append(numeric % 10)
        current = output
        yield current

@pytest.mark.parametrize('a,b',[
    (1, [1,0,-1,0,1,0,-1,0,1,0,-1,0,1,0,-1]),
    (2, [0,1,1,0,0,-1,-1,0,0,1,1,0,0,-1,-1]),
    (3, [0,0,1,1,1,0,0,0,-1,-1,-1, 0,0,0,1]),
])
def test_base_pattern_expander(a, b):
    t = list(itertools.islice(base_pattern(a), 15))
    assert t == b

def part2(puzzle_input):
    puzzle_input = 1000 * puzzle_input
    msg_offset = int(''.join(str(i) for i in puzzle_input[:7]))
    stopped = itertools.islice(fft_phases(puzzle_input), 1000)
    drop_first = itertools.islice(stopped, 999, None)
    final_phase = next(drop_first)
    return final_phase[msg_offset:msg_offset+8]


def main():
    with open("input/day16.txt") as f:
        puzzle_input = list(map(int, f.read().strip()))

    for i, e in enumerate(itertools.islice(fft_phases(puzzle_input), 101)):
        watch = ''.join(str(d) for d in e[:8])
        print("{}: {}..".format(i + 1, watch))

if __name__ == "__main__":
    main()


def test_tiny():
    phases = fft_phases(list(map(int, "12345678")))
    assert next(phases) == list(map(int, "48226158"))
    assert next(phases) == list(map(int, "34040438"))

def test_example1():
    answer = part2(list(map(int, "03036732577212944063491565474664")))
    assert answer == list(map(int, "84462026"))
