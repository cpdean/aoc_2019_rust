import itertools
import pytest

def base_pattern(ith_element):
    base = [0, 1, 0, -1]
    expanded = [[i] * ith_element for i in base]
    return itertools.cycle(itertools.chain.from_iterable(expanded))

def fft_phases(a, b):
    while True:
        output = []
        for left, right in zip(a, itertools.cycle(b)):
            output.append(left * right)
        numeric = sum(output)
        yield numeric
        a = numeric

@pytest.mark.parametrize('a,b',[
    (1, [0,1,0,-1,0,1,0,-1,0,1,0,-1,0,1,0,-1]),
    (2, [0,0,1,1,0,0,-1,-1,0,0,1,1,0,0,-1,-1]),
    (3, [0,0,0,1,1,1,0,0,0,-1,-1,-1, 0,0,0,1]),
])
def test_base_pattern_expander(a, b):
    t = list(itertools.islice(base_pattern(a), 16))
    assert t == b


def xtest_tiny():
    phases = fft_phases([9, 8, 7, 6, 5], [1, 2, 3])
    assert next(phases) == 38
