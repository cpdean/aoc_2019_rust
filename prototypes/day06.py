from collections import defaultdict
import sqlite3

def sql_orbital(file_body):
    conn = sqlite3.connect(":memory:")
    cursor = conn.cursor()
    cursor.execute("""
    create table orbits (root text, leaf text);
    """)
    for line in file_body.strip().split("\n"):
        root, leaf = line.strip().split(")")
        cursor.execute("""
        insert into orbits VALUES (?, ?)
        """, (root, leaf))
    conn.commit()
    return (cursor, conn)

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
K)L
K)YOU
I)SAN"""
    om = sql_orbital(file_body)
    cursor, conn = om

    # can_reach(Root, Leaf) :- orbit(Root, Leaf).
    # can_reach(Root, Leaf) :- orbit(Root, X), can_reach(X, Leaf).
    cursor.execute("""
    with recursive can_reach as (
    select
    root, leaf, 0 degree
    from orbits
    union all
    select
    o.root, can_reach.leaf, can_reach.degree + 1 degree
    from orbits o
    join can_reach on o.leaf = can_reach.root
    )
    select
    *
    from can_reach
    where leaf = 'YOU'
    or leaf = 'SAN'
    """).fetchall()


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
    print(sql_orbital_count(file_body))
