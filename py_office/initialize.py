import sqlite3
from pathlib import Path

from reader import gen_iterate


def insert_raw_player():
    prize_kind: dict[str, int] = {
        "first": 1,
        "first_adjacent": 2,
        "first_different_group": 3,
        "second": 4,
        "third": 5,
        "fourth": 6,
        "fifth": 7,
        "sixth": 8,
        "seventh": 9,
    }

    # records = ((int(row[0]), row[1]) for row in reader if row)
    records = ((int(row[0]), 1 if row[1] == 'sequence_10' else 2, prize_kind[row[2]], int(row[3]), int(row[4])) for row
               in
               gen_iterate(Path('./data/15output_players.tsv')) if row)

    with sqlite3.connect(database='./data.sqlite') as conn:
        conn.execute("PRAGMA journal_mode = OFF;")
        conn.execute("PRAGMA synchronous = OFF;")
        conn.execute("PRAGMA foreign_keys = OFF;")

        conn.executemany('INSERT INTO raw_players(id, strategy, prize, cnt, payout) VALUES (?, ?, ?, ?, ?)', records)


if __name__ == "__main__":
    insert_raw_player()
