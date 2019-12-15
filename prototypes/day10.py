def parse_positions(data):
    p = []
    for (y, line) in enumerate(data.split('\n')):
        for (x, cell) in enumerate(line):
            if cell == '#':
                p.append((x, y))
    return p

def distance(a, b):
    (a_x, a_y) = a
    (b_x, b_y) = b
    eh = (b_x - a_x)**2 + (b_y - a_y)**2
    return eh**(.5)


def identity_vector(vec):
    d = distance((0, 0), vec)
    x, y = vec
    return (x / d, y / d)


def subtract(a, b):
    (a_x, a_y) = a
    (b_x, b_y) = b
    return (a_x - b_x, a_y - b_y)


def score_of_position(position, other_points):
    possible = set(identity_vector(subtract(position, i)) for i in other_points)
    return possible

def find_best_position(positions):
    best_so_far = (None, 0)
    for i in range(len(positions)):
        possible = positions[i]
        other = positions[:i] + positions[i+1:]
        _s = score_of_position(possible, other)
        print((possible, len(_s)))
        for i in _s:
            print(i)
        score = len(_s)
        (_, old_score) = best_so_far
        if old_score < score:
            best_so_far = (possible, score)
    return best_so_far

def show_scores(positions):
    scores = dict()
    for i in range(len(positions)):
        possible = positions[i]
        other = positions[:i] + positions[i+1:]
        score = len(score_of_position(possible, other))
        scores[possible] = score
    width = max(x for (x,y) in scores.keys())
    height = max(y for (x,y) in scores.keys())
    for y in range(height+1):
        line = ''.join(str(scores.get((x, y), ".")) for x in range(width+1))
        print(line)

def main():
    with open("input/day10.txt") as f:
        data = f.read().strip()
    positions = parse_positions(data)
    print(find_best_position(positions))


if __name__ == '__main__':
    main()

def test_example():
    data = """.#..#
.....
#####
....#
...##"""
    positions = parse_positions(data)
    best = find_best_position(positions)
    show_scores(positions)
    assert best == ((3, 4), 8)

def test_line_left():
    positions = [
        (0, 0),
        (0, 1),
        (0, 2)
    ]
    assert len(score_of_position(positions[0], [positions[1], positions[2]])) == 1

def test_line_middle():
    positions = [
        (0, 0),
        (0, 1),
        (0, 2)
    ]
    assert len(score_of_position(positions[1], [positions[0], positions[2]])) == 2

def test_line_right():
    positions = [
        (0, 0),
        (0, 1),
        (0, 2)
    ]
    assert len(score_of_position(positions[2], [positions[0], positions[1]])) == 1

def test_line_best():
    positions = [
        (0, 0),
        (0, 1),
        (0, 2)
    ]
    assert find_best_position(positions) == ((0,1), 2)
