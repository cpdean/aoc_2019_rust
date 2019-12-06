from collections import defaultdict
import sqlite3

def parse_orbital_map(file_body):
    orbital_map = defaultdict(list)
    for line in file_body.split("\n"):
        root, leaf = line.strip().split(")")
        orbital_map[root].append(leaf)
    return orbital_map

def orbit_count(orbital_map, body):
    print(f"visiting {body}")
    if len(orbital_map[body]) == 0:
        return 1
    else:
        s = 1
        for c in orbital_map[body]:
            print(f"visiting child {c}")
            the_count = orbit_count(orbital_map, c)
            print(f"got {the_count}")
            s += the_count
        print(f"total orbits under {body}, {s}")
        return s
test_example()

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
    om = sql_orbital(file_body)
    cursor, conn = om


def sql_orbital(file_body):
    conn = sqlite3.connect(":memory:")
    cursor = conn.cursor()
    cursor.execute("""
    create table orbits (root text, leaf text);
    """)
    for line in file_body.split("\n"):
        root, leaf = line.strip().split(")")
        cursor.execute("""
        insert into orbits VALUES (?, ?)
        """, (root, leaf))
    conn.commit()
    return (cursor, conn)

def sql_orbital_count(file_body):
    cursor, conn = sql_orbital(file_body)
    (count, ) = cursor.execute("""
    with recursive can_reach as (
    select
    root, leaf
    from orbits
    union all
    select
    o.root, o.leaf
    from orbits o
    join can_reach on o.root = can_reach.leaf
    )
    select
    count(1)
    from can_reach
    """).fetchone()
    return count


with open("input/day06.txt") as f:
    file_body = f.read()
    orbital_map = parse_orbital_map(file_body)
