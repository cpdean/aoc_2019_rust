from fractions import Fraction
import pytest
import math


def parse_positions(data):
    p = []
    for (y, line) in enumerate(data.split("\n")):
        for (x, cell) in enumerate(line):
            if cell == "#":
                p.append((Fraction(x), Fraction(y)))
    return p


def distance(a, b):
    (a_x, a_y) = a
    (b_x, b_y) = b
    eh = (b_x - a_x) ** 2 + (b_y - a_y) ** 2
    return eh ** (0.5)


def identity_vector(vec):
    d = Fraction(distance((Fraction(0), Fraction(0)), vec))
    x, y = vec
    return (x / d, y / d)

def angle_between(a, b):
    result = math.atan2(b[0] - a[0], a[1] - b[1]) * 180 / math.pi
    if result < 0:
        return 360 + result
    return result


def subtract(a, b):
    (a_x, a_y) = a
    (b_x, b_y) = b
    return (Fraction(a_x - b_x), Fraction(a_y - b_y))


def score_of_position(position, other_points):
    possible = {angle_between(position, i): i for i in other_points}
    return possible


def find_best_position(positions):
    best_so_far = (None, 0)
    for i in range(len(positions)):
        possible = positions[i]
        other = positions[:i] + positions[i + 1 :]
        _s = score_of_position(possible, other)
        score = len(_s)
        (_, old_score) = best_so_far
        if old_score < score:
            best_so_far = (possible, score)
    return best_so_far


def show_scores(positions):
    scores = dict()
    for i in range(len(positions)):
        possible = positions[i]
        other = positions[:i] + positions[i + 1 :]
        score = len(score_of_position(possible, other))
        scores[possible] = score
    width = max(int(x) for (x, y) in scores.keys())
    height = max(int(y) for (x, y) in scores.keys())
    for y in range(height + 1):
        line = "".join(str(scores.get((x, y), ".")) for x in range(width + 1))
        print(line)


def main():
    with open("input/day10.txt") as f:
        data = f.read().strip()
    positions = parse_positions(data)
    print(find_best_position(positions))


def to_frac(ints):
    return [(Fraction(x), Fraction(y)) for x, y in ints]


if __name__ == "__main__":
    main()


def xtest_example():
    data = ".#..#\n" ".....\n" "#####\n" "....#\n" "...##\n"
    positions = parse_positions(data)
    print(positions)
    best = find_best_position(positions)
    show_scores(positions)
    assert best == ((Fraction(3), Fraction(4)), 8)


def test_please():
    positions = [
        (Fraction(1, 1), Fraction(0, 1)),
        (Fraction(4, 1), Fraction(0, 1)),
        (Fraction(0, 1), Fraction(2, 1)),
        (Fraction(1, 1), Fraction(2, 1)),
        (Fraction(2, 1), Fraction(2, 1)),
        (Fraction(3, 1), Fraction(2, 1)),
        (Fraction(4, 1), Fraction(2, 1)),
        (Fraction(4, 1), Fraction(3, 1)),
        (Fraction(3, 1), Fraction(4, 1)),
        (Fraction(4, 1), Fraction(4, 1)),
    ]
    best = find_best_position(positions)
    show_scores(positions)
    assert best == ((Fraction(3), Fraction(4)), 8)


def test_line_left():
    positions = [(0, 0), (0, 1), (0, 2)]
    assert len(score_of_position(positions[0], [positions[1], positions[2]])) == 1


def test_line_middle():
    positions = [(0, 0), (0, 1), (0, 2)]
    assert len(score_of_position(positions[1], [positions[0], positions[2]])) == 2


def test_line_right():
    positions = [(0, 0), (0, 1), (0, 2)]
    assert len(score_of_position(positions[2], [positions[0], positions[1]])) == 1


def test_line_best():
    positions = [(0, 0), (0, 1), (0, 2)]
    assert find_best_position(positions) == ((0, 1), 2)


def test_four_best():
    positions = [
        (0, 0),
        (0, 1),
        (0, 2),
        (0, 3),
        (1, 1),
    ]
    assert find_best_position(positions) == ((1, 1), 4)


def test_best5():
    positions = [
        (0, 0),
        (0, 1),
        (0, 2),
        (0, 3),
        (1, 1),
        (2, 1),
    ]
    assert find_best_position(positions) == ((1, 1), 5)


def test_best_long_diag():
    positions = [
        (0, 0),
        (1, 1),
        (2, 2),
        (3, 3),
        (4, 4),
        (0, 4),
    ]
    assert find_best_position(positions) == ((0, 4), 5)


def test_best_long_diag_shallow():
    positions = [
        (0, 0),
        (1, 2),
        (2, 4),
        (3, 6),
        (4, 8),
        (0, 4),
    ]
    assert find_best_position(positions) == ((0, 4), 5)


def test_best_long_diag_steep():
    positions = to_frac([(0, 0), (2, 1), (4, 2), (6, 3), (8, 4), (4, 0),])
    assert find_best_position(positions) == ((4, 0), 5)
