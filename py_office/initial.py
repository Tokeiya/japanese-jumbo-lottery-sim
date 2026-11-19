import csv
import sqlite3
from pathlib import Path
from sqlite3 import Connection


def create_table(conn: Connection) -> None:
    print("Creating table")
    conn.executescript("""
                       PRAGMA FOREIGN_KEYS = OFF;
                       PRAGMA JOURNAL_MODE = OFF;
                       PRAGMA SYNCHRONOUS = OFF;


                       DROP TABLE IF EXISTS raw;
                       DROP TABLE IF EXISTS prize_kind;
                       DROP TABLE IF EXISTS strategy_kind;
                       DROP TABLE IF EXISTS prizes;
                       DROP TABLE IF EXISTS person;


                       CREATE TABLE raw
                       (
                           id                          INTEGER PRIMARY KEY AUTOINCREMENT,
                           unit                        INTEGER NOT NULL DEFAULT 0,
                           strategy                    TEXT    NOT NULL,
                           total_payed                 INTEGER NOT NULL,
                           first_cnt                   INTEGER NOT NULL,
                           first_prize                 INTEGER NOT NULL,
                           first_adjacent_cnt          INTEGER NOT NULL,
                           first_adjacent_prize        INTEGER NOT NULL,
                           first_different_group_cnt   INTEGER NOT NULL,
                           first_different_group_prize INTEGER NOT NULL,
                           second_cnt                  INTEGER NOT NULL,
                           second_prize                INTEGER NOT NULL,
                           third_cnt                   INTEGER NOT NULL,
                           third_prize                 INTEGER NOT NULL,
                           fourth_cnt                  INTEGER NOT NULL,
                           fourth_prize                INTEGER NOT NULL,
                           fifth_cnt                   INTEGER NOT NULL,
                           fifth_prize                 INTEGER NOT NULL,
                           sixth_cnt                   INTEGER NOT NULL,
                           sixth_prize                 INTEGER NOT NULL,
                           seventh_cnt                 INTEGER NOT NULL,
                           seventh_prize               INTEGER NOT NULL
                       );

                       CREATE TABLE strategy_kind
                       (
                           id   INTEGER PRIMARY KEY,
                           name TEXT NOT NULL
                       );

                       INSERT INTO strategy_kind(id, name)
                       VALUES (1, 'sequence_10'),
                              (2, 'random_10');

                       CREATE TABLE prize_kind
                       (
                           id   INTEGER PRIMARY KEY,
                           name TEXT NOT NULL
                       );

                       INSERT INTO prize_kind(id, name)
                       VALUES (1, 'first'),
                              (2, 'first_adjacent'),
                              (3, 'first_different_group'),
                              (4, 'second'),
                              (5, 'third'),
                              (6, 'fourth'),
                              (7, 'fifth'),
                              (8, 'sixth'),
                              (9, 'seventh');

                       CREATE TABLE person
                       (
                           id          INTEGER PRIMARY KEY,
                           unit        INTEGER NOT NULL,
                           total_payed INTEGER NOT NULL
                       );

                       CREATE TABLE prizes
                       (
                           id        INTEGER PRIMARY KEY,
                           person_id INTEGER NOT NULL REFERENCES person (id),
                           kind      INTEGER NOT NULL REFERENCES prize_kind (id),
                           cnt       INTEGER NOT NULL,
                           reward    INTEGER NOT NULL
                       );

                       VACUUM;
                       """)
    print("table created")


def generate_typed_iter(path: Path):
    with open(path, mode='r', encoding='utf-8') as file:
        reader = csv.reader(
            file,
            delimiter="\t",
        )

        next(reader)

        for rec in reader:
            yield int(rec[1] == 'sequence_10'), *(int(v) for v in rec[2:])


def sample() -> None:
    with sqlite3.connect("./data.sqlite") as conn:
        create_table(conn)
        conn.execute('PRAGMA journal_mode = OFF;')
        conn.execute('PRAGMA synchronous = OFF;')
        conn.execute('PRAGMA locking_mode = EXCLUSIVE;')

        placeholders = ", ".join(["?"] * 20)
        conn.executemany(
            f"INSERT INTO raw(strategy, total_payed, first_cnt, first_prize, first_adjacent_cnt, first_adjacent_prize, first_different_group_cnt, first_different_group_prize, second_cnt, second_prize, third_cnt, third_prize, fourth_cnt, fourth_prize, fifth_cnt, fifth_prize, sixth_cnt, sixth_prize, seventh_cnt, seventh_prize) VALUES ({placeholders})"
            , generate_typed_iter(Path('../outpt.tsv')))


if __name__ == '__main__':
    sample()
