import csv
import sqlite3
from typing import TextIO
from sqlite3 import Connection
from itertools import islice


from collections.abc import Iterable, Iterator
from contextlib import closing
from pathlib import Path


def create_table(conn:Connection)->None:
    print("Creating table")
    conn.execute("""
PRAGMA FOREIGN_KEYS =OFF;
PRAGMA JOURNAL_MODE =OFF;
PRAGMA SYNCHRONOUS =OFF;


DROP TABLE IF EXISTS raw;
DROP TABLE IF EXISTS prize_kind;
DROP TABLE IF EXISTS strategy_kind;
DROP TABLE IF EXISTS prizes;
DROP TABLE IF EXISTS person;


CREATE TABLE raw
(
    id                          INTEGER PRIMARY KEY AUTOINCREMENT ,
    unit                        INTEGER NOT NULL DEFAULT 0,
    strategy                    TEXT NOT NULL,
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

CREATE TABLE strategy_kind(
    id INTEGER PRIMARY KEY ,
    name TEXT NOT NULL
);

INSERT INTO strategy_kind(id,name)VALUES (1,'sequence_10'),(2,'random_10');

CREATE TABLE prize_kind(
    id INTEGER PRIMARY KEY ,
    name TEXT NOT NULL
);

INSERT INTO prize_kind(id,name) VALUES
(1,'first'),
(2,'first_adjacent'),
(3,'first_different_group'),
(4,'second'),
(5,'third'),
(6,'fourth'),
(7,'fifth'),
(8,'sixth'),
(9,'seventh');

CREATE TABLE person(
    id INTEGER PRIMARY KEY ,
    unit INTEGER NOT NULL ,
    total_payed INTEGER NOT NULL
);

CREATE TABLE prizes(
    id INTEGER PRIMARY KEY ,
    person_id INTEGER NOT NULL REFERENCES person(id),
    kind INTEGER NOT NULL REFERENCES prize_kind(id),
    cnt INTEGER NOT NULL ,
    reward INTEGER NOT NULL
);

VACUUM ;
    """)
    print("table created")


def generate_typed_iter(path:Path):
    with open(path,mode='r',encoding='utf-8') as file:
        reader = csv.reader(
            file,
            delimiter="\t",
        )

        next(reader)

        for rec in reader:
           yield int(rec[1]=='sequence_10'), *(int(v) for v in rec[2:])


def sample()->None:
    for rec in generate_typed_iter(Path('../outpt.tsv')):
        print(rec)




if __name__ == '__main__':
    sample()