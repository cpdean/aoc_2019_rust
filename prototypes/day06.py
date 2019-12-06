from collections import defaultdict
with open("input/day06.txt") as f:
    file_body = f.read()
    orbital_map = parse_orbital_map(file_body)


def parse_orbital_map(file_body):
    orbital_map = defaultdict(list)
    for line in file_body.split("\n"):
        root, leaf = line.strip().split(")")
        orbital_map[root].append(leaf)
    return orbital_map

orbital_map['COM']

def orbit_count(orbital_map, body):
    if len(orbital_map[body]) == 0:
        return 1
    else:
        return sum(orbit_count(orbital_map, c) for c in orbital_map[body])

def test_example():
    file_body = """COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L"""
    om = parse_orbital_map(file_body)
    print(orbit_count(om, "COM"))

